use crate::c_api::{xcalloc_zeroed, ASCIICode, BufPointer, BufType};
use std::cell::RefCell;
use std::mem;

const BUF_SIZE: usize = 20000;

thread_local! {
    static GLOBAL_BUFFERS: RefCell<GlobalBuffer> = RefCell::new(GlobalBuffer::new());
}

pub(crate) fn reset() {
    GLOBAL_BUFFERS.with(|cell| *cell.borrow_mut() = GlobalBuffer::new());
}

pub fn with_buffers<T>(f: impl FnOnce(&GlobalBuffer) -> T) -> T {
    GLOBAL_BUFFERS.with(|buffers| f(&buffers.borrow()))
}

pub fn with_buffers_mut<T>(f: impl FnOnce(&mut GlobalBuffer) -> T) -> T {
    GLOBAL_BUFFERS.with(|buffers| f(&mut buffers.borrow_mut()))
}

struct Buffer<T: 'static, const N: usize> {
    ptr: &'static mut [T],
    /// Stateful offsets into the buffer
    offset: [BufPointer; N],
    /// Initialized length of the buffer
    init: BufPointer,
}

impl<T: Copy + 'static, const N: usize> Buffer<T, N> {
    fn new(len: usize) -> Buffer<T, N> {
        Buffer {
            ptr: unsafe { xcalloc_zeroed(len, mem::size_of::<T>()) },
            offset: [0; N],
            init: 0,
        }
    }

    fn grow(&mut self, new_len: usize) {
        // TODO: xrealloc_zeroed
        let new_ptr = unsafe { xcalloc_zeroed(new_len, mem::size_of::<T>()) };
        new_ptr.copy_from_slice(self.ptr);
        unsafe { libc::free((self.ptr as *mut [_]).cast()) };
        self.ptr = new_ptr;
    }
}

impl<T: 'static, const N: usize> Drop for Buffer<T, N> {
    fn drop(&mut self) {
        unsafe { libc::free((self.ptr as *mut [T]).cast()) };
    }
}

pub struct GlobalBuffer {
    /// Allocated length of all buffers
    buf_len: usize,
    buffer: Buffer<ASCIICode, 2>,
    sv_buffer: Buffer<ASCIICode, 2>,
    ex_buf: Buffer<ASCIICode, 1>,
    out_buf: Buffer<ASCIICode, 1>,
    name_tok: Buffer<ASCIICode, 2>,
}

impl GlobalBuffer {
    fn new() -> GlobalBuffer {
        let buf_len = BUF_SIZE + 1;
        GlobalBuffer {
            buf_len,
            buffer: Buffer::new(buf_len),
            sv_buffer: Buffer::new(buf_len),
            ex_buf: Buffer::new(buf_len),
            out_buf: Buffer::new(buf_len),
            name_tok: Buffer::new(buf_len),
        }
    }

    pub fn len(&self) -> usize {
        self.buf_len
    }

    pub fn buffer(&self, ty: BufTy) -> &[ASCIICode] {
        match ty {
            BufTy::Base => self.buffer.ptr,
            BufTy::Sv => self.sv_buffer.ptr,
            BufTy::Ex => self.ex_buf.ptr,
            BufTy::Out => self.out_buf.ptr,
            BufTy::NameTok => self.name_tok.ptr,
        }
    }

    pub fn buffer_mut(&mut self, ty: BufTy) -> &mut [ASCIICode] {
        match ty {
            BufTy::Base => self.buffer.ptr,
            BufTy::Sv => self.sv_buffer.ptr,
            BufTy::Ex => self.ex_buf.ptr,
            BufTy::Out => self.out_buf.ptr,
            BufTy::NameTok => self.name_tok.ptr,
        }
    }

    pub fn at(&self, ty: BufTy, offset: usize) -> ASCIICode {
        self.buffer(ty)[offset]
    }

