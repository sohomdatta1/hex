use crate::chrome::*;
use crate::print_error::*;
use std::fs::File;
use std::io;
use std::io::prelude::*;

pub fn main_search(filename: &str, search_str: &[u8], should_have_color_support: bool) -> Result<(), String> {
    let b_arr_size: usize = 0x4000;
    let mut f = File::open(filename)
        .map_err(|e| format!("Cannot open {}\n\nCaused by:\n {}", filename, e))?;
    let chrome_instance = ChromeInstance::new(should_have_color_support);
    let mut s = Search::new(chrome_instance);
    loop {
        let mut b_arr = Vec::with_capacity(b_arr_size);
        let n = Read::by_ref(&mut f)
            .take(b_arr_size as u64)
            .read_to_end(&mut b_arr)
            .map_err(|e| format!("Cannot read from {}\n\nCaused by:\n {}", filename, e))?;
        if n == 0 {
            break;
        }
        s.process_bytes(&b_arr, search_str);
        if n < b_arr_size {
            break;
        }
    }
    s.dump_results();
    Ok(())
}

struct Search {
    chrome_instance: ChromeInstance,
    results: Vec<usize>,
}

impl Search {
    fn new(chrome_instance: ChromeInstance) -> Search {
        Search {
            chrome_instance: chrome_instance,
            results: Vec::new(),
        }
    }

    fn create_bad_chars_arr(search_str: &[u8]) -> Vec<isize> {
        let mut c = 0;
        let mut bad_arr= Vec::with_capacity(256);
        bad_arr.resize(256, -1);
        for i in search_str {
            bad_arr[*i as usize] = c;
            c += 1;
        }
        bad_arr
    }

    fn process_bytes(&mut self, curr_buf: &[u8], search_str: &[u8]) {
        let bad_arr = Search::create_bad_chars_arr(search_str);
        let n = curr_buf.len();
        let m = search_str.len();
        let mut s = 0;
        while s <= n - m {
            let mut j: isize = (m - 1) as isize;
            while j >= 0 && search_str[j as usize] == curr_buf[s + j as usize] {
                j -= 1;
            }
            if j < 0 {
                self.results.push(s as usize);
                let skip = m - if s + m < n { bad_arr[curr_buf[s + m] as usize] } else { 1 as isize } as usize;
                s += skip;
            } else {
                s += std::cmp::max(j - bad_arr[curr_buf[(s + j as usize) as usize] as usize], 1) as usize
            }
        }
    }

    fn dump_results(&mut self) {

        for result in &self.results {
            pe(
                write!(
                    io::stdout(),
                    "{}0x{:08x}{}",
                    self.chrome_instance.get_green(),
                    result,
                    self.chrome_instance.get_reset()
                )
            );
        }
    }
}