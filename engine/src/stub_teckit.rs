/*------------------------------------------------------------------------
Copyright (C) 2002-2014 SIL International. All rights reserved.

Distributable under the terms of either the Common Public License or the
GNU Lesser General Public License, as specified in the LICENSING.txt file.

File: TECkit_Engine.h
Responsibility: Jonathan Kew
Last reviewed: Not yet.

Description:
    Public API to the TECkit conversion engine.
-------------------------------------------------------------------------*/

extern "C" {
    pub type Opaque_TECkit_Converter;

    #[no_mangle]
    pub fn TECkit_CreateConverter(
        mapping: *mut u8,
        mappingSize: u32,
        mapForward: u8,
        sourceForm: u16,
        targetForm: u16,
        converter: *mut TECkit_Converter,
    ) -> TECkit_Status;
    #[no_mangle]
    pub fn TECkit_ConvertBuffer(
        converter: TECkit_Converter,
        inBuffer: *const u8,
        inLength: u32,
        inUsed: *mut u32,
        outBuffer: *mut u8,
        outLength: u32,
        outUsed: *mut u32,
        inputIsComplete: u8,
    ) -> TECkit_Status;
}

pub type TECkit_Status = i64;

pub type TECkit_Converter = *mut Opaque_TECkit_Converter;
