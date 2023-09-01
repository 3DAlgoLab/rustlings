pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fmt;
    use std::io;
    use std::error:Error;

    #[test]
    #[should_panic]
    fn it_works() {
        panic!();
    }

    #[test]
    #[should_panic]
    fn test_unwrap() {
        None::<i32>.unwrap();
    }

    #[test]
    #[should_panic(expected = "Unwrap with a message")]
    fn test_expect() {
        None::<i32>.expect("Unwrap with a message");
    }

    #[derive(Debug)]
    pub struct InvalidDeviceIdError(usize);
    #[derive(Debug)]
    pub struct DeviceNotPresentError(usize);
    #[derive(Debug)]
    pub struct UnexpectedDeviceStateError(usize);
    #[derive(Debug)]
    pub enum ErrorWrapper{
        
    }
}
