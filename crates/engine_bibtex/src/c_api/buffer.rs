use crate::c_api::{xcalloc, xrealloc, ASCIICode, BufPointer, BufType};
use std::cell::UnsafeCell;
use std::{mem, ptr};

struct UnsafeSendSync<T>(UnsafeCell<T>);

unsafe impl<T> Sync for UnsafeSendSync<T> {}
unsafe impl<T> Send for UnsafeSendSync<T> {}

impl<T> UnsafeSendSync<T> {
    const fn new(val: T) -> UnsafeSendSync<T> {
        UnsafeSendSync(UnsafeCell::new(val))
    }

    fn get(&self) -> *mut T {
        self.0.get()
    }
}

const BUF_SIZE: usize = 20000;
static GLOBAL_BUFFERS: UnsafeSendSync<GlobalBuffer> = UnsafeSendSync::new(GlobalBuffer::new());

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

struct GlobalBuffer {
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

    fn init(&mut self) {
        self.buf_len = BUF_SIZE + 1;
        self.buffer.alloc(self.buf_len);
        self.sv_buffer.alloc(self.buf_len);
        self.ex_buf.alloc(self.buf_len);
        self.out_buf.alloc(self.buf_len);
        self.name_tok.alloc(self.buf_len);
        self.name_sep_char.alloc(self.buf_len);
    }

    fn grow_all(&mut self) {
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
    let buffers = unsafe { &*GLOBAL_BUFFERS.get() };
    buffers.buf_len as i32
}

#[no_mangle]
pub extern "C" fn bib_buf(ty: BufTy) -> BufType {
    let buffers = unsafe { &*GLOBAL_BUFFERS.get() };
    match ty {
        BufTy::Base => buffers.buffer.ptr,
        BufTy::Sv => buffers.sv_buffer.ptr,
        BufTy::Ex => buffers.ex_buf.ptr,
    }
}

#[no_mangle]
pub unsafe extern "C" fn bib_buf_at(ty: BufTy, num: BufPointer) -> ASCIICode {
    let ptr = bib_buf(ty);
    unsafe { *ptr.add(num as usize) }
}

#[no_mangle]
pub unsafe extern "C" fn bib_buf_at_offset(ty: BufTy, num: usize) -> ASCIICode {
    let ptr = bib_buf(ty);
    let offset = bib_buf_offset(ty, num);
    unsafe { *ptr.add(offset as usize) }
}

#[no_mangle]
pub extern "C" fn bib_buf_offset(ty: BufTy, num: usize) -> BufPointer {
    let buffers = unsafe { &*GLOBAL_BUFFERS.get() };
    match ty {
        BufTy::Base => buffers.buffer.offset[num - 1],
        BufTy::Sv => buffers.sv_buffer.offset[num - 1],
        BufTy::Ex => buffers.ex_buf.offset[num - 1],
    }
}

#[no_mangle]
pub extern "C" fn bib_set_buf_offset(ty: BufTy, num: usize, offset: BufPointer) {
    let buffers = unsafe { &mut *GLOBAL_BUFFERS.get() };
    match ty {
        BufTy::Base => buffers.buffer.offset[num - 1] = offset,
        BufTy::Sv => buffers.sv_buffer.offset[num - 1] = offset,
        BufTy::Ex => buffers.ex_buf.offset[num - 1] = offset,
    }
}

#[no_mangle]
pub unsafe extern "C" fn buffer_overflow() {
    let buffers = &mut *GLOBAL_BUFFERS.get();
    buffers.grow_all();
}

#[no_mangle]
pub unsafe extern "C" fn bib_init_buffers() {
    let buffers = &mut *GLOBAL_BUFFERS.get();
    buffers.init();
}
