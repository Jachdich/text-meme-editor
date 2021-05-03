use crate::rgb::RGB;
use crate::format_char::FmtChar;

fn read_until(ch: char, data: &Vec<char>, mut pos: usize) -> (Vec<char>, usize) {
    if pos >= data.len() - 1 {
        return (vec![], pos);
    }
    let start = pos;
    while pos < data.len() && data[pos] != ch {
    	pos += 1;
    }
    if pos >= data.len() - 1 {
        return (data[start..pos].to_vec(), pos);
    } 
    (data[start..pos].to_vec(), pos)
}

pub fn make_data(data: Vec<char>) -> Vec<Vec<FmtChar>> {
    let mut out: Vec<FmtChar> = Vec::new();
    let mut pos = 0;
    let mut bg = RGB::new(0, 0, 0);
    let mut fg = RGB::new(255, 255, 255);

    while pos < data.len() {
        if data[pos] == '\u{001b}' {
            pos += 2;
            if data[pos..pos + 2].to_vec().into_iter().collect::<String>() == "0m" {
                bg = RGB::new(0, 0, 0);
                fg = RGB::new(255, 255, 255);
                pos += 2;
                continue;
            }
            if data[pos..pos + 3].to_vec().into_iter().collect::<String>() == "39m" {
                bg = RGB::new(0, 0, 0);
                fg = RGB::new(255, 255, 255);
                pos += 3;
                continue;
            }
            if data[pos..pos + 3].to_vec().into_iter().collect::<String>() == "49m" {
                bg = RGB::new(0, 0, 0);
                fg = RGB::new(255, 255, 255);
                pos += 3;
                continue;
            }
            let (first_num,   npos) = read_until(';', &data, pos); pos = npos;
            let (_second_num, npos) = read_until(';', &data, pos + 1); pos = npos;

            let (rv, npos) = read_until(';', &data, pos + 1); pos = npos;
            let (gv, npos) = read_until(';', &data, pos + 1); pos = npos;
            let (bv, npos) = read_until('m', &data, pos + 1); pos = npos;
            let r = rv.into_iter().collect::<String>().parse::<u8>().unwrap();
            let g = gv.into_iter().collect::<String>().parse::<u8>().unwrap();
            let b = bv.into_iter().collect::<String>().parse::<u8>().unwrap();
            pos += 1;
            let s: String = first_num.into_iter().collect();
            if s == "38" {
                fg = RGB::new(r, g, b);
            } else if s == "48" {
                bg = RGB::new(r, g, b);
            }
        } else {
            out.push(FmtChar{ch: data[pos], fg: fg, bg: bg});
            pos += 1;
        }
    }
    let mut n: Vec<Vec<FmtChar>> = Vec::new();
    n.push(Vec::new());
    for ch in out {
    	if ch.ch == '\n' {
    		n.push(Vec::new());
    	} else {
    		n.last_mut().unwrap().push(ch);
    	}
    }
    if n.last().unwrap().len() == 0 {
    	n.pop();
    }
    return n;
}

pub fn construct_buffer(data: &Vec<Vec<FmtChar>>) -> String {
	let mut buffer = "".to_string();
	let mut last_fg = RGB::new(0, 0, 0);
	let mut last_bg = RGB::new(0, 0, 0);
	for e in data {
		for ch in e {
		    if last_fg != ch.fg {
			    buffer.push_str(&termion::color::Fg(termion::color::Rgb(ch.fg.r, ch.fg.g, ch.fg.b)).to_string());
			    last_fg = ch.fg;
			}
			if last_bg != ch.bg {
			    buffer.push_str(&termion::color::Bg(termion::color::Rgb(ch.bg.r, ch.bg.g, ch.bg.b)).to_string());
			    last_bg = ch.bg;
			}
			buffer.push(ch.ch);
		}
		buffer.push_str(&format!("{}{}\r\n", termion::color::Fg(termion::color::Reset), termion::color::Bg(termion::color::Reset)));
	}
	buffer
}

pub fn make_char_sheet(txt: String) -> Vec<Vec<char>> {
	let mut ret: Vec<Vec<char>> = Vec::new();
	let arr_1d: Vec<char> = txt.chars().collect::<Vec<char>>();
	ret.push(Vec::new());
	for ch in arr_1d {
		if ch == '\n' {
			ret.push(Vec::new());
		} else {
			ret.last_mut().unwrap().push(ch);
		}
	}
	while ret.len() > 0 && ret.last().unwrap().len() == 0 {
		ret.pop();
	}
	ret
}
