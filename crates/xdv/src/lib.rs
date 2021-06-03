// Copyright 2018 the Tectonic Project
// Licensed under the MIT License.

#![deny(missing_docs)]

//! A decoder for the XDV and SPX file formats used by Tectonic and XeTeX.
//!
//! Both of these file formats are derived from the venerable “device
//! independent” (DVI) format used by TeX. The XDV format (name presumably
//! meaning something like “XeTeX DVI” or “extended DVI”) adds a few codes
//! needed to express native fonts in the output. The SPX format
//! (“semantically-paginated XDV”) is essentially the same as XDV, but
//! expresses output that is not paginated for print — this is what Tectonic
//! uses to produce its HTML output.

use byteorder::{BigEndian, ByteOrder};
use std::error;
use std::fmt::{Debug, Display, Error as FmtError, Formatter};
use std::io::{Error as IoError, Read};
use std::marker::PhantomData;
use std::mem;

/// Errors that can occur when parsing XDV/SPX files.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum XdvError {
    /// The file is malformed in some way. The error occurred at the byte
    /// offset indicated in the error value.
    Malformed(u64),

    /// An illegal opcode was encountered. The error occurred at the byte
    /// offset indicated in the error value.
    IllegalOpcode(u8, u64),

    /// Stream ended before expected.
    UnexpectedEndOfStream,
}

impl Display for XdvError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        match *self {
            XdvError::Malformed(offset) => {
                write!(f, "unexpected XDV data at byte offset {}", offset)
            }
            XdvError::IllegalOpcode(opcode, offset) => {
                write!(f, "illegal XDV opcode {} at byte offset {}", opcode, offset)
            }
            XdvError::UnexpectedEndOfStream => write!(f, "stream ended unexpectedly soon"),
        }
    }
}

impl error::Error for XdvError {
    fn description(&self) -> &str {
        match *self {
            XdvError::Malformed(_) => "malformed XDV data",
            XdvError::IllegalOpcode(_, _) => "illegal XDV opcode",
            XdvError::UnexpectedEndOfStream => "stream ended unexpectedly soon",
        }
    }

    fn cause(&self) -> Option<&dyn error::Error> {
        None
    }
}

/// In case you want to use String as your error type.
impl From<XdvError> for String {
    fn from(e: XdvError) -> Self {
        format!("{}", e)
    }
}

impl XdvError {
    /// We can't implement this as a From trait on InternalError because it
    /// conflicts with the generic From: XdvError satisfies `Debug +
    /// From<XdvError>`!
    fn into_internal<T: Debug + From<XdvError>>(self) -> InternalError<T> {
        InternalError::Other(self.into())
    }
}

/// This internal error type lets us leverage Results to conveniently
/// check for the need-more-data case, while not exporting that type
/// to the outside world. If NeedMoreData were part of XdvError, there
/// would also be a problem checking for NeedMoreData once that type
/// was converted into the caller's Error type.
#[derive(Debug)]
enum InternalError<T: Debug + From<XdvError>> {
    NeedMoreData,
    Other(T),
}

type InternalResult<T, E> = Result<T, InternalError<E>>;

impl<T: Debug + From<XdvError>> From<T> for InternalError<T> {
    fn from(e: T) -> Self {
        InternalError::Other(e)
    }
}

/// Types implementing this trait accept events from the XDV parser.
pub trait XdvEvents {
    /// An error type returned by the handler functions defined in this trait. It
    /// must implement `From<XdvError>`.
    type Error: Debug + From<XdvError>;

    /// Handle the XDV header
    #[allow(unused)] // <= prevents underscore-prefixed names from showing up in docs
    fn handle_header(&mut self, filetype: FileType, comment: &[u8]) -> Result<(), Self::Error> {
        Ok(())
    }

    /// Begin a new page.
    #[allow(unused)]
    fn handle_begin_page(
        &mut self,
        counters: &[i32],
        previous_bop: i32,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    /// Handle a `\special`.
    #[allow(unused)]
    fn handle_special(&mut self, contents: &[u8]) -> Result<(), Self::Error> {
        Ok(())
    }

    /// Handle a sequence of characters without intervening commands
    #[allow(unused)]
    fn handle_char_run(&mut self, chars: &[i32]) -> Result<(), Self::Error> {
        Ok(())
    }
}

/// State for parsing an XDV file.
#[derive(Debug)]
pub struct XdvParser<T: XdvEvents> {
    events: T,
    filetype: FileType,
    state: ParserState,
    stack: Vec<State>,
    cur_font_num: Option<i32>,
    offset: u64,
    cur_char_run: Vec<i32>,
}

/// Which type of file is being parsed.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FileType {
    /// Traditional XDV.
    Xdv,

