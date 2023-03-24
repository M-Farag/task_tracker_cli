#[derive(Debug)]
pub struct Task<'a> {
    name:&'a String,
    duration:usize,
    unit:&'a String
}

impl<'a> Task<'a> {

   pub fn new(name:&'a String, duration:usize, unit:&'a String) -> Self
    {
        Self { name, duration, unit }
    }
    
}