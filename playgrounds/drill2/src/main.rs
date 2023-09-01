use std::string::FromUtf8Error;

fn str_upper_match(str: Vec<u8>) -> Result<String, FromUtf8Error> {
    let ret = match String::from_utf8(str) {
        Ok(s) => s.to_uppercase(),
        Err(e) => return Err(e),
    };
    println!("ret:{}", ret);
    Ok(ret)
}

fn str_upper_concise(str: Vec<u8>) -> Result<String, FromUtf8Error> {
    let ret = String::from_utf8(str)?.to_uppercase();
    println!("ret:{}", ret);
    Ok(ret)
}

fn main() {
    let invalid_str = str_upper_match(vec![197, 198]);
    println!("invalid_str:{:?}", invalid_str);

    let valid_str = str_upper_concise(vec![121, 97, 89]);
    println!("valid_str:{:?}", valid_str);
}
