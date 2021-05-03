extern crate termion;

use termion::raw::IntoRawMode;
use termion::event::{Key, Event};
use std::io::{Write, stdout, stdin};
use crate::termion::input::TermRead;

mod rgb;
mod parser;
mod format_char;
use format_char::FmtChar;
use rgb::RGB;


#[derive(PartialEq)]
enum Focus {
    Image,
    Colours,
    Charsheet,
}

#[derive(Debug, PartialEq)]
enum Tool {
    None,
    Pen,
    Paint,
    Text
}

struct ImageView {
    data: Vec<Vec<FmtChar>>,
    width: u16,
    height: u16,
    cur_x: u16,
    cur_y: u16,
    curr_buffer: String,
    tool: Tool,
    tool_down: bool,
}

struct ColoursView {
    fg: RGB,
    bg: RGB,
    cur_x: u16,
    cur_y: u16,
    inp_fg: u8,
    inp_bg: u8,
    inp_buffer: String,
    colours: Vec<RGB>,
}

struct CharView {
	char_sheet: Vec<Vec<char>>,
	cur_x: u16,
	cur_y: u16,
    pen_char: char,
}

impl CharView {
    fn new() -> Self {
        let char_sheet_txt = std::fs::read_to_string("character_sheet.txt").unwrap();
    	let char_sheet     = parser::make_char_sheet(char_sheet_txt);
        CharView {
            char_sheet,
            cur_x: 1,
            cur_y: 1,
            pen_char: ' ',
        }
    }
    fn handle_event(&mut self, event: termion::event::Event) {
        match event.clone() {
            Event::Key(Key::Left)  => self.cur_x -= 1,
			Event::Key(Key::Right) => self.cur_x += 1,
			Event::Key(Key::Up)    => self.cur_y -= 1,
			Event::Key(Key::Down)  => self.cur_y += 1,
			_ => (),
        }
        if self.cur_x < 1 { self.cur_x = 1; }
        if self.cur_y < 1 { self.cur_y = 1; }
        if self.cur_x > self.char_sheet[0].len() as u16 { self.cur_x = self.char_sheet[0].len() as u16; }
        if self.cur_y > self.char_sheet.len() as u16 { self.cur_y = self.char_sheet.len() as u16; }
        self.pen_char = self.char_sheet[self.cur_y as usize - 1][self.cur_x as usize - 1];
    }
    
    fn draw(&self, ax: u16, ay: u16) -> String {
        let mut x: u16 = 0;
        let mut y: u16 = 0;
        let mut chsheet_render = format!("{}{}",
            termion::color::Fg(termion::color::Reset),
            termion::color::Bg(termion::color::Reset));
        for line in &self.char_sheet {
            for ch in line {
                chsheet_render.push_str(&format!("{}{}",
                	termion::cursor::Goto(ax + x, ay + y), ch));
                x += 2;
            }
            y += 2;
            x = 0;
        }
        chsheet_render.push_str(&format!("{}╭─╮{}│{}│{}╰─╯",
        	termion::cursor::Goto(ax + (self.cur_x - 1) * 2 - 1, ay + (self.cur_y - 1) * 2 - 1),
			termion::cursor::Goto(ax + (self.cur_x - 1) * 2 - 1, ay + (self.cur_y - 1) * 2),
			termion::cursor::Goto(ax + (self.cur_x - 1) * 2 + 1, ay + (self.cur_y - 1) * 2),
			termion::cursor::Goto(ax + (self.cur_x - 1) * 2 - 1, ay + (self.cur_y - 1) * 2 + 1)));
        chsheet_render
    }
}

impl ColoursView {
    fn new() -> Self {
        let colours = [0xffffff, 0xffff01, 0xff6600, 0xde0000,
                       0xff0198, 0x330099, 0x0001cd, 0x0098fe,
                       0x01ab02, 0x016701, 0x673301, 0x9a6634,
                       0xbbbbbb, 0x888888, 0x444444, 0x000000].iter().map(|x| RGB::from_html(*x)).collect::<Vec<RGB>>();
        ColoursView {
            fg: RGB::new(0, 0, 0),
            bg: RGB::new(255, 255, 255),
            cur_x: 1,
            cur_y: 1,
            inp_fg: 0,
            inp_bg: 0,
            inp_buffer: "".to_string(),
            colours
        }
    }
    
