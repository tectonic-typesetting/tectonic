use crate::sys::CGFloat;
use crate::{sys, CoreType};
use std::slice;

cfty! {
    /// A homogeneous array of CFType values, similar to [`Vec`].
    CGColor : CGColorGetTypeID
}

impl CGColor {
    /// Get the components of this color. This will be the number of color components in the color
    /// space plus one alpha channel.
    pub fn components(&self) -> &[CGFloat] {
        let num = unsafe { sys::CGColorGetNumberOfComponents(self.as_type_ref()) };
        let arr = unsafe { sys::CGColorGetComponents(self.as_type_ref()) };
        unsafe { slice::from_raw_parts(arr, num) }
    }
}