    /// Tectonic’s SPX (“semanticallly-paginated XDV”).
    Spx,
}

impl Display for FileType {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        write!(
            f,
            "{}",
            match *self {
                FileType::Xdv => "XDV",
                FileType::Spx => "SPX",
            }
        )
    }
}

/// The current state of the parser.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ParserState {
    /// We have just started parsing.
    Preamble,

    /// Between pages.
    BetweenPages,

    /// In a page.
    InPage,

    /// Reading font definitions and nops in the postamble.
    PostambleFontDefinitions,

    /// We read through the post-postamble and are satisfied with this file.
    Finished,
}

impl<T: XdvEvents> XdvParser<T> {
    /// Create a new XDV/SPX parser.
    ///
    /// Methods implemented by *events* will be called as various items are
    /// encountered in the file.
    pub fn new(events: T) -> Self {
        XdvParser {
            events,
            filetype: FileType::Xdv,
            state: ParserState::Preamble,
            stack: Vec::new(),
            cur_font_num: None,
            offset: 0,
            cur_char_run: Vec::new(),
        }
    }

    /// Parse an entire XDV/SPX stream.
    ///
    /// Returns the input “events” variable and the number of bytes that were
    /// processed.
    ///
    /// Because the `io::Read` trait is used, the event result type must
    /// implement `From<io::Error>` as well as `From<XdvError>`.
    ///
    /// The initial buffer size is hardcoded to 4096 bytes. It doubles in size
    /// every time the parser is unable to make any progress at all in the
    /// current chunk, which should be a rare circumstance.
    pub fn process<R: Read>(mut stream: R, events: T) -> Result<(T, u64), T::Error>
    where
        T::Error: From<IoError>,
    {
        const BUF_SIZE: usize = 4096;
        let mut parser = Self::new(events);
        // Note that it is unsound to pass uninitialized data to a read() call,
        // even though it *should* never cause problems ...
        let mut buf = vec![0; BUF_SIZE];
        let mut n_saved_bytes = 0;

        loop {
            let n_read = stream.read(&mut buf[n_saved_bytes..])?;
            let n_in_buffer = n_saved_bytes + n_read;
            let n_consumed = parser.parse(&buf[..n_in_buffer])?;
            n_saved_bytes = n_in_buffer - n_consumed;

            if n_consumed != 0 && n_saved_bytes != 0 {
                // The current parse did not consume the full buffer, so we
                // must copy the un-parsed bytes to its beginning. The next
                // time that we read data, we will append to these
                // already-read bytes so that the parser gets a nice
                // contiguous set of bytes to look at. The copy may involve
                // overlapping memory regions (imagine we read 4096 bytes but
                // only consume 1) so we have to get unsafe.
                use std::ptr;
                let ptr = buf.as_mut_ptr();
                unsafe {
                    ptr::copy(ptr.add(n_consumed), ptr, n_saved_bytes);
                }
            }

            if n_in_buffer != 0 && n_consumed == 0 {
                // We're going to need a bigger buffer in order to handle whatever
                // we're reading. Let's double it.
                let len = buf.len();
                buf.reserve(len);
                unsafe {
                    buf.set_len(2 * len);
                }
            }

            if n_read == 0 {
                break;
            }
        }

        let n_bytes = parser.current_offset();
        Ok((parser.finish()?, n_bytes))
    }

