use std::error::Error;
pub fn really_complicated_code_b(a: u8, b: u8) -> Result<u8, Box<dyn Error>> {
    Ok(a + b)
}