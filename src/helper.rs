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

// Validate the length of the arguments
//
// # Arguments
//      * `args` - A Vec<String> reference
//      * `expected_arguments_length` - A usize
// # Returns
//      * `Result<(),&str>` - A Result with a () and a &str
//         * If the length of the arguments is less than the expected length, then an error is returned
pub fn validate_arguments_length(args:&Vec<String>, expected_arguments_length:usize) -> Result<(),&str>
{
    if args.len() < expected_arguments_length {
       return Err("Missing arguments");
    }
    Ok(())
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

    #[test]
    fn test_helper_validate_arguments_length_works()
    {
        let args:Vec<String> = vec![String::from("a"),String::from("b"),String::from("c")];
        let result = validate_arguments_length(&args, 3);
        assert_eq!(result,Ok(()));
    }

    #[test]
    fn test_helper_validate_arguments_length_fails()
    {
        let args:Vec<String> = vec![String::from("a"),String::from("b"),String::from("c")];
        let result = validate_arguments_length(&args, 4);
        assert_eq!(result,Err("Missing arguments"));
    }

    #[test]
    fn test_helper_validate_arguments_length_fails_with_empty_arguments()
    {
        let args:Vec<String> = vec![];
        let result = validate_arguments_length(&args, 4);
        assert_eq!(result,Err("Missing arguments"));
    }

    #[test]
    #[should_panic(expected = "Missing arguments")]
    fn test_helper_validate_arguments_with_exception_when_using_unwrap()
    {
        let args:Vec<String> = vec![String::from("a"),String::from("b"),String::from("c")];
        validate_arguments_length(&args, 4).unwrap();
    }

}