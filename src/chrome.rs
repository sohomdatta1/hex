use crate::envman::get_env;

pub fn should_have_color_support() -> bool {
    termion::is_tty(&mut std::io::stdout()) && get_env("NO_COLOR").len() == 0
}

pub struct ChromeInstance {
    color_is_supported: bool,
}

impl ChromeInstance {
    pub fn get_green(&mut self) -> &'static str {
        if self.color_is_supported {
            "\x1b[38;2;60;174;163m"
            //color::Rgb(0x3c, 0xae, 0xa3).fg_string()
        } else {
            ""
        }
    }

    pub fn get_red(&mut self) -> &'static str {
        if self.color_is_supported {
            "\x1b[38;2;220;50;47m"
            //color::Rgb(0xdc, 0x32, 0x2f).fg_string()
        } else {
            ""
        }
    }

    pub fn get_grey(&mut self) -> &'static str {
        if self.color_is_supported {
            "\x1b[38;2;68;68;68m"
            // color::Rgb(0x44, 0x44, 0x44).fg_string()
        } else {
            ""
        }
    }

    pub fn get_reset(&mut self) -> &'static str {
        if self.color_is_supported {
            "\x1b[39m"
        } else {
            ""
        }
    }

    pub fn new(color_is_supported: bool) -> ChromeInstance {
        ChromeInstance { color_is_supported }
    }
}