    /// Parse the next chunk of XDV data.
    ///
    /// Returns the number of bytes consumed from the input buffer. If this is
    /// not the same as the buffer size, some of the existing bytes must be
    /// re-fed to the parser. If the returned value is 0, you need a bigger
    /// buffer in order to be able to parse the next directive.
    #[allow(clippy::cognitive_complexity)]
    pub fn parse(&mut self, chunk: &[u8]) -> Result<usize, T::Error> {
        let mut cursor = Cursor::new(chunk, self.offset);

        while cursor.remaining() > 0 {
            if self.state == ParserState::Finished {
                cursor.consume_remainder();
                break;
            }

            let opcode = cursor.get_u8().unwrap();
            let mut char_run_ended = true; // most commands end runs of characters

            let rv = match opcode {
                // This is the least ugly way I've found to map the u8 to the
                // symbolic enum values.
                oc if oc == Opcode::Noop as u8 => {
                    char_run_ended = false;
                    Ok(())
                }

                oc if oc >= Opcode::DefineFont1 as u8 && oc <= Opcode::DefineFont4 as u8 => {
                    char_run_ended = false;
                    self.do_define_font(oc, &mut cursor)
                }

                oc if oc == Opcode::DefineNativeFont as u8 => {
                    char_run_ended = false;
                    self.do_define_native_font(oc, &mut cursor)
                }

                oc if oc == Opcode::BeginningOfPage as u8 => {
                    self.do_beginning_of_page(oc, &mut cursor)
                }

                oc if oc == Opcode::EndOfPage as u8 => self.do_end_of_page(oc, &mut cursor),

                oc if oc == Opcode::PushStack as u8 => {
                    char_run_ended = false;
                    self.do_push_stack(oc, &mut cursor)
                }

                oc if oc == Opcode::PopStack as u8 => self.do_pop_stack(oc, &mut cursor),

                oc if oc >= Opcode::Right1 as u8 && oc <= Opcode::Right4 as u8 => {
                    self.do_right(oc, &mut cursor)
                }

                oc if oc == Opcode::RightByW as u8 => self.do_right_by_w(oc, &mut cursor),

                oc if oc >= Opcode::SetW1 as u8 && oc <= Opcode::SetW4 as u8 => {
                    self.do_set_w(oc, &mut cursor)
                }

                oc if oc == Opcode::RightByX as u8 => self.do_right_by_x(oc, &mut cursor),

                oc if oc >= Opcode::SetX1 as u8 && oc <= Opcode::SetX4 as u8 => {
                    self.do_set_x(oc, &mut cursor)
                }

                oc if oc >= Opcode::Down1 as u8 && oc <= Opcode::Down4 as u8 => {
                    self.do_down(oc, &mut cursor)
                }

                oc if oc == Opcode::DownByY as u8 => self.do_down_by_y(oc, &mut cursor),

                oc if oc >= Opcode::SetY1 as u8 && oc <= Opcode::SetY4 as u8 => {
                    self.do_set_y(oc, &mut cursor)
                }

                oc if oc == Opcode::DownByZ as u8 => self.do_down_by_z(oc, &mut cursor),

                oc if oc >= Opcode::SetZ1 as u8 && oc <= Opcode::SetZ4 as u8 => {
                    self.do_set_z(oc, &mut cursor)
                }

                oc if oc >= Opcode::SetFontNumber0 as u8 && oc <= Opcode::SetFontNumber63 as u8 => {
                    self.do_set_font_number(oc, &mut cursor)
                }

                oc if oc >= Opcode::SetFont1 as u8 && oc <= Opcode::SetFont4 as u8 => {
                    self.do_set_font(oc, &mut cursor)
                }

                oc if oc >= Opcode::SetCharNumber0 as u8
                    && oc <= Opcode::SetCharNumber127 as u8 =>
                {
                    char_run_ended = false;
                    self.do_set_char_number(oc, &mut cursor)
                }

                oc if oc >= Opcode::SetChar1 as u8 && oc <= Opcode::SetChar4 as u8 => {
                    char_run_ended = false;
                    self.do_set_char(oc, &mut cursor)
                }

                oc if oc == Opcode::SetGlyphs as u8 => self.do_set_glyphs(oc, &mut cursor),

                oc if oc == Opcode::SetTextAndGlyphs as u8 => {
                    self.do_set_text_and_glyphs(oc, &mut cursor)
                }

                oc if oc >= Opcode::Special1 as u8 && oc <= Opcode::Special4 as u8 => {
                    self.do_special(oc, &mut cursor)
                }

                oc if oc == Opcode::Preamble as u8 => self.do_preamble(oc, &mut cursor),

                oc if oc == Opcode::Postamble as u8 => self.do_postamble(oc, &mut cursor),

                oc if oc == Opcode::DoublePostamble as u8 => {
                    self.do_double_postamble(oc, &mut cursor)
                }

                _ => {
                    return Err(XdvError::IllegalOpcode(opcode, cursor.global_offset()).into());
                }
            };

            match rv {
                Ok(()) => {
                    cursor.checkpoint(); // Opcode was successfully processed.
                }
                Err(InternalError::NeedMoreData) => {
                    break;
                }
                Err(InternalError::Other(e)) => {
                    return Err(e);
                }
            }

            if char_run_ended && !self.cur_char_run.is_empty() {
                self.events.handle_char_run(&self.cur_char_run)?;
                self.cur_char_run.clear();
            }
        }

        self.offset += cursor.checkpoint as u64;
        Ok(cursor.checkpoint)
    }

    fn do_preamble(&mut self, opcode: u8, cursor: &mut Cursor<T>) -> InternalResult<(), T::Error> {
        if self.state != ParserState::Preamble {
            return Err(XdvError::IllegalOpcode(opcode, cursor.global_offset()).into_internal());
        }

        self.filetype = match cursor.get_u8()? {
            b if b == IdByte::Xdv as u8 => FileType::Xdv,
            b if b == IdByte::Spx as u8 => FileType::Spx,
            _ => {
                return Err(XdvError::Malformed(cursor.global_offset()).into_internal());
            }
        };

        cursor.assert_u32(25_400_000)?; // dimensions unit numerator
        cursor.assert_u32(473_628_672)?; // dimensions unit denominator
        cursor.get_u32()?; // 'mag' factor
        let n_comment = cursor.get_u8()?;
        self.events
            .handle_header(self.filetype, cursor.get_slice(n_comment as usize)?)?;

        self.state = ParserState::BetweenPages;
        Ok(())
    }