    fn handle_event(&mut self, event: termion::event::Event) {
        match event.clone() {
            Event::Key(Key::Left) => self.cur_x -= 1,
			Event::Key(Key::Right) => self.cur_x += 1,
			Event::Key(Key::Up) => self.cur_y -= 1,
			Event::Key(Key::Down) => self.cur_y += 1,
			Event::Key(Key::Char('\n')) => {
				self.fg = self.colours[((self.cur_y - 1) * 4 + (self.cur_x - 1)) as usize];
			},
			Event::Key(Key::Backspace) => {
				self.bg = self.colours[((self.cur_y - 1) * 4 + (self.cur_x - 1)) as usize];
			},
			Event::Key(Key::Char('r')) => {
				self.fg.default = !self.fg.default;
			},
            Event::Key(Key::Char('R')) => {
				self.bg.default = !self.bg.default;
			},
            Event::Key(Key::Char('#')) => {
                if self.inp_fg == 0 && self.inp_bg == 0 {
                    self.inp_buffer = self.fg.to_html_string();
                    self.inp_fg = 1;
                }
            },
            Event::Key(Key::Char('~')) => {
                if self.inp_fg == 0 && self.inp_bg == 0 {
                    self.inp_buffer = self.bg.to_html_string();
                    self.inp_bg = 1;
                }
            },
            Event::Key(Key::Char(c)) => {
                if c.is_digit(16) {
                    if self.inp_fg != 0 || self.inp_bg != 0 {
                        //fucking stupid hack to get around the
                        //lack of ability to set a char in a string in rust
                        let mut res = String::with_capacity(self.inp_buffer.len());
                        let mut idx = 0;
                        for ch in self.inp_buffer.chars() {
                            //evil hack because one of these will (hopefully) always be zero
                            if idx == (self.inp_fg + self.inp_bg) {
                                res.push(c);
                            } else {
                                res.push(ch);
                            }
                            idx += 1;
                        }
                        self.inp_buffer = res;
                    }
                    if self.inp_fg != 0 {
                        self.inp_fg += 1;
                        if self.inp_fg > 6 {
                            self.inp_fg = 0;
                            self.fg = RGB::from_html(u32::from_str_radix(self.inp_buffer.trim_start_matches("#"), 16).unwrap());
                        }
                    }
                    if self.inp_bg != 0 {
                        self.inp_bg += 1;
                        if self.inp_bg > 6 {
                            self.inp_bg = 0;
                            self.bg = RGB::from_html(u32::from_str_radix(self.inp_buffer.trim_start_matches("#"), 16).unwrap());
                        }
                    }
                }
            },
			_ => ()
		}
		if self.cur_x < 1 { self.cur_x = 1; }
		if self.cur_y < 1 { self.cur_y = 1; }
		if self.cur_x > 4 { self.cur_x = 4; }
		if self.cur_y > 4 { self.cur_y = 4; }
	}
	
	fn draw(&self, x: u16, y: u16) -> String {
	    let fg_string: String;
	    let bg_string: String;
	    if self.inp_fg != 0 { fg_string = self.inp_buffer.clone();
	    } else { fg_string = self.fg.to_html_string(); }
	    if self.inp_bg != 0 { bg_string = self.inp_buffer.clone();
	    } else { bg_string = self.bg.to_html_string(); }
	    
		let mut render = "".to_string();
	    for row in 0..4 as usize {
	       	render.push_str(&format!("{}{}{}█{}█{}█{}█", 
				termion::cursor::Goto(x, row as u16 + y),
				termion::color::Bg(termion::color::Reset),
				self.colours[row * 4 + 0].to_fg(),
				self.colours[row * 4 + 1].to_fg(),
				self.colours[row * 4 + 2].to_fg(),
				self.colours[row * 4 + 3].to_fg(),
	       	));
	    }
	
		let sel = self.colours[((self.cur_y - 1) * 4 + (self.cur_x - 1)) as usize];
		render.push_str(&format!("{}{}{}╳{}{}{}{}{}{}{}{}",
	    	termion::cursor::Goto((self.cur_x - 1) + x, (self.cur_y - 1) + y),
	        sel.get_inverted().to_fg(),
	        sel.to_bg(),
			termion::cursor::Goto(x, y + 4),
			self.fg.to_bg(),
			self.fg.get_inverted().to_fg(),
			fg_string,
			termion::cursor::Goto(x, y + 5),
			self.bg.to_bg(),
			self.bg.get_inverted().to_fg(),
			bg_string,
		));
		render
	}
}


