use crate::chrome::*;
use crate::print_error::*;
use std::fs::File;
use std::io;
use std::io::prelude::*;

pub fn main_strings(filename: &str, should_have_color_support: bool) -> Result<(), String> {
    let b_arr_size: usize = 0x4000;
    let mut f = File::open(filename)
        .map_err(|e| format!("Cannot open {}\n\nCaused by:\n {}", filename, e))?;
    let chrome_instance = ChromeInstance::new(should_have_color_support);
    let mut s = Strings::new(chrome_instance);
    loop {
        let mut b_arr = Vec::with_capacity(b_arr_size);
        let n = Read::by_ref(&mut f)
            .take(b_arr_size as u64)
            .read_to_end(&mut b_arr)
            .map_err(|e| format!("Cannot read from {}\n\nCaused by:\n {}", filename, e))?;
        if n == 0 {
            break;
        }
        for i in 0..n {
            s.process_byte(b_arr[i]);
        }
        if n < b_arr_size {
            break;
        }
    }
    Ok(())
}

struct Strings {
    byte_count: usize,
    curr_byte_offset: usize,
    curr_string: String,
    chrome_instance: ChromeInstance,
}

impl Strings {
    fn new(chrome_instance: ChromeInstance) -> Strings {
        Strings {
            byte_count: 0,
            curr_byte_offset: 0,
            curr_string: String::new(),
            chrome_instance: chrome_instance,
        }
    }

    fn is_print(curr_byte: u8) -> bool {
        curr_byte >= 0x20 && curr_byte <= 0x7E
    }

    fn process_byte(&mut self, curr_byte: u8) {
        if Strings::is_print(curr_byte) {
            self.curr_string.push(curr_byte as char);
            self.byte_count += 1;
        } else {
            if self.curr_string.len() == 0 {
                self.byte_count += 1;
                self.curr_byte_offset = self.byte_count;
            } else  {
                // paging + Linux being weird here
                if self.curr_string.len() > 1 {
                    pe(writeln!(
                        io::stdout(),
                        "{}{:#08x}{}: \"{}{}{}\" {}// {}-{} ({:#08x}-{:#08x}){}",
                        self.chrome_instance.get_red(),
                        self.curr_byte_offset,
                        self.chrome_instance.get_reset(),
                        self.chrome_instance.get_green(),
                        self.curr_string,
                        self.chrome_instance.get_reset(),
                        self.chrome_instance.get_grey(),
                        self.curr_byte_offset,
                        self.curr_byte_offset + self.curr_string.len(),
                        self.curr_byte_offset,
                        self.curr_byte_offset + self.curr_string.len(),
                        self.chrome_instance.get_reset()
                    ));
                }
                self.curr_string.clear();
                self.byte_count += 1;
            }
        }
        self.byte_count = self.byte_count + 1;
    }
}
