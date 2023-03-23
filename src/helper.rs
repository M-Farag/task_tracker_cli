use std::num::ParseIntError;
/// Convert String references to usize
/// 
/// # Arguments
/// 
/// * `text` - A String reference that holds the text to be parsed
/// 
/// # Returns
/// * `Result<usize, &str
pub fn convert_text_to_usize(text:&String) -> Result<usize,ParseIntError>
{
    let number:usize = text.parse()?;
    Ok(number)
}