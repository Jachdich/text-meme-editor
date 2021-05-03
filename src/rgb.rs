extern crate termion;

#[derive(Copy, Clone, Debug)]
pub struct RGB {
	pub r: u8,
	pub g: u8,
	pub b: u8,
	pub default: bool,
}

impl RGB {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        RGB {
            r: r, g: g, b: b,
            default: false
        }
    }
    pub fn from_html(n: u32) -> Self {
    	let r: u8 = ((n >> 16) & 0xFF) as u8;
    	let g: u8 = ((n >> 8)  & 0xFF) as u8;
    	let b: u8 = ((n >> 0)  & 0xFF) as u8;
    	RGB {
    		r:r, g:g, b:b, default:false
    	}
    }
    pub fn to_fg(&self) -> termion::color::Fg<termion::color::Rgb> {
    	return termion::color::Fg(termion::color::Rgb(self.r, self.g, self.b));
    }
    pub fn to_bg(&self) -> termion::color::Bg<termion::color::Rgb> {
    	return termion::color::Bg(termion::color::Rgb(self.r, self.g, self.b));
    }
    pub fn get_inverted(&self) -> RGB {
    	let txt_col: RGB;
        if self.r as u16 + self.g as u16 + self.b as u16 > 384 {
        	txt_col = RGB::new(0, 0, 0);
        } else {
        	txt_col = RGB::new(255, 255, 255);
        }
        return txt_col;
    }

    pub fn to_html_string(&self) -> String {
    	if self.default {
    		return "default".to_string();
    	}
    	format!("#{:02X?}{:02X?}{:02X?}", self.r, self.g, self.b)
    }
}

impl std::cmp::PartialEq for RGB {
    fn eq(&self, other: &Self) -> bool {
        self.r == other.r && self.g == other.g && self.b == other.b
    }
}