    fn do_define_font(
        &mut self,
        opcode: u8,
        cursor: &mut Cursor<T>,
    ) -> InternalResult<(), T::Error> {
        if self.state == ParserState::Preamble {
            return Err(XdvError::IllegalOpcode(opcode, cursor.global_offset()).into_internal());
        }

        let _font_num = cursor.get_compact_i32_smpos(opcode - Opcode::DefineFont1 as u8)?;
        let _checksum = cursor.get_u32()?;
        let _scale_factor = cursor.get_u32()?;
        let _design_size = cursor.get_u32()?;
        let area_len = cursor.get_u8()?;
        let name_len = cursor.get_u8()?;
        // XXX TEMP
        use std::str::from_utf8;
        let _area_str = from_utf8(cursor.get_slice(area_len as usize)?)
            .unwrap()
            .to_owned();
        let _name_str = from_utf8(cursor.get_slice(name_len as usize)?)
            .unwrap()
            .to_owned();
        Ok(())
    }

    fn do_define_native_font(
        &mut self,
        opcode: u8,
        cursor: &mut Cursor<T>,
    ) -> InternalResult<(), T::Error> {
        if self.state == ParserState::Preamble {
            return Err(XdvError::IllegalOpcode(opcode, cursor.global_offset()).into_internal());
        }

        let _font_num = cursor.get_i32()?;
        let _size = cursor.get_i32()?; // fixed-point
        let flags = cursor.get_u16()?;
        let name_len = cursor.get_u8()?;
        // XXX TEMP
        use std::str::from_utf8;
        let _name_str = from_utf8(cursor.get_slice(name_len as usize)?)
            .unwrap()
            .to_owned();
        let _face_index = cursor.get_u32()?;

        let _color_rgba = if flags & NativeFontFlags::Colored as u16 != 0 {
            Some(cursor.get_u32()?)
        } else {
            None
        };

        let _extend = if flags & NativeFontFlags::Extend as u16 != 0 {
            Some(cursor.get_u32()?) // fixed-point
        } else {
            None
        };

        let _slant = if flags & NativeFontFlags::Slant as u16 != 0 {
            Some(cursor.get_u32()?) // fixed-point
        } else {
            None
        };

        let _embolden = if flags & NativeFontFlags::Embolden as u16 != 0 {
            Some(cursor.get_u32()?) // fixed-point
        } else {
            None
        };

        Ok(())
    }

    fn do_beginning_of_page(
        &mut self,
        opcode: u8,
        cursor: &mut Cursor<T>,
    ) -> InternalResult<(), T::Error> {
        if self.state != ParserState::BetweenPages {
            return Err(XdvError::IllegalOpcode(opcode, cursor.global_offset()).into_internal());
        }

        let mut counters = [0i32; 10];

        for counter in &mut counters {
            *counter = cursor.get_i32()?;
        }

        let previous_bop = cursor.get_i32()?; // previous beginning-of-page marker
        self.events.handle_begin_page(&counters, previous_bop)?;

        self.state = ParserState::InPage;
        self.stack.clear();
        self.stack.push(State::new());
        self.cur_font_num = None;
        Ok(())
    }

    fn do_end_of_page(
        &mut self,
        opcode: u8,
        cursor: &mut Cursor<T>,
    ) -> InternalResult<(), T::Error> {
        if self.state != ParserState::InPage {
            return Err(XdvError::IllegalOpcode(opcode, cursor.global_offset()).into_internal());
        }

        if self.stack.len() != 1 {
            return Err(XdvError::Malformed(cursor.global_offset()).into_internal());
        }

        self.state = ParserState::BetweenPages;
        Ok(())
    }

    fn do_push_stack(
        &mut self,
        opcode: u8,
        cursor: &mut Cursor<T>,
    ) -> InternalResult<(), T::Error> {
        if self.state != ParserState::InPage {
            return Err(XdvError::IllegalOpcode(opcode, cursor.global_offset()).into_internal());
        }

        let dup = self.stack.last().unwrap().clone();
        self.stack.push(dup);
        Ok(())
    }

    fn do_pop_stack(&mut self, opcode: u8, cursor: &mut Cursor<T>) -> InternalResult<(), T::Error> {
        if self.state != ParserState::InPage {
            return Err(XdvError::IllegalOpcode(opcode, cursor.global_offset()).into_internal());
        }

        if self.stack.len() < 2 {
            return Err(XdvError::Malformed(cursor.global_offset()).into_internal());
        }

        self.stack.pop();
        Ok(())
    }

