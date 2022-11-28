//
// MIT License
//
// Copyright (c) 2022 AtomicGamer9523
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
//

#[doc(hidden)]
use std3::__reexports::lazy_static::lazy_static;
#[doc(hidden)]
use std3::ops::{Deref, DerefMut};
#[doc(hidden)]
use std3::__reexports::volatile::Volatile;
#[doc(hidden)]
pub use std3::fmt::{self,Write};
#[doc(hidden)]
use std3::sync::Mutex;

lazy_static! {
    #[doc(hidden)]
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
        column_position: 0,
        color_code: ColorCode::new(Color::White, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    });
    #[doc(hidden)]
    pub static ref INFOWRITER: Mutex<Writer> = Mutex::new(Writer {
        column_position: 0,
        color_code: ColorCode::new(Color::Magenta, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    });
    #[doc(hidden)]
    pub static ref LOGOWRITER: Mutex<Writer> = Mutex::new(Writer {
        column_position: 0,
        color_code: ColorCode::new(Color::Brown, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    });
    #[doc(hidden)]
    pub static ref OKWRITER: Mutex<Writer> = Mutex::new(Writer {
        column_position: 0,
        color_code: ColorCode::new(Color::Green, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    });
    #[doc(hidden)]
    pub static ref TRACEWRITER: Mutex<Writer> = Mutex::new(Writer {
        column_position: 0,
        color_code: ColorCode::new(Color::LightBlue, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    });
    #[doc(hidden)]
    pub static ref WARNWRITER: Mutex<Writer> = Mutex::new(Writer {
        column_position: 0,
        color_code: ColorCode::new(Color::LightRed, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    });
    #[doc(hidden)]
    pub static ref ERRWRITER: Mutex<Writer> = Mutex::new(Writer {
        column_position: 0,
        color_code: ColorCode::new(Color::Red, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    });
}




/// A VGA color.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
    /// Black.
    Black = 0,
    /// Blue.
    Blue = 1,
    /// Green.
    Green = 2,
    /// Cyan.
    Cyan = 3,
    /// Red.
    Red = 4,
    /// Magenta.
    Magenta = 5,
    /// Brown.
    Brown = 6,
    /// Light Gray.
    LightGray = 7,
    /// Dark Gray.
    DarkGray = 8,
    /// Light Blue.
    LightBlue = 9,
    /// Light Green.
    LightGreen = 10,
    /// Light Cyan.
    LightCyan = 11,
    /// Light Red.
    LightRed = 12,
    /// Light Magenta.
    LightMagenta = 13,
    /// Yellow.
    Yellow = 14,
    /// White.
    White = 15,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
struct ColorCode(u8);

impl ColorCode {
    pub fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}





#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
    ascii_character: u8,
    color_code: ColorCode,
}
impl Deref for ScreenChar {
    type Target = ScreenChar;

    fn deref(&self) -> &ScreenChar {
        self
    }
}

impl DerefMut for ScreenChar {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self
    }
}


const BUFFER_WIDTH:  usize = 80;
const BUFFER_HEIGHT: usize = 25;






#[repr(transparent)]
#[derive(Debug, Clone)]
struct Buffer {
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

/// A writer that writes to the VGA text buffer.
#[derive(Debug)]
pub struct Writer {
    column_position: usize,
    color_code: ColorCode,
    buffer: &'static mut Buffer,
}
impl Writer {
    /// Writes a byte character to the screen
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();
                }
                let row = BUFFER_HEIGHT - 1;
                let col = self.column_position;
                let color_code = self.color_code;
                self.buffer.chars[row][col].write(ScreenChar {
                    ascii_character: byte,
                    color_code,
                });
                self.column_position += 1;
            }
        }
    }
    #[doc(hidden)]
    fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                _ => self.write_byte(0xfe),
            }
        }
    }
    #[doc(hidden)]
    fn new_line(&mut self) {
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                let character = self.buffer.chars[row][col].read();
                self.buffer.chars[row - 1][col].write(character);
            }
        }
        self.clear_row(BUFFER_HEIGHT - 1);
        self.column_position = 0;
    }
    #[doc(hidden)]
    fn clear_row(&mut self, row: usize) {
        let blank = ScreenChar {
            ascii_character: b' ',
            color_code: ColorCode::new(Color::Black, Color::Black),
        };
        for col in 0..BUFFER_WIDTH {
            self.buffer.chars[row][col].write(blank);
        }
    }
}
impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}
