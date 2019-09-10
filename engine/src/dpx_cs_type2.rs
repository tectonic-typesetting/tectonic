#![allow(dead_code,
         mutable_transmutes,
         non_camel_case_types,
         non_snake_case,
         non_upper_case_globals,
         unused_assignments,
         unused_mut)]

extern crate libc;
extern "C" {
    #[no_mangle]
    fn sqrt(_: f64) -> f64;
    #[no_mangle]
    fn fabs(_: f64) -> f64;
    #[no_mangle]
    fn floor(_: f64) -> f64;
    #[no_mangle]
    fn memmove(_: *mut libc::c_void, _: *const libc::c_void, _: u64)
        -> *mut libc::c_void;
    #[no_mangle]
    fn _tt_abort(format: *const i8, _: ...) -> !;
    #[no_mangle]
    fn dpx_warning(fmt: *const i8, _: ...);
}
pub type card8 = u8;
pub type card16 = u16;
pub type c_offsize = u8;
pub type l_offset = u32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cff_index {
    pub count: card16,
    pub offsize: c_offsize,
    pub offset: *mut l_offset,
    pub data: *mut card8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cs_ginfo {
    pub flags: libc::c_int,
    pub wx: f64,
    pub wy: f64,
    pub bbox: C2RustUnnamed_0,
    pub seac: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub asb: f64,
    pub adx: f64,
    pub ady: f64,
    pub bchar: card8,
    pub achar: card8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub llx: f64,
    pub lly: f64,
    pub urx: f64,
    pub ury: f64,
}
static mut status: libc::c_int = -1i32;
/* hintmask and cntrmask need number of stem zones */
static mut num_stems: libc::c_int = 0i32;
static mut phase: libc::c_int = 0i32;
/* subroutine nesting */
static mut nest: libc::c_int = 0i32;
/* advance width */
static mut have_width: libc::c_int = 0i32;
static mut width: f64 = 0.0f64;
/* Operand stack and Transient array */
static mut stack_top: libc::c_int = 0i32;
static mut arg_stack: [f64; 48] = [0.; 48];
static mut trn_array: [f64; 32] = [0.; 32];
/*
 * clear_stack() put all operands sotred in operand stack to dest.
 */
unsafe extern "C" fn clear_stack(mut dest: *mut *mut card8, mut limit: *mut card8) {
    let mut i: libc::c_int = 0;
    i = 0i32;
    while i < stack_top {
        let mut value: f64 = 0.;
        let mut ivalue: libc::c_int = 0;
        value = arg_stack[i as usize];
        /* Nearest integer value */
        ivalue = floor(value + 0.5f64) as libc::c_int;
        if value >= 0x8000i64 as f64
            || value <= (-0x8000 - 1 as i64) as f64
        {
            /*
             * This number cannot be represented as a single operand.
             * We must use `a b mul ...' or `a c div' to represent large values.
             */
            _tt_abort(
                b"%s: Argument value too large. (This is bug)\x00" as *const u8
                    as *const i8,
                b"Type2 Charstring Parser\x00" as *const u8 as *const i8,
            );
        } else {
            if fabs(value - ivalue as f64) > 3.0e-5f64 {
                /* 16.16-bit signed fixed value  */
                if limit < (*dest).offset(5) {
                    status = -3i32;
                    return;
                }
                let fresh0 = *dest;
                *dest = (*dest).offset(1);
                *fresh0 = 255i32 as card8;
                /* Everything else are integers. */
                ivalue = floor(value) as libc::c_int; /* mantissa */
                let fresh1 = *dest; /* fraction */
                *dest = (*dest).offset(1); /* Shouldn't come here */
                *fresh1 = (ivalue >> 8i32 & 0xffi32) as card8;
                let fresh2 = *dest;
                *dest = (*dest).offset(1);
                *fresh2 = (ivalue & 0xffi32) as card8;
                ivalue = ((value - ivalue as f64) * 0x10000i64 as f64)
                    as libc::c_int;
                let fresh3 = *dest;
                *dest = (*dest).offset(1);
                *fresh3 = (ivalue >> 8i32 & 0xffi32) as card8;
                let fresh4 = *dest;
                *dest = (*dest).offset(1);
                *fresh4 = (ivalue & 0xffi32) as card8
            } else if ivalue >= -107i32 && ivalue <= 107i32 {
                if limit < (*dest).offset(1) {
                    status = -3i32;
                    return;
                }
                let fresh5 = *dest;
                *dest = (*dest).offset(1);
                *fresh5 = (ivalue + 139i32) as card8
            } else if ivalue >= 108i32 && ivalue <= 1131i32 {
                if limit < (*dest).offset(2) {
                    status = -3i32;
                    return;
                }
                ivalue = 0xf700u32
                    .wrapping_add(ivalue as libc::c_uint)
                    .wrapping_sub(108i32 as libc::c_uint) as libc::c_int;
                let fresh6 = *dest;
                *dest = (*dest).offset(1);
                *fresh6 = (ivalue >> 8i32 & 0xffi32) as card8;
                let fresh7 = *dest;
                *dest = (*dest).offset(1);
                *fresh7 = (ivalue & 0xffi32) as card8
            } else if ivalue >= -1131i32 && ivalue <= -108i32 {
                if limit < (*dest).offset(2) {
                    status = -3i32;
                    return;
                }
                ivalue = 0xfb00u32
                    .wrapping_sub(ivalue as libc::c_uint)
                    .wrapping_sub(108i32 as libc::c_uint) as libc::c_int;
                let fresh8 = *dest;
                *dest = (*dest).offset(1);
                *fresh8 = (ivalue >> 8i32 & 0xffi32) as card8;
                let fresh9 = *dest;
                *dest = (*dest).offset(1);
                *fresh9 = (ivalue & 0xffi32) as card8
            } else if ivalue >= -32768i32 && ivalue <= 32767i32 {
                /* shortint */
                if limit < (*dest).offset(3) {
                    status = -3i32; /* clear stack */
                    return;
                }
                let fresh10 = *dest;
                *dest = (*dest).offset(1);
                *fresh10 = 28i32 as card8;
                let fresh11 = *dest;
                *dest = (*dest).offset(1);
                *fresh11 = (ivalue >> 8i32 & 0xffi32) as card8;
                let fresh12 = *dest;
                *dest = (*dest).offset(1);
                *fresh12 = (ivalue & 0xffi32) as card8
            } else {
                _tt_abort(
                    b"%s: Unexpected error.\x00" as *const u8 as *const i8,
                    b"Type2 Charstring Parser\x00" as *const u8 as *const i8,
                );
            }
        }
        i += 1
    }
    stack_top = 0i32;
}
/*
 * Single byte operators:
 *  Path construction, Operator for finishing a path, Hint operators.
 *
 * phase:
 *  0: inital state
 *  1: hint declaration, first stack-clearing operator appeared
 *  2: in path construction
 */
unsafe extern "C" fn do_operator1(
    mut dest: *mut *mut card8,
    mut limit: *mut card8,
    mut data: *mut *mut card8,
    mut endptr: *mut card8,
) {
    let mut op: card8 = **data;
    *data = (*data).offset(1);
    match op as libc::c_int {
        18 | 23 | 1 | 3 => {
            /* charstring may have hintmask if above operator have seen */
            if phase == 0i32 && stack_top % 2i32 != 0 {
                have_width = 1i32;
                width = arg_stack[0]
            }
            num_stems += stack_top / 2i32;
            clear_stack(dest, limit);
            if limit < (*dest).offset(1) {
                status = -3i32;
                return;
            }
            let fresh13 = *dest;
            *dest = (*dest).offset(1);
            *fresh13 = op;
            phase = 1i32
        }
        19 | 20 => {
            if phase < 2i32 {
                if phase == 0i32 && stack_top % 2i32 != 0 {
                    have_width = 1i32;
                    width = arg_stack[0]
                }
                num_stems += stack_top / 2i32
            }
            clear_stack(dest, limit);
            if limit < (*dest).offset(1) {
                status = -3i32;
                return;
            }
            let fresh14 = *dest;
            *dest = (*dest).offset(1);
            *fresh14 = op;
            if num_stems > 0i32 {
                let mut masklen: libc::c_int = (num_stems + 7i32) / 8i32;
                if limit < (*dest).offset(masklen as isize) {
                    status = -3i32;
                    return;
                }
                if endptr < (*data).offset(masklen as isize) {
                    status = -1i32;
                    return;
                }
                memmove(
                    *dest as *mut libc::c_void,
                    *data as *const libc::c_void,
                    masklen as u64,
                );
                *data = (*data).offset(masklen as isize);
                *dest = (*dest).offset(masklen as isize)
            }
            phase = 2i32
        }
        21 => {
            if phase == 0i32 && stack_top % 2i32 != 0 {
                have_width = 1i32;
                width = arg_stack[0]
            }
            clear_stack(dest, limit);
            if limit < (*dest).offset(1) {
                status = -3i32;
                return;
            }
            let fresh15 = *dest;
            *dest = (*dest).offset(1);
            *fresh15 = op;
            phase = 2i32
        }
        22 | 4 => {
            if phase == 0i32 && stack_top % 2i32 == 0i32 {
                have_width = 1i32;
                width = arg_stack[0]
            }
            clear_stack(dest, limit);
            if limit < (*dest).offset(1) {
                status = -3i32;
                return;
            }
            let fresh16 = *dest;
            *dest = (*dest).offset(1);
            *fresh16 = op;
            phase = 2i32
        }
        14 => {
            if stack_top == 1i32 {
                have_width = 1i32;
                width = arg_stack[0];
                clear_stack(dest, limit);
            } else if stack_top == 4i32 || stack_top == 5i32 {
                dpx_warning(
                    b"\"seac\" character deprecated in Type 2 charstring.\x00" as *const u8
                        as *const i8,
                );
                status = -1i32;
                return;
            } else {
                if stack_top > 0i32 {
                    dpx_warning(
                        b"%s: Operand stack not empty.\x00" as *const u8 as *const i8,
                        b"Type2 Charstring Parser\x00" as *const u8 as *const i8,
                    );
                }
            }
            if limit < (*dest).offset(1) {
                status = -3i32;
                return;
            }
            let fresh17 = *dest;
            *dest = (*dest).offset(1);
            *fresh17 = op;
            status = 3i32
        }
        5 | 6 | 7 | 8 | 24 | 25 | 26 | 27 | 30 | 31 => {
            /* above oprators are candidate for first stack-clearing operator */
            if phase < 2i32 {
                dpx_warning(
                    b"%s: Broken Type 2 charstring.\x00" as *const u8 as *const i8,
                    b"Type2 Charstring Parser\x00" as *const u8 as *const i8,
                );
                status = -1i32;
                return;
            }
            clear_stack(dest, limit);
            if limit < (*dest).offset(1) {
                status = -3i32;
                return;
            }
            let fresh18 = *dest;
            *dest = (*dest).offset(1);
            *fresh18 = op
        }
        11 | 29 | 10 => {
            /* all operotors above are stack-clearing operator */
            /* no output */
            _tt_abort(
                b"%s: Unexpected call(g)subr/return\x00" as *const u8 as *const i8,
                b"Type2 Charstring Parser\x00" as *const u8 as *const i8,
            );
        }
        _ => {
            /* no-op ? */
            dpx_warning(
                b"%s: Unknown charstring operator: 0x%02x\x00" as *const u8 as *const i8,
                b"Type2 Charstring Parser\x00" as *const u8 as *const i8,
                op as libc::c_int,
            );
            status = -1i32
        }
    };
}
/*
 * Double byte operators:
 *  Flex, arithmetic, conditional, and storage operators.
 *
 * Following operators are not supported:
 *  random: How random ?
 */
unsafe extern "C" fn do_operator2(
    mut dest: *mut *mut card8,
    mut limit: *mut card8,
    mut data: *mut *mut card8,
    mut endptr: *mut card8,
) {
    let mut op: card8 = 0;
    *data = (*data).offset(1);
    if endptr < (*data).offset(1) {
        status = -1i32;
        return;
    }
    op = **data;
    *data = (*data).offset(1);
    match op as libc::c_int {
        0 => {
            /* deprecated */
            dpx_warning(
                b"Operator \"dotsection\" deprecated in Type 2 charstring.\x00" as *const u8
                    as *const i8,
            );
            status = -1i32;
            return;
        }
        34 | 35 | 36 | 37 => {
            if phase < 2i32 {
                dpx_warning(
                    b"%s: Broken Type 2 charstring.\x00" as *const u8 as *const i8,
                    b"Type2 Charstring Parser\x00" as *const u8 as *const i8,
                );
                status = -1i32;
                return;
            }
            clear_stack(dest, limit);
            if limit < (*dest).offset(2) {
                status = -3i32;
                return;
            }
            let fresh19 = *dest;
            *dest = (*dest).offset(1);
            *fresh19 = 12i32 as card8;
            let fresh20 = *dest;
            *dest = (*dest).offset(1);
            *fresh20 = op
        }
        3 => {
            /* all operator above are stack-clearing */
            /* no output */
            if stack_top < 2i32 {
                status = -2i32;
                return;
            }
            stack_top -= 1;
            if arg_stack[stack_top as usize] != 0. && arg_stack[(stack_top - 1i32) as usize] != 0. {
                arg_stack[(stack_top - 1i32) as usize] = 1.0f64
            } else {
                arg_stack[(stack_top - 1i32) as usize] = 0.0f64
            }
        }
        4 => {
            if stack_top < 2i32 {
                status = -2i32;
                return;
            }
            stack_top -= 1;
            if arg_stack[stack_top as usize] != 0. || arg_stack[(stack_top - 1i32) as usize] != 0. {
                arg_stack[(stack_top - 1i32) as usize] = 1.0f64
            } else {
                arg_stack[(stack_top - 1i32) as usize] = 0.0f64
            }
        }
        5 => {
            if stack_top < 1i32 {
                status = -2i32;
                return;
            }
            if arg_stack[(stack_top - 1i32) as usize] != 0. {
                arg_stack[(stack_top - 1i32) as usize] = 0.0f64
            } else {
                arg_stack[(stack_top - 1i32) as usize] = 1.0f64
            }
        }
        9 => {
            if stack_top < 1i32 {
                status = -2i32;
                return;
            }
            arg_stack[(stack_top - 1i32) as usize] = fabs(arg_stack[(stack_top - 1i32) as usize])
        }
        10 => {
            if stack_top < 2i32 {
                status = -2i32;
                return;
            }
            arg_stack[(stack_top - 2i32) as usize] += arg_stack[(stack_top - 1i32) as usize];
            stack_top -= 1
        }
        11 => {
            if stack_top < 2i32 {
                status = -2i32;
                return;
            }
            arg_stack[(stack_top - 2i32) as usize] -= arg_stack[(stack_top - 1i32) as usize];
            stack_top -= 1
        }
        12 => {
            /* doesn't check overflow */
            if stack_top < 2i32 {
                status = -2i32;
                return;
            }
            arg_stack[(stack_top - 2i32) as usize] /= arg_stack[(stack_top - 1i32) as usize];
            stack_top -= 1
        }
        14 => {
            if stack_top < 1i32 {
                status = -2i32;
                return;
            }
            arg_stack[(stack_top - 1i32) as usize] *= -1.0f64
        }
        15 => {
            if stack_top < 2i32 {
                status = -2i32;
                return;
            }
            stack_top -= 1;
            if arg_stack[stack_top as usize] == arg_stack[(stack_top - 1i32) as usize] {
                arg_stack[(stack_top - 1i32) as usize] = 1.0f64
            } else {
                arg_stack[(stack_top - 1i32) as usize] = 0.0f64
            }
        }
        18 => {
            if stack_top < 1i32 {
                status = -2i32;
                return;
            }
            stack_top -= 1
        }
        20 => {
            if stack_top < 2i32 {
                status = -2i32;
                return;
            }
            stack_top -= 1;
            let mut idx: libc::c_int = arg_stack[stack_top as usize] as libc::c_int;
            if 32i32 < idx {
                status = -2i32;
                return;
            }
            stack_top -= 1;
            trn_array[idx as usize] = arg_stack[stack_top as usize]
        }
        21 => {
            if stack_top < 1i32 {
                status = -2i32;
                return;
            }
            let mut idx_0: libc::c_int = arg_stack[(stack_top - 1i32) as usize] as libc::c_int;
            if 32i32 < idx_0 {
                status = -2i32;
                return;
            }
            arg_stack[(stack_top - 1i32) as usize] = trn_array[idx_0 as usize]
        }
        22 => {
            if stack_top < 4i32 {
                status = -2i32;
                return;
            }
            stack_top -= 3i32;
            if arg_stack[(stack_top + 1i32) as usize] > arg_stack[(stack_top + 2i32) as usize] {
                arg_stack[(stack_top - 1i32) as usize] = arg_stack[stack_top as usize]
            }
        }
        24 => {
            if stack_top < 2i32 {
                status = -2i32;
                return;
            }
            arg_stack[(stack_top - 2i32) as usize] =
                arg_stack[(stack_top - 2i32) as usize] * arg_stack[(stack_top - 1i32) as usize];
            stack_top -= 1
        }
        26 => {
            if stack_top < 1i32 {
                status = -2i32;
                return;
            }
            arg_stack[(stack_top - 1i32) as usize] = sqrt(arg_stack[(stack_top - 1i32) as usize])
        }
        27 => {
            if stack_top < 1i32 {
                status = -2i32;
                return;
            }
            if 48i32 < stack_top + 1i32 {
                status = -2i32;
                return;
            }
            arg_stack[stack_top as usize] = arg_stack[(stack_top - 1i32) as usize];
            stack_top += 1
        }
        28 => {
            if stack_top < 2i32 {
                status = -2i32;
                return;
            }
            let mut save: f64 = arg_stack[(stack_top - 2i32) as usize];
            arg_stack[(stack_top - 2i32) as usize] = arg_stack[(stack_top - 1i32) as usize];
            arg_stack[(stack_top - 1i32) as usize] = save
        }
        29 => {
            if stack_top < 2i32 {
                status = -2i32;
                return;
            }
            /* need two arguments at least */
            let mut idx_1: libc::c_int = arg_stack[(stack_top - 1i32) as usize] as libc::c_int;
            if idx_1 < 0i32 {
                arg_stack[(stack_top - 1i32) as usize] = arg_stack[(stack_top - 2i32) as usize]
            } else {
                if stack_top < idx_1 + 2i32 {
                    status = -2i32;
                    return;
                }
                arg_stack[(stack_top - 1i32) as usize] =
                    arg_stack[(stack_top - idx_1 - 2i32) as usize]
            }
        }
        30 => {
            if stack_top < 2i32 {
                status = -2i32;
                return;
            }
            let mut N: libc::c_int = 0;
            let mut J: libc::c_int = 0;
            stack_top -= 1;
            J = arg_stack[stack_top as usize] as libc::c_int;
            stack_top -= 1;
            N = arg_stack[stack_top as usize] as libc::c_int;
            if stack_top < N {
                status = -2i32;
                return;
            }
            if J > 0i32 {
                J = J % N;
                loop {
                    let fresh21 = J;
                    J = J - 1;
                    if !(fresh21 > 0i32) {
                        break;
                    }
                    let mut save_0: f64 = arg_stack[(stack_top - 1i32) as usize];
                    let mut i: libc::c_int = stack_top - 1i32;
                    while i > stack_top - N {
                        arg_stack[i as usize] = arg_stack[(i - 1i32) as usize];
                        i -= 1
                    }
                    arg_stack[i as usize] = save_0
                }
            } else {
                J = -J % N;
                loop {
                    let fresh22 = J;
                    J = J - 1;
                    if !(fresh22 > 0i32) {
                        break;
                    }
                    let mut save_1: f64 = arg_stack[(stack_top - N) as usize];
                    let mut i_0: libc::c_int = stack_top - N;
                    while i_0 < stack_top - 1i32 {
                        arg_stack[i_0 as usize] = arg_stack[(i_0 + 1i32) as usize];
                        i_0 += 1
                    }
                    arg_stack[i_0 as usize] = save_1
                }
            }
        }
        23 => {
            dpx_warning(
                b"%s: Charstring operator \"random\" found.\x00" as *const u8
                    as *const i8,
                b"Type2 Charstring Parser\x00" as *const u8 as *const i8,
            );
            if 48i32 < stack_top + 1i32 {
                status = -2i32;
                return;
            }
            let fresh23 = stack_top;
            stack_top = stack_top + 1;
            arg_stack[fresh23 as usize] = 1.0f64
        }
        _ => {
            /* no-op ? */
            dpx_warning(
                b"%s: Unknown charstring operator: 0x0c%02x\x00" as *const u8
                    as *const i8,
                b"Type2 Charstring Parser\x00" as *const u8 as *const i8,
                op as libc::c_int,
            );
            status = -1i32
        }
    };
}
/*
 * integer:
 *  exactly the same as the DICT encoding (except 29)
 */
unsafe extern "C" fn get_integer(mut data: *mut *mut card8, mut endptr: *mut card8) {
    let mut result: libc::c_int = 0i32;
    let mut b0: card8 = **data;
    let mut b1: card8 = 0;
    let mut b2: card8 = 0;
    *data = (*data).offset(1);
    if b0 as libc::c_int == 28i32 {
        /* shortint */
        if endptr < (*data).offset(2) {
            status = -1i32;
            return;
        }
        b1 = **data;
        b2 = *(*data).offset(1);
        result = b1 as libc::c_int * 256i32 + b2 as libc::c_int;
        if result > 0x7fffi32 {
            result = (result as i64 - 0x10000) as libc::c_int
        }
        *data = (*data).offset(2)
    } else if b0 as libc::c_int >= 32i32 && b0 as libc::c_int <= 246i32 {
        /* int (1) */
        result = b0 as libc::c_int - 139i32
    } else if b0 as libc::c_int >= 247i32 && b0 as libc::c_int <= 250i32 {
        /* int (2) */
        if endptr < (*data).offset(1) {
            status = -1i32;
            return;
        }
        b1 = **data;
        result = (b0 as libc::c_int - 247i32) * 256i32 + b1 as libc::c_int + 108i32;
        *data = (*data).offset(1)
    } else if b0 as libc::c_int >= 251i32 && b0 as libc::c_int <= 254i32 {
        if endptr < (*data).offset(1) {
            status = -1i32;
            return;
        }
        b1 = **data;
        result = -(b0 as libc::c_int - 251i32) * 256i32 - b1 as libc::c_int - 108i32;
        *data = (*data).offset(1)
    } else {
        status = -1i32;
        return;
    }
    if 48i32 < stack_top + 1i32 {
        status = -2i32;
        return;
    }
    let fresh24 = stack_top;
    stack_top = stack_top + 1;
    arg_stack[fresh24 as usize] = result as f64;
}
/*
 * Signed 16.16-bits fixed number for Type 2 charstring encoding
 */
unsafe extern "C" fn get_fixed(mut data: *mut *mut card8, mut endptr: *mut card8) {
    let mut ivalue: libc::c_int = 0;
    let mut rvalue: f64 = 0.;
    *data = (*data).offset(1);
    if endptr < (*data).offset(4) {
        status = -1i32;
        return;
    }
    ivalue = **data as libc::c_int * 0x100i32 + *(*data).offset(1) as libc::c_int;
    rvalue = (if ivalue as i64 > 0x7fff {
        ivalue as i64 - 0x10000
    } else {
        ivalue as i64
    }) as f64;
    ivalue = *(*data).offset(2) as libc::c_int * 0x100i32 + *(*data).offset(3) as libc::c_int;
    rvalue += ivalue as f64 / 0x10000i64 as f64;
    if 48i32 < stack_top + 1i32 {
        status = -2i32;
        return;
    }
    let fresh25 = stack_top;
    stack_top = stack_top + 1;
    arg_stack[fresh25 as usize] = rvalue;
    *data = (*data).offset(4);
}
/*
 * Subroutines:
 *  The bias for subroutine number is introduced in type 2 charstrings.
 *
 * subr:     set to a pointer to the subroutine charstring.
 * len:      set to the length of subroutine charstring.
 * subr_idx: CFF INDEX data that contains subroutines.
 * id:       biased subroutine number.
 */
unsafe extern "C" fn get_subr(
    mut subr: *mut *mut card8,
    mut len: *mut libc::c_int,
    mut subr_idx: *mut cff_index,
    mut id: libc::c_int,
) {
    let mut count: card16 = 0;
    if subr_idx.is_null() {
        _tt_abort(
            b"%s: Subroutine called but no subroutine found.\x00" as *const u8
                as *const i8,
            b"Type2 Charstring Parser\x00" as *const u8 as *const i8,
        );
    }
    count = (*subr_idx).count;
    /* Adding bias number */
    if (count as libc::c_int) < 1240i32 {
        id += 107i32
    } else if (count as libc::c_int) < 33900i32 {
        id += 1131i32
    } else {
        id += 32768i32
    }
    if id > count as libc::c_int {
        _tt_abort(
            b"%s: Invalid Subr index: %d (max=%u)\x00" as *const u8 as *const i8,
            b"Type2 Charstring Parser\x00" as *const u8 as *const i8,
            id,
            count as libc::c_int,
        );
    }
    *len = (*(*subr_idx).offset.offset((id + 1i32) as isize))
        .wrapping_sub(*(*subr_idx).offset.offset(id as isize)) as libc::c_int;
    *subr = (*subr_idx)
        .data
        .offset(*(*subr_idx).offset.offset(id as isize) as isize)
        .offset(-1);
}
/*
 * NOTE:
 *  The Type 2 interpretation of a number encoded in five-bytes (those with
 *  an initial byte value of 255) differs from how it is interpreted in the
 *  Type 1 format.
 */
unsafe extern "C" fn do_charstring(
    mut dest: *mut *mut card8,
    mut limit: *mut card8,
    mut data: *mut *mut card8,
    mut endptr: *mut card8,
    mut gsubr_idx: *mut cff_index,
    mut subr_idx: *mut cff_index,
) {
    let mut b0: card8 = 0i32 as card8;
    let mut subr: *mut card8 = 0 as *mut card8;
    let mut len: libc::c_int = 0;
    if nest > 10i32 {
        _tt_abort(
            b"%s: Subroutine nested too deeply.\x00" as *const u8 as *const i8,
            b"Type2 Charstring Parser\x00" as *const u8 as *const i8,
        );
    }
    nest += 1;
    while *data < endptr && status == 0i32 {
        b0 = **data;
        if b0 as libc::c_int == 255i32 {
            /* 16-bit.16-bit fixed signed number */
            get_fixed(data, endptr);
        } else if b0 as libc::c_int == 11i32 {
            status = 2i32
        } else if b0 as libc::c_int == 29i32 {
            if stack_top < 1i32 {
                status = -2i32
            } else {
                stack_top -= 1;
                get_subr(
                    &mut subr,
                    &mut len,
                    gsubr_idx,
                    arg_stack[stack_top as usize] as libc::c_int,
                );
                if (*dest).offset(len as isize) > limit {
                    _tt_abort(
                        b"%s: Possible buffer overflow.\x00" as *const u8 as *const i8,
                        b"Type2 Charstring Parser\x00" as *const u8 as *const i8,
                    );
                }
                do_charstring(
                    dest,
                    limit,
                    &mut subr,
                    subr.offset(len as isize),
                    gsubr_idx,
                    subr_idx,
                );
                *data = (*data).offset(1)
            }
        } else if b0 as libc::c_int == 10i32 {
            if stack_top < 1i32 {
                status = -2i32
            } else {
                stack_top -= 1;
                get_subr(
                    &mut subr,
                    &mut len,
                    subr_idx,
                    arg_stack[stack_top as usize] as libc::c_int,
                );
                if limit < (*dest).offset(len as isize) {
                    _tt_abort(
                        b"%s: Possible buffer overflow.\x00" as *const u8 as *const i8,
                        b"Type2 Charstring Parser\x00" as *const u8 as *const i8,
                    );
                }
                do_charstring(
                    dest,
                    limit,
                    &mut subr,
                    subr.offset(len as isize),
                    gsubr_idx,
                    subr_idx,
                );
                *data = (*data).offset(1)
            }
        } else if b0 as libc::c_int == 12i32 {
            do_operator2(dest, limit, data, endptr);
        } else if (b0 as libc::c_int) < 32i32 && b0 as libc::c_int != 28i32 {
            /* 19, 20 need mask */
            do_operator1(dest, limit, data, endptr);
        } else if b0 as libc::c_int >= 22i32 && b0 as libc::c_int <= 27i32
            || b0 as libc::c_int == 31i32
        {
            /* reserved */
            status = -1i32
        /* not an error ? */
        } else {
            get_integer(data, endptr);
        }
    }
    if status == 2i32 {
        status = 0i32
    } else if status == 3i32 && *data < endptr {
        dpx_warning(
            b"%s: Garbage after endchar.\x00" as *const u8 as *const i8,
            b"Type2 Charstring Parser\x00" as *const u8 as *const i8,
        );
    } else if status < 0i32 {
        /* error */
        _tt_abort(
            b"%s: Parsing charstring failed: (status=%d, stack=%d)\x00" as *const u8
                as *const i8,
            b"Type2 Charstring Parser\x00" as *const u8 as *const i8,
            status,
            stack_top,
        );
    }
    nest -= 1;
}
unsafe extern "C" fn cs_parse_init() {
    status = 0i32;
    nest = 0i32;
    phase = 0i32;
    num_stems = 0i32;
    stack_top = 0i32;
}
/* This is dvipdfmx, an eXtended version of dvipdfm by Mark A. Wicks.

    Copyright (C) 2002-2016 by Jin-Hwan Cho and Shunsaku Hirata,
    the dvipdfmx project team.

    This program is free software; you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation; either version 2 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program; if not, write to the Free Software
    Foundation, Inc., 59 Temple Place, Suite 330, Boston, MA 02111-1307 USA.
*/
/* unused in Type 2 charstring */
/* unused in Type 2 charstring */
/*
 * Not just copying...
 */
#[no_mangle]
pub unsafe extern "C" fn cs_copy_charstring(
    mut dst: *mut card8,
    mut dstlen: libc::c_int,
    mut src: *mut card8,
    mut srclen: libc::c_int,
    mut gsubr: *mut cff_index,
    mut subr: *mut cff_index,
    mut default_width: f64,
    mut nominal_width: f64,
    mut ginfo: *mut cs_ginfo,
) -> libc::c_int {
    let mut save: *mut card8 = dst;
    cs_parse_init();
    width = 0.0f64;
    have_width = 0i32;
    /* expand call(g)subrs */
    do_charstring(
        &mut dst,
        dst.offset(dstlen as isize),
        &mut src,
        src.offset(srclen as isize),
        gsubr,
        subr,
    ); /* not used */
    if !ginfo.is_null() {
        (*ginfo).flags = 0i32;
        if have_width != 0 {
            (*ginfo).wx = nominal_width + width
        } else {
            (*ginfo).wx = default_width
        }
    }
    return dst.wrapping_offset_from(save) as i64 as libc::c_int;
}