    fn do_right(&mut self, opcode: u8, cursor: &mut Cursor<T>) -> InternalResult<(), T::Error> {
        if self.state != ParserState::InPage {
            return Err(XdvError::IllegalOpcode(opcode, cursor.global_offset()).into_internal());
        }

        let n = cursor.get_compact_i32_smneg(opcode - Opcode::Right1 as u8)?;
        self.stack.last_mut().unwrap().h += n;
        Ok(())
    }

    fn do_right_by_w(
        &mut self,
        opcode: u8,
        cursor: &mut Cursor<T>,
    ) -> InternalResult<(), T::Error> {
        if self.state != ParserState::InPage {
            return Err(XdvError::IllegalOpcode(opcode, cursor.global_offset()).into_internal());
        }

        let state = self.stack.last_mut().unwrap();
        state.h += state.w;
        Ok(())
    }

    fn do_set_w(&mut self, opcode: u8, cursor: &mut Cursor<T>) -> InternalResult<(), T::Error> {
        if self.state != ParserState::InPage {
            return Err(XdvError::IllegalOpcode(opcode, cursor.global_offset()).into_internal());
        }

        let n = cursor.get_compact_i32_smneg(opcode - Opcode::SetW1 as u8)?;
        let state = self.stack.last_mut().unwrap();
        state.w = n;
        state.h += n;
        Ok(())
    }

    fn do_right_by_x(
        &mut self,
        opcode: u8,
        cursor: &mut Cursor<T>,
    ) -> InternalResult<(), T::Error> {
        if self.state != ParserState::InPage {
            return Err(XdvError::IllegalOpcode(opcode, cursor.global_offset()).into_internal());
        }

        let state = self.stack.last_mut().unwrap();
        state.h += state.x;
        Ok(())
    }

    fn do_set_x(&mut self, opcode: u8, cursor: &mut Cursor<T>) -> InternalResult<(), T::Error> {
        if self.state != ParserState::InPage {
            return Err(XdvError::IllegalOpcode(opcode, cursor.global_offset()).into_internal());
        }

        let n = cursor.get_compact_i32_smneg(opcode - Opcode::SetX1 as u8)?;
        let state = self.stack.last_mut().unwrap();
        state.x = n;
        state.h += n;
        Ok(())
    }

    fn do_down(&mut self, opcode: u8, cursor: &mut Cursor<T>) -> InternalResult<(), T::Error> {
        if self.state != ParserState::InPage {
            return Err(XdvError::IllegalOpcode(opcode, cursor.global_offset()).into_internal());
        }

        let n = cursor.get_compact_i32_smneg(opcode - Opcode::Down1 as u8)?;
        self.stack.last_mut().unwrap().v += n;
        Ok(())
    }

    fn do_down_by_y(&mut self, opcode: u8, cursor: &mut Cursor<T>) -> InternalResult<(), T::Error> {
        if self.state != ParserState::InPage {
            return Err(XdvError::IllegalOpcode(opcode, cursor.global_offset()).into_internal());
        }

        let state = self.stack.last_mut().unwrap();
        state.v += state.y;
        Ok(())
    }

    fn do_set_y(&mut self, opcode: u8, cursor: &mut Cursor<T>) -> InternalResult<(), T::Error> {
        if self.state != ParserState::InPage {
            return Err(XdvError::IllegalOpcode(opcode, cursor.global_offset()).into_internal());
        }

        let n = cursor.get_compact_i32_smneg(opcode - Opcode::SetY1 as u8)?;
        let state = self.stack.last_mut().unwrap();
        state.y = n;
        state.v += n;
        Ok(())
    }

    fn do_down_by_z(&mut self, opcode: u8, cursor: &mut Cursor<T>) -> InternalResult<(), T::Error> {
        if self.state != ParserState::InPage {
            return Err(XdvError::IllegalOpcode(opcode, cursor.global_offset()).into_internal());
        }

        let state = self.stack.last_mut().unwrap();
        state.v += state.z;
        Ok(())
    }

    fn do_set_z(&mut self, opcode: u8, cursor: &mut Cursor<T>) -> InternalResult<(), T::Error> {
        if self.state != ParserState::InPage {
            return Err(XdvError::IllegalOpcode(opcode, cursor.global_offset()).into_internal());
        }

        let n = cursor.get_compact_i32_smneg(opcode - Opcode::SetZ1 as u8)?;
        let state = self.stack.last_mut().unwrap();
        state.z = n;
        state.v += n;
        Ok(())
    }

