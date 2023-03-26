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

pub fn validate_arguments_length(args:&Vec<String>, max_length:usize) -> Result<(),&str>
{
    if args.len() < max_length {
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
       let x:usize =  convert_text_to_usize(&String::from("10")).unwrap();
       assert_eq!(10,x);
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
    #[should_panic]
    fn test_helper_validate_arguments_with_exception_when_using_unwrap()
    {
        let args:Vec<String> = vec![String::from("a"),String::from("b"),String::from("c")];
        validate_arguments_length(&args, 4).unwrap();
    }

}