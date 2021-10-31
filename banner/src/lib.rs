use std::collections::HashMap;
use std::num::ParseFloatError;

#[derive(Debug)]
pub struct Flag {
    pub short_hand:String,
    pub long_hand:String,
    pub desc:String,
}

impl Flag {
    pub fn opt_flag(l_h: &str, d: &str) -> Flag {
	let mut long = "--".to_string(); 
	let mut short = "-".to_string(); 
	short.push(l_h.to_string().chars().nth(0).unwrap());
	long.push_str(l_h);
	return Flag { 
		short_hand: short,
		long_hand: long,
		desc: d.to_string(),
	}
    }
}

pub type Callback = fn(&str, &str) -> Result<String, ParseFloatError>;

pub struct FlagsHandler {
    pub flags: HashMap<(String, String), Callback>,
}

impl FlagsHandler {
    pub fn add_flag(&mut self, flag: (String, String), func: Callback) {
	    self.flags.insert(flag, func);

    }
    pub fn exec_func(&mut self, flag: (String, String), argv: &[&str]) -> String {
	// println!("{:?} {:?}", self.flags.keys(), flag);
	match self.flags[&flag](argv[0], argv[1]) {
		Ok(num) => return num,
	    Err(error) => return error.to_string(),
	};
    }
}

pub fn div(a: &str, b: &str) -> Result<String, ParseFloatError> {
	let mut res: f32;
	match a.parse::<f32>() {
	    Ok(num) => res = num,
	    Err(error) => return Err(error),
	};
	match b.parse::<f32>() {
	    Ok(num) => res = res / num,
	    Err(error) => return Err(error),
	};
	Ok(res.to_string())
}
pub fn rem(a: &str, b: &str) -> Result<String, ParseFloatError> {
	let mut res: f32;
    match a.parse::<f32>() {
        Ok(num) => res = num,
        Err(error) => return Err(error),
    };
    match b.parse::<f32>() {
        Ok(num) => res = res % num,
        Err(error) => return Err(error),
    };
    Ok(res.to_string())
}

