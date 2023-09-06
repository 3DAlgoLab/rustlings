#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_struct() {
        let s = create_struct!(MyStruct);
        dbg!(s);
    }
}
