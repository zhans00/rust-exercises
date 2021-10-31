#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CipherError {
    pub validation: bool,
    pub expected: String
}
impl CipherError {
    pub fn new(validation: bool, expected: String) -> CipherError {
	return CipherError {
		validation: validation,
		expected: expected,
	}
    }
}
pub fn cipher(original: &str, ciphered: &str) -> Option<Result<bool, CipherError>> {
	if original == "" {
		return None;
	}
	let mut coded = original.to_string();
    
	let coded_no_spacing = String::from_utf8(
	    coded
		.bytes()
		.map(|c| {
		    if c >= 'a' as u8 && c <= 'z' as u8{
			122 - c + 97
		    } else if c >= 'A' as u8 && c <= 'Z' as u8{
			90 - c + 65
		    }
		    else {
			c
		    }
		})
		.collect(),
	)
	.unwrap(); 
	// println!("{} {}",coded_no_spacing, ciphered);
	if coded_no_spacing.to_string() == ciphered.to_string() {
		return Some(Ok(true));
	}

	return Some(Err(CipherError::new(false, coded_no_spacing)));
}