    /// This variant uses the opcode to encode the font number.
    fn do_set_font_number(
        &mut self,
        opcode: u8,
        cursor: &mut Cursor<T>,
    ) -> InternalResult<(), T::Error> {
        if self.state != ParserState::InPage {
            return Err(XdvError::IllegalOpcode(opcode, cursor.global_offset()).into_internal());
        }

        self.cur_font_num = Some(i32::from(opcode - Opcode::SetFontNumber0 as u8));
        Ok(())
    }

    /// This variant takes an argument that encodes the font number.
    fn do_set_font(&mut self, opcode: u8, cursor: &mut Cursor<T>) -> InternalResult<(), T::Error> {
        if self.state != ParserState::InPage {
            return Err(XdvError::IllegalOpcode(opcode, cursor.global_offset()).into_internal());
        }

        self.cur_font_num = Some(cursor.get_compact_i32_smpos(opcode - Opcode::SetFont1 as u8)?);
        Ok(())
    }

    /// This variant uses the opcode to encode the character number.
    fn do_set_char_number(
        &mut self,
        opcode: u8,
        cursor: &mut Cursor<T>,
    ) -> InternalResult<(), T::Error> {
        if self.state != ParserState::InPage {
            return Err(XdvError::IllegalOpcode(opcode, cursor.global_offset()).into_internal());
        }

        let char_num = opcode - Opcode::SetCharNumber0 as u8;
        self.cur_char_run.push(i32::from(char_num));
        Ok(())
    }

    /// This variant takes an argument that encodes the character number.
    fn do_set_char(&mut self, opcode: u8, cursor: &mut Cursor<T>) -> InternalResult<(), T::Error> {
        if self.state != ParserState::InPage {
            return Err(XdvError::IllegalOpcode(opcode, cursor.global_offset()).into_internal());
        }

        let char_num = cursor.get_compact_i32_smpos(opcode - Opcode::SetChar1 as u8)?;
        self.cur_char_run.push(char_num as i32);
        Ok(())
    }

    fn do_set_glyphs(
        &mut self,
        opcode: u8,
        cursor: &mut Cursor<T>,
    ) -> InternalResult<(), T::Error> {
        if self.state != ParserState::InPage {
            return Err(XdvError::IllegalOpcode(opcode, cursor.global_offset()).into_internal());
        }

        let _width = cursor.get_i32()?;
        let n_glyphs = cursor.get_u16()?;

        for _ in 0..n_glyphs {
            let _x = cursor.get_u32()?;
            let _y = cursor.get_u32()?;
        }

        for _ in 0..n_glyphs {
            let _glyph_id = cursor.get_i16()?;
        }

        Ok(())
    }

    fn do_set_text_and_glyphs(
        &mut self,
        opcode: u8,
        cursor: &mut Cursor<T>,
    ) -> InternalResult<(), T::Error> {
        if self.state != ParserState::InPage {
            return Err(XdvError::IllegalOpcode(opcode, cursor.global_offset()).into_internal());
        }

        let n_chars = cursor.get_u16()?;
        let mut chars = Vec::new();

        for _ in 0..n_chars {
            chars.push(cursor.get_u16()?);
        }

        let _width = cursor.get_i32()?;
        let n_glyphs = cursor.get_u16()?;

        for _ in 0..n_glyphs {
            let _x = cursor.get_u32()?;
            let _y = cursor.get_u32()?;
        }

        for _ in 0..n_glyphs {
            let _glyph_id = cursor.get_i16()?;
        }

        Ok(())
    }

    fn do_special(&mut self, opcode: u8, cursor: &mut Cursor<T>) -> InternalResult<(), T::Error> {
        if self.state != ParserState::InPage {
            return Err(XdvError::IllegalOpcode(opcode, cursor.global_offset()).into_internal());
        }

        let n = cursor.get_compact_u32(opcode - Opcode::Special1 as u8)?;
        self.events.handle_special(cursor.get_slice(n as usize)?)?;

        Ok(())
    }

    fn do_postamble(&mut self, opcode: u8, cursor: &mut Cursor<T>) -> InternalResult<(), T::Error> {
        if self.state != ParserState::BetweenPages {
            return Err(XdvError::IllegalOpcode(opcode, cursor.global_offset()).into_internal());
        }

        cursor.get_u32()?; // last_bop
        cursor.assert_u32(25_400_000)?; // dimensions unit numerator
        cursor.assert_u32(473_628_672)?; // dimensions unit denominator
        cursor.get_u32()?; // 'mag' factor
        cursor.get_u32()?; // largest height+depth of tallest page
        cursor.get_u32()?; // largest width of widest page
        cursor.get_u16()?; // maximum stack depth
        cursor.get_u16()?; // number of pages

        self.state = ParserState::PostambleFontDefinitions;
        Ok(())
    }

