pub struct Message {
	content: String, 
	user: String,
}

impl Message {
  pub fn new(ms: String, u: String) -> Message {
	  return Message {
		  content: ms,
		  user: u,
	  }

  }
  pub fn send_ms(&self) -> Option<&str> {
	if self.content.is_empty()  || self.content.contains("stupid") {
		return None;
	}

	return Some(self.content.as_str());
  }
}

pub fn check_ms(ms: &Message) -> (bool, &str) {
	let res = ms.send_ms();

	if res.is_none() {
		return(false, "ERROR: illegal");
	}

	return (true, res.unwrap());
}