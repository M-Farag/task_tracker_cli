
pub mod helper;

#[cfg(test)]
mod tests {
    use super::*;

    //tp is Text Processor
    #[test]
    fn test_tp_convert_string_to_usize_works()
    {
       let x:usize =  helper::convert_text_to_integer(&String::from("10")).unwrap();
       assert_eq!(10,x);
    }

    #[test]
    #[should_panic]
    fn test_tp_convert_string_to_usize_can_panic_if_invalid_input()
    {
       helper::convert_text_to_integer(&String::from("abc")).unwrap();
    }

}
