use std::num::ParseIntError;
// Helper functions

// Convert a String reference to a usize
//
// # Arguments
//      * `text` - A String reference
// # Returns
//      * `Result<usize,ParseIntError>` - A Result with a usize and a ParseIntError
//         * If the text cannot be converted to a usize, then an error is returned
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
       let result:Result<usize, ParseIntError> =  convert_text_to_usize(&String::from("10"));
       assert_eq!(Ok(10),result);
    }

    #[test]
    #[should_panic]
    fn test_tp_convert_string_to_usize_can_panic_if_invalid_input()
    {
       convert_text_to_usize(&String::from("abc")).unwrap();
    }

}