    fn do_double_postamble(
        &mut self,
        opcode: u8,
        cursor: &mut Cursor<T>,
    ) -> InternalResult<(), T::Error> {
        if self.state != ParserState::PostambleFontDefinitions {
            return Err(XdvError::IllegalOpcode(opcode, cursor.global_offset()).into_internal());
        }

        cursor.get_u32()?; // pointer to postamble
        cursor.assert_u8(match self.filetype {
            FileType::Xdv => IdByte::Xdv,
            FileType::Spx => IdByte::Spx,
        } as u8)?;
        cursor.assert_u32(0xDFDF_DFDF)?; // at least four 0xDF's

        self.state = ParserState::Finished;
        Ok(())
    }

    /// Get the current byte offset of the parsing.
    pub fn current_offset(&self) -> u64 {
        self.offset
    }

    /// Finish parsing, consume this object, and return the underlying event
    /// handler, assuming all went well.
    pub fn finish(self) -> Result<T, T::Error> {
        if self.state != ParserState::Finished {
            return Err(XdvError::UnexpectedEndOfStream.into());
        }

        Ok(self.events)
    }
}

/// The states that may be stacked while processing the DVI.
#[derive(Clone, Debug, Eq, PartialEq)]
struct State {
    pub h: i32,
    pub v: i32,
    pub w: i32,
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl State {
    pub fn new() -> Self {
        State {
            h: 0,
            v: 0,
            w: 0,
            x: 0,
            y: 0,
            z: 0,
        }
    }
}

/// A simple cursor on a buffer.
#[derive(Debug)]
struct Cursor<'a, T: XdvEvents> {
    /// The *remaining* unprocessed bytes.
    buf: &'a [u8],

    /// How many bytes from the *original* chunk buffer have been fully processed.
    pub checkpoint: usize,

    /// How many bytes from the *original* chunk buffer we have looked at.
    offset: usize,

    /// The offset into the *total stream* that the original chunk buffer was
    /// at.
    global_offset: u64,

    /// Helps us avoid tedious type annotations.
    _events: PhantomData<T>,
}

impl<'a, T: XdvEvents> Cursor<'a, T> {
    pub fn new(buf: &'a [u8], global_offset: u64) -> Self {
        Cursor {
            buf,
            checkpoint: 0,
            offset: 0,
            global_offset,
            _events: PhantomData,
        }
    }

    pub fn remaining(&self) -> usize {
        self.buf.len()
    }

    pub fn global_offset(&self) -> u64 {
        self.global_offset + self.offset as u64
    }

    pub fn checkpoint(&mut self) {
        self.checkpoint = self.offset;
    }

    pub fn consume_remainder(&mut self) {
        self.offset += self.buf.len();
        self.checkpoint();
        self.buf = &[0u8; 0];
    }

    pub fn get_u8(&mut self) -> InternalResult<u8, T::Error> {
        if self.buf.is_empty() {
            return Err(InternalError::NeedMoreData);
        }

        let rv = self.buf[0];
        self.buf = &self.buf[1..];
        self.offset += 1;
        Ok(rv)
    }

    pub fn assert_u8(&mut self, expected: u8) -> InternalResult<(), T::Error> {
        if self.get_u8()? == expected {
            Ok(())
        } else {
            Err(XdvError::Malformed(self.global_offset + self.offset as u64 - 1).into_internal())
        }
    }

    pub fn get_i8(&mut self) -> InternalResult<i8, T::Error> {
        if self.buf.is_empty() {
            return Err(InternalError::NeedMoreData);
        }

        let rv = unsafe { mem::transmute(self.buf[0]) };
        self.buf = &self.buf[1..];
        self.offset += 1;
        Ok(rv)
    }

    pub fn get_u16(&mut self) -> InternalResult<u16, T::Error> {
        if self.buf.len() < 2 {
            return Err(InternalError::NeedMoreData);
        }

        let rv = BigEndian::read_u16(&self.buf[..2]);
        self.buf = &self.buf[2..];
        self.offset += 2;
        Ok(rv)
    }

    pub fn get_i16(&mut self) -> InternalResult<i16, T::Error> {
        if self.buf.len() < 2 {
            return Err(InternalError::NeedMoreData);
        }

        let rv = BigEndian::read_i16(&self.buf[..2]);
        self.buf = &self.buf[2..];
        self.offset += 2;
        Ok(rv)
    }

    pub fn get_u24(&mut self) -> InternalResult<u32, T::Error> {
        if self.buf.len() < 3 {
            return Err(InternalError::NeedMoreData);
        }

        let rv = BigEndian::read_u24(&self.buf[..3]);
        self.buf = &self.buf[3..];
        self.offset += 3;
        Ok(rv)
    }