impl ImageView {
    fn new() -> Self {
    	let contents = std::fs::read_to_string("mem1.txt").unwrap();
    	let data = parser::make_data(contents.chars().collect::<Vec<char>>());
    	let width = data[0].len() as u16;
    	let height = data.len() as u16;
    	let curr_buffer = parser::construct_buffer(&data);
    	ImageView {
    	    data,
    	    width,
    	    height,
    	    cur_x: 1,
    	    cur_y: 1,
    	    curr_buffer,
    	    tool: Tool::None,
    	    tool_down: false
    	}
    }
    
    fn handle_event(&mut self, event: termion::event::Event, colours: &mut ColoursView, chars: &mut CharView) {
        match event.clone() {
            Event::Key(Key::Left) => self.cur_x -= 1,
			Event::Key(Key::Right) => self.cur_x += 1,
			Event::Key(Key::Up) => self.cur_y -= 1,
			Event::Key(Key::Down) => self.cur_y += 1,
			Event::Key(Key::Char('\n')) => self.tool_down = !self.tool_down,
		    _ => (),
        }
        if self.cur_x < 1 { self.cur_x = 1 }
        if self.cur_y < 1 { self.cur_y = 1; }
        if self.cur_x > self.width  { self.cur_x = self.width; }
        if self.cur_y > self.height { self.cur_y = self.height; }

        if !self.tool_down {
           match event.clone() {
   			    Event::Key(Key::Char('t')) => {
       				self.tool = Tool::Text;
       			}
       			Event::Key(Key::Char('p')) => {
                    self.tool = Tool::Pen;
       			}
       			Event::Key(Key::Char('o')) => {
       			    self.tool = Tool::Paint;
       			}
       			Event::Key(Key::Char('g')) => {
                    colours.fg = self.data[(self.cur_y - 1) as usize][(self.cur_x - 1) as usize].fg;
                    colours.bg = self.data[(self.cur_y - 1) as usize][(self.cur_x - 1) as usize].bg;
       			}
                Event::Key(Key::Char('h')) => {
                    colours.fg  = self.data[(self.cur_y - 1) as usize][(self.cur_x - 1) as usize].fg;
                    colours.bg  = self.data[(self.cur_y - 1) as usize][(self.cur_x - 1) as usize].bg;
                    chars.pen_char = self.data[(self.cur_y - 1) as usize][(self.cur_x - 1) as usize].ch;
       			}
       			_ => (),
   			}
        }
        if self.tool == Tool::Text && self.tool_down {
            match event.clone() {
                Event::Key(Key::Char('\n')) => (),
			    Event::Key(Key::Char(c)) => {
    				self.data[(self.cur_y - 1) as usize][(self.cur_x - 1) as usize].ch = c;
            		if !colours.fg.default {
            	        self.data[(self.cur_y - 1) as usize][(self.cur_x - 1) as usize].fg = colours.fg;
            	    }
            	    if !colours.bg.default {
              		    self.data[(self.cur_y - 1) as usize][(self.cur_x - 1) as usize].bg = colours.bg;
              		}
                	self.cur_x += 1;
                	self.curr_buffer = parser::construct_buffer(&self.data);
                }
                _ => (),
            }
        } else if self.tool == Tool::Pen && self.tool_down {
            if !colours.fg.default {
                self.data[(self.cur_y - 1) as usize][(self.cur_x - 1) as usize].fg = colours.fg;
            }
            if !colours.bg.default {
                self.data[(self.cur_y - 1) as usize][(self.cur_x - 1) as usize].bg = colours.bg;
            }
            self.data[(self.cur_y - 1) as usize][(self.cur_x - 1) as usize].ch = chars.pen_char;
            self.curr_buffer = parser::construct_buffer(&self.data);
        } else if self.tool == Tool::Paint && self.tool_down {
            if !colours.fg.default {
                self.data[(self.cur_y - 1) as usize][(self.cur_x - 1) as usize].fg = colours.fg;
            }
            if !colours.bg.default {
    		    self.data[(self.cur_y - 1) as usize][(self.cur_x - 1) as usize].bg = colours.bg;
            }
            self.curr_buffer = parser::construct_buffer(&self.data);
        }
    }
}

