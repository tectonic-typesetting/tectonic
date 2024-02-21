use crate::{ASCIICode, BufPointer};
use std::slice;

pub(crate) const BUF_SIZE: usize = 20000;

struct Buffer<T, const N: usize> {
    ptr: Vec<T>,
    /// Stateful offsets into the buffer
    offset: [BufPointer; N],
    /// Initialized length of the buffer
    init: BufPointer,
}

impl<T: Default + Clone, const N: usize> Buffer<T, N> {
    fn new(len: usize) -> Buffer<T, N> {
        Buffer {
            ptr: vec![T::default(); len],
            offset: [0; N],
            init: 0,
        }
    }

    fn grow(&mut self, new_len: usize) {
        self.ptr.resize(self.ptr.len() + new_len, T::default());
    }
}

pub(crate) struct GlobalBuffer {
    /// Allocated length of all buffers
    buf_len: usize,
    buffer: Buffer<ASCIICode, 2>,
    sv_buffer: Buffer<ASCIICode, 0>,
    ex_buf: Buffer<ASCIICode, 1>,
    out_buf: Buffer<ASCIICode, 0>,
    name_sep_char: Buffer<ASCIICode, 0>,
    name_tok: Vec<BufPointer>,
}

impl GlobalBuffer {
    pub(crate) fn new() -> GlobalBuffer {
        let buf_len = BUF_SIZE + 1;
        GlobalBuffer {
            buf_len,
            buffer: Buffer::new(buf_len),
            sv_buffer: Buffer::new(buf_len),
            ex_buf: Buffer::new(buf_len),
            out_buf: Buffer::new(buf_len),
            name_sep_char: Buffer::new(buf_len),
            name_tok: vec![0; buf_len],
        }
    }

    pub fn len(&self) -> usize {
        self.buf_len
    }

    fn buffer_raw(&mut self, ty: BufTy) -> *mut ASCIICode {
        match ty {
            BufTy::Base => self.buffer.ptr.as_mut_ptr(),
            BufTy::Sv => self.sv_buffer.ptr.as_mut_ptr(),
            BufTy::Ex => self.ex_buf.ptr.as_mut_ptr(),
            BufTy::Out => self.out_buf.ptr.as_mut_ptr(),
            BufTy::NameSep => self.name_sep_char.ptr.as_mut_ptr(),
        }
    }

    pub fn buffer(&self, ty: BufTy) -> &[ASCIICode] {
        match ty {
            BufTy::Base => &self.buffer.ptr,
            BufTy::Sv => &self.sv_buffer.ptr,
            BufTy::Ex => &self.ex_buf.ptr,
            BufTy::Out => &self.out_buf.ptr,
            BufTy::NameSep => &self.name_sep_char.ptr,
        }
    }

    pub fn buffer_mut(&mut self, ty: BufTy) -> &mut [ASCIICode] {
        match ty {
            BufTy::Base => &mut self.buffer.ptr,
            BufTy::Sv => &mut self.sv_buffer.ptr,
            BufTy::Ex => &mut self.ex_buf.ptr,
            BufTy::Out => &mut self.out_buf.ptr,
            BufTy::NameSep => &mut self.name_sep_char.ptr,
        }
    }

    fn copy_within_same(&mut self, ty: BufTy, from: usize, to: usize, len: usize) {
        let buf = self.buffer_mut(ty);
        buf.copy_within(from..from + len, to);
    }

    pub fn copy_within(
        &mut self,
        from: BufTy,
        to: BufTy,
        from_start: usize,
        to_start: usize,
        len: usize,
    ) {
        assert!(to_start + len < self.buf_len);
        assert!(from_start + len < self.buf_len);
        if from == to {
            self.copy_within_same(from, from_start, to_start, len);
        } else {
            // SAFETY: Pointer guaranteed valid for up to `len`
            let to = unsafe { slice::from_raw_parts_mut(self.buffer_raw(to).add(to_start), len) };
            let from = &self.buffer(from)[from_start..from_start + len];

            to.copy_from_slice(from);
        }
    }

    pub fn copy_from(&mut self, ty: BufTy, pos: usize, val: &[ASCIICode]) {
        self.buffer_mut(ty)[pos..pos + val.len()].copy_from_slice(val);
    }

    pub fn at(&self, ty: BufTy, offset: usize) -> ASCIICode {
        self.buffer(ty)[offset]
    }

    pub fn set_at(&mut self, ty: BufTy, offset: usize, val: ASCIICode) {
        self.buffer_mut(ty)[offset] = val;
    }

    pub fn at_offset(&self, ty: BufTy, offset: usize) -> ASCIICode {
        self.buffer(ty)[self.offset(ty, offset)]
    }

    pub fn set_offset(&mut self, ty: BufTy, offset: usize, val: BufPointer) {
        match ty {
            BufTy::Base => self.buffer.offset[offset - 1] = val,
            BufTy::Ex => self.ex_buf.offset[offset - 1] = val,
            BufTy::Sv | BufTy::Out | BufTy::NameSep => {
                unreachable!("Buffer {:?} has no offsets", ty)
            }
        }
    }

    pub fn name_tok(&self, pos: BufPointer) -> BufPointer {
        self.name_tok[pos]
    }

    pub fn set_name_tok(&mut self, pos: BufPointer, val: BufPointer) {
        self.name_tok[pos] = val;
    }

    // pub fn incr_offset(&mut self, ty: BufTy, offset: usize) {
    //     match ty {
    //         BufTy::Base => self.buffer.offset[offset - 1] += 1,
    //         BufTy::Ex => self.ex_buf.offset[offset - 1] += 1,
    //         BufTy::Sv | BufTy::Out | BufTy::NameSep => {
    //             unreachable!("Buffer {:?} has no offsets", ty)
    //         }
    //     }
    // }

    pub fn offset(&self, ty: BufTy, offset: usize) -> BufPointer {
        match ty {
            BufTy::Base => self.buffer.offset[offset - 1],
            BufTy::Ex => self.ex_buf.offset[offset - 1],
            BufTy::Sv | BufTy::Out | BufTy::NameSep => {
                unreachable!("Buffer {:?} has no offsets", ty)
            }
        }
    }

    pub fn init(&self, ty: BufTy) -> BufPointer {
        match ty {
            BufTy::Base => self.buffer.init,
            BufTy::Sv => self.sv_buffer.init,
            BufTy::Ex => self.ex_buf.init,
            BufTy::Out => self.out_buf.init,
            BufTy::NameSep => self.name_sep_char.init,
        }
    }

    pub fn set_init(&mut self, ty: BufTy, val: BufPointer) {
        match ty {
            BufTy::Base => self.buffer.init = val,
            BufTy::Sv => self.sv_buffer.init = val,
            BufTy::Ex => self.ex_buf.init = val,
            BufTy::Out => self.out_buf.init = val,
            BufTy::NameSep => self.name_sep_char.init = val,
        }
    }

    pub fn grow_all(&mut self) {
        let new_len = self.buf_len + BUF_SIZE;
        self.buffer.grow(BUF_SIZE);
        self.sv_buffer.grow(BUF_SIZE);
        self.ex_buf.grow(BUF_SIZE);
        self.out_buf.grow(BUF_SIZE);
        self.name_sep_char.grow(BUF_SIZE);
        self.name_tok.resize(self.name_tok.len() + BUF_SIZE, 0);
        self.buf_len = new_len;
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub(crate) enum BufTy {
    Base,
    Sv,
    Ex,
    Out,
    NameSep,
}
