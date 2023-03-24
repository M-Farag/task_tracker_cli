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


#[cfg(test)]
mod test_helper_module {
    use super::*;

    //tp is Text Processor
    #[test]
    fn test_tp_convert_string_to_usize_works()
    {
       let x:usize =  convert_text_to_usize(&String::from("10")).unwrap();
       assert_eq!(10,x);
    }

    #[test]
    #[should_panic]
    fn test_tp_convert_string_to_usize_can_panic_if_invalid_input()
    {
       convert_text_to_usize(&String::from("abc")).unwrap();
    }

}