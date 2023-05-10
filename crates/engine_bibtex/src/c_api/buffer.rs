use crate::c_api::{xcalloc, xrealloc, ASCIICode, BufPointer, BufType};
use std::cell::RefCell;
use std::{mem, ptr};

const BUF_SIZE: usize = 20000;

thread_local! {
    static GLOBAL_BUFFERS: RefCell<GlobalBuffer> = const { RefCell::new(GlobalBuffer::new()) };
}

pub fn with_buffers<T>(f: impl FnOnce(&GlobalBuffer) -> T) -> T {
    GLOBAL_BUFFERS.with(|buffers| f(&buffers.borrow()))
}

pub fn with_buffers_mut<T>(f: impl FnOnce(&mut GlobalBuffer) -> T) -> T {
    GLOBAL_BUFFERS.with(|buffers| f(&mut buffers.borrow_mut()))
}

struct Buffer<T, const N: usize> {
    ptr: *mut T,
    /// Stateful offsets into the buffer
    offset: [BufPointer; N],
    /// Initialized length of this buffer
    init: usize,
}

impl<T, const N: usize> Buffer<T, N> {
    const fn new() -> Buffer<T, N> {
        Buffer {
            ptr: ptr::null_mut(),
            offset: [0; N],
            init: 0,
        }
    }

    fn alloc(&mut self, len: usize) {
        self.ptr = unsafe { xcalloc(len, mem::size_of::<T>()) }.cast();
    }

    fn grow(&mut self, new_len: usize) {
        self.ptr = unsafe { xrealloc(self.ptr.cast(), new_len) }.cast();
    }
}

pub struct GlobalBuffer {
    /// Allocated length of all buffers
    buf_len: usize,
    buffer: Buffer<ASCIICode, 2>,
    sv_buffer: Buffer<ASCIICode, 2>,
    ex_buf: Buffer<ASCIICode, 1>,
    out_buf: Buffer<ASCIICode, 1>,
    name_tok: Buffer<BufPointer, 1>,
    name_sep_char: Buffer<BufType, 1>,
}

impl GlobalBuffer {
    const fn new() -> GlobalBuffer {
        GlobalBuffer {
            buf_len: 0,
            buffer: Buffer::new(),
            sv_buffer: Buffer::new(),
            ex_buf: Buffer::new(),
            out_buf: Buffer::new(),
            name_tok: Buffer::new(),
            name_sep_char: Buffer::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.buf_len
    }

    pub fn buffer(&self, ty: BufTy) -> *mut ASCIICode {
        match ty {
            BufTy::Base => self.buffer.ptr,
            BufTy::Sv => self.sv_buffer.ptr,
            BufTy::Ex => self.ex_buf.ptr,
        }
    }

    pub unsafe fn at(&self, ty: BufTy, offset: usize) -> ASCIICode {
        let ptr = self.buffer(ty);
        unsafe { *ptr.add(offset) }
    }

    fn init(&mut self) {
        self.buf_len = BUF_SIZE + 1;
        self.buffer.alloc(self.buf_len);
        self.sv_buffer.alloc(self.buf_len);
        self.ex_buf.alloc(self.buf_len);
        self.out_buf.alloc(self.buf_len);
        self.name_tok.alloc(self.buf_len);
        self.name_sep_char.alloc(self.buf_len);
    }

    pub fn grow_all(&mut self) {
        let new_len = self.buf_len + BUF_SIZE;
        self.buffer.grow(new_len);
        self.sv_buffer.grow(new_len);
        self.ex_buf.grow(new_len);
        self.out_buf.grow(new_len);
        self.name_tok.grow(new_len);
        self.name_sep_char.grow(new_len);
        self.buf_len = new_len;
    }
}

/// cbindgen:rename-all=ScreamingSnakeCase
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub enum BufTy {
    Base,
    Sv,
    Ex,
}

#[no_mangle]
pub extern "C" fn bib_buf_size() -> i32 {
    GLOBAL_BUFFERS.with(|buffers| buffers.borrow().buf_len as i32)
}

#[no_mangle]
pub extern "C" fn bib_buf(ty: BufTy) -> BufType {
    with_buffers(|b| b.buffer(ty))
}

#[no_mangle]
pub unsafe extern "C" fn bib_buf_at(ty: BufTy, num: BufPointer) -> ASCIICode {
    with_buffers(|b| b.at(ty, num as usize))
}

#[no_mangle]
pub unsafe extern "C" fn bib_buf_at_offset(ty: BufTy, num: usize) -> ASCIICode {
    let ptr = bib_buf(ty);
    let offset = bib_buf_offset(ty, num);
    unsafe { *ptr.add(offset as usize) }
}

#[no_mangle]
pub extern "C" fn bib_buf_offset(ty: BufTy, num: usize) -> BufPointer {
    GLOBAL_BUFFERS.with(|buffers| {
        let buffers = buffers.borrow();
        match ty {
            BufTy::Base => buffers.buffer.offset[num - 1],
            BufTy::Sv => buffers.sv_buffer.offset[num - 1],
            BufTy::Ex => buffers.ex_buf.offset[num - 1],
        }
    })
}

#[no_mangle]
pub extern "C" fn bib_set_buf_offset(ty: BufTy, num: usize, offset: BufPointer) {
    GLOBAL_BUFFERS.with(|buffers| {
        let mut buffers = buffers.borrow_mut();
        match ty {
            BufTy::Base => buffers.buffer.offset[num - 1] = offset,
            BufTy::Sv => buffers.sv_buffer.offset[num - 1] = offset,
            BufTy::Ex => buffers.ex_buf.offset[num - 1] = offset,
        }
    })
}

#[no_mangle]
pub extern "C" fn buffer_overflow() {
    GLOBAL_BUFFERS.with(|buffers| buffers.borrow_mut().grow_all())
}

#[no_mangle]
pub extern "C" fn bib_init_buffers() {
    GLOBAL_BUFFERS.with(|buffers| buffers.borrow_mut().init())
}