    pub fn at_offset(&self, ty: BufTy, offset: usize) -> ASCIICode {
        self.buffer(ty)[self.offset(ty, offset) as usize]
    }

    pub fn set_offset(&mut self, ty: BufTy, offset: usize, val: BufPointer) {
        match ty {
            BufTy::Base => self.buffer.offset[offset - 1] = val,
            BufTy::Sv => self.sv_buffer.offset[offset - 1] = val,
            BufTy::Ex => self.ex_buf.offset[offset - 1] = val,
            BufTy::Out => self.out_buf.offset[offset - 1] = val,
            BufTy::NameTok => self.name_tok.offset[offset - 1] = val,
        }
    }

    pub fn offset(&self, ty: BufTy, offset: usize) -> BufPointer {
        match ty {
            BufTy::Base => self.buffer.offset[offset - 1],
            BufTy::Sv => self.sv_buffer.offset[offset - 1],
            BufTy::Ex => self.ex_buf.offset[offset - 1],
            BufTy::Out => self.out_buf.offset[offset - 1],
            BufTy::NameTok => self.name_tok.offset[offset - 1],
        }
    }

    pub fn init(&self, ty: BufTy) -> BufPointer {
        match ty {
            BufTy::Base => self.buffer.init,
            BufTy::Sv => self.sv_buffer.init,
            BufTy::Ex => self.ex_buf.init,
            BufTy::Out => self.out_buf.init,
            BufTy::NameTok => self.name_tok.init,
        }
    }

    pub fn set_init(&mut self, ty: BufTy, val: BufPointer) {
        match ty {
            BufTy::Base => self.buffer.init = val,
            BufTy::Sv => self.sv_buffer.init = val,
            BufTy::Ex => self.ex_buf.init = val,
            BufTy::Out => self.out_buf.init = val,
            BufTy::NameTok => self.name_tok.init = val,
        }
    }

    pub fn grow_all(&mut self) {
        let new_len = self.buf_len + BUF_SIZE;
        self.buffer.grow(new_len);
        self.sv_buffer.grow(new_len);
        self.ex_buf.grow(new_len);
        self.out_buf.grow(new_len);
        self.name_tok.grow(new_len);
        // self.name_sep_char.grow(new_len);
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
    Out,
    NameTok,
}

#[no_mangle]
pub extern "C" fn bib_buf_size() -> i32 {
    with_buffers(|buffers| buffers.len() as i32)
}

#[no_mangle]
pub extern "C" fn bib_buf(ty: BufTy) -> BufType {
    with_buffers_mut(|b| (b.buffer_mut(ty) as *mut [u8]).cast())
}

#[no_mangle]
pub unsafe extern "C" fn bib_buf_at(ty: BufTy, num: BufPointer) -> ASCIICode {
    with_buffers(|b| b.at(ty, num as usize))
}

#[no_mangle]
pub unsafe extern "C" fn bib_buf_at_offset(ty: BufTy, num: usize) -> ASCIICode {
    with_buffers(|b| b.at_offset(ty, num))
}

#[no_mangle]
pub extern "C" fn bib_buf_offset(ty: BufTy, num: usize) -> BufPointer {
    with_buffers(|buffers| buffers.offset(ty, num))
}

#[no_mangle]
pub extern "C" fn bib_set_buf_offset(ty: BufTy, num: usize, offset: BufPointer) {
    with_buffers_mut(|buffers| buffers.set_offset(ty, num, offset))
}

#[no_mangle]
pub extern "C" fn bib_buf_len(ty: BufTy) -> BufPointer {
    with_buffers(|buffers| buffers.init(ty))
}

#[no_mangle]
pub extern "C" fn bib_set_buf_len(ty: BufTy, len: BufPointer) {
    with_buffers_mut(|buffers| buffers.set_init(ty, len))
}

#[no_mangle]
pub extern "C" fn buffer_overflow() {
    with_buffers_mut(|buffers| buffers.grow_all())
}