fn main() {
	let stdout     = stdout().into_raw_mode().unwrap();
	let screen     = termion::screen::AlternateScreen::from(stdout).into_raw_mode().unwrap();
	let mut screen = termion::input::MouseTerminal::from(screen).into_raw_mode().unwrap();
	
    let stdin = stdin();

	write!(screen, "{}{}", termion::cursor::Hide, termion::clear::All).unwrap();

	let mut focus = Focus::Image;

	let mut image   = ImageView::new();
	let mut colours = ColoursView::new();
	let mut chars   = CharView::new();

    for event in stdin.events() {
        let event = event.unwrap();
		match event.clone() {
			Event::Key(Key::Ctrl('c')) => break,
			Event::Key(Key::Ctrl('q')) => break,
			Event::Key(Key::Ctrl('s')) => {
				let mut file = std::fs::File::create("mem1.txt").unwrap();
				file.write_all(image.curr_buffer.replace("\r\n", "\n").as_bytes()).unwrap(); //TODO move to image struct
			},
			Event::Key(Key::Ctrl('e')) => {
			    if !image.tool_down {
    		    	if focus != Focus::Charsheet {
    		    		focus = Focus::Charsheet;
    		    	} else {
    		    		focus = Focus::Image; //TODO ??
    		    	}
		    	}
			},
			Event::Key(Key::Char('\t')) => {
			    if !image.tool_down {
                    if focus == Focus::Image {
                        focus = Focus::Colours;
                    } else if focus == Focus::Colours {
                        focus = Focus::Image;
                    }
                }
			},
			_ => (),
		}

		if focus == Focus::Colours {
		    colours.handle_event(event.clone());
		}

		if focus == Focus::Charsheet {
			chars.handle_event(event.clone());
		}

        if focus == Focus::Image {
            image.handle_event(event.clone(), &mut colours, &mut chars);
        }
		let cur_fg: termion::color::Fg<termion::color::Rgb>;
		let cur_bg: termion::color::Bg<termion::color::Rgb>;
		if (image.tool == Tool::Paint || image.tool == Tool::Pen) && image.tool_down {
			cur_bg = colours.bg.to_bg();
			cur_fg = colours.fg.to_fg();
		} else {
			cur_bg = termion::color::Bg(termion::color::Rgb(0, 0, 0));
			cur_fg = termion::color::Fg(termion::color::Rgb(255, 255, 255));
		}
        
        let colour_select  = colours.draw(image.width + 1, 3);
        let chsheet_render = chars.draw(image.width + 9, 3);

		write!(screen, "{}{}{}{}{}{}{}{}{}{}{}{}{:?}{}{}{}",
			termion::color::Bg(termion::color::Reset),
			termion::clear::All,
		    colour_select,
			termion::cursor::Goto(1, 1),
			image.curr_buffer,
			termion::cursor::Goto(image.cur_x, image.cur_y),
			cur_fg,
			cur_bg,
			image.data[(image.cur_y - 1) as usize][(image.cur_x - 1) as usize].ch,
			termion::cursor::Goto(image.cur_x, image.cur_y),
        	termion::color::Bg(termion::color::Reset),
            termion::cursor::Goto(image.width + 1, 1),
            image.tool,
            termion::cursor::Goto(image.width + 1, 2),
            image.tool_down,
		    chsheet_render,
		).unwrap();
		
		screen.flush().unwrap();
	}
	write!(screen, "{}", termion::cursor::Show).unwrap();
}