    pub fn get_i24(&mut self) -> InternalResult<i32, T::Error> {
        if self.buf.len() < 3 {
            return Err(InternalError::NeedMoreData);
        }

        let rv = BigEndian::read_i24(&self.buf[..3]);
        self.buf = &self.buf[3..];
        self.offset += 3;
        Ok(rv)
    }

    pub fn get_u32(&mut self) -> InternalResult<u32, T::Error> {
        if self.buf.len() < 4 {
            return Err(InternalError::NeedMoreData);
        }

        let rv = BigEndian::read_u32(&self.buf[..4]);
        self.buf = &self.buf[4..];
        self.offset += 4;
        Ok(rv)
    }

    pub fn get_compact_u32(&mut self, size_marker: u8) -> InternalResult<u32, T::Error> {
        match size_marker {
            0 => Ok(u32::from(self.get_u8()?)),
            1 => Ok(u32::from(self.get_u16()?)),
            2 => Ok(self.get_u24()?),
            3 => self.get_u32(),
            _ => Err(XdvError::Malformed(self.global_offset()).into_internal()),
        }
    }

    pub fn assert_u32(&mut self, expected: u32) -> InternalResult<(), T::Error> {
        if self.get_u32()? == expected {
            Ok(())
        } else {
            Err(XdvError::Malformed(self.global_offset + self.offset as u64 - 4).into_internal())
        }
    }

    pub fn get_i32(&mut self) -> InternalResult<i32, T::Error> {
        if self.buf.len() < 4 {
            return Err(InternalError::NeedMoreData);
        }

        let rv = BigEndian::read_i32(&self.buf[..4]);
        self.buf = &self.buf[4..];
        self.offset += 4;
        Ok(rv)
    }

    /// This variation lets small values be signed (used by right, down, etc).
    pub fn get_compact_i32_smneg(&mut self, size_marker: u8) -> InternalResult<i32, T::Error> {
        match size_marker {
            0 => Ok(i32::from(self.get_i8()?)),
            1 => Ok(i32::from(self.get_i16()?)),
            2 => Ok(self.get_i24()?),
            3 => self.get_i32(),
            _ => Err(XdvError::Malformed(self.global_offset()).into_internal()),
        }
    }

    /// This variation has unsigned small values (used by fnt_def).
    pub fn get_compact_i32_smpos(&mut self, size_marker: u8) -> InternalResult<i32, T::Error> {
        match size_marker {
            0 => Ok(i32::from(self.get_u8()?)),
            1 => Ok(i32::from(self.get_u16()?)),
            2 => Ok(self.get_u24()? as i32),
            3 => self.get_i32(),
            _ => Err(XdvError::Malformed(self.global_offset()).into_internal()),
        }
    }

    pub fn get_slice(&mut self, n: usize) -> InternalResult<&[u8], T::Error> {
        if self.buf.len() < n {
            return Err(InternalError::NeedMoreData);
        }

        let (rv, remainder) = self.buf.split_at(n);
        self.buf = remainder;
        self.offset += n;
        Ok(rv)
    }
}

/// XDV opcodes.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u8)]
enum Opcode {
    SetCharNumber0 = 0,
    SetCharNumber127 = 127,
    SetChar1 = 128,
    SetChar4 = 131,
    Noop = 138,
    BeginningOfPage = 139,
    EndOfPage = 140,
    PushStack = 141,
    PopStack = 142,
    Right1 = 143,
    Right4 = 146,
    RightByW = 147,
    SetW1 = 148,
    SetW4 = 151,
    RightByX = 152,
    SetX1 = 153,
    SetX4 = 156,
    Down1 = 157,
    Down4 = 160,
    DownByY = 161,
    SetY1 = 162,
    SetY4 = 165,
    DownByZ = 166,
    SetZ1 = 167,
    SetZ4 = 170,
    SetFontNumber0 = 171,
    SetFontNumber63 = 234,
    SetFont1 = 235,
    SetFont4 = 238,
    Special1 = 239,
    Special4 = 242,
    DefineFont1 = 243,
    DefineFont4 = 246,
    Preamble = 247,
    Postamble = 248,
    DoublePostamble = 249,
    DefineNativeFont = 252,
    SetGlyphs = 253,        // "SET_GLYPHS", "XDV_GLYPHS"
    SetTextAndGlyphs = 254, // "SET_TEXT_AND_GLYPHS", "XDV_TEXT_AND_GLYPHS"
}

/// Identifier bytes used by the XDV writing code.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u8)]
enum IdByte {
    Xdv = 7,
    Spx = 100,
}

/// Flags for XeTeX native fonts
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u16)]
#[allow(unused)]
enum NativeFontFlags {
    Vertical = 0x0100,
    Colored = 0x0200,
    Extend = 0x1000,
    Slant = 0x2000,
    Embolden = 0x4000,
}
