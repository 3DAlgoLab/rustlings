#[cfg(test)]
mod test {
    use super::*;
    use my_proc_macro::function_to_string;
    const OUTPUT: &str = "";

    #[function_to_string]
    fn some_function_for_ai_llm(_param: &str) {
        println!("some_function_for_ai_llm");
        println!("{}", OUTPUT);
    }

    #[test]
    fn test_proc_macro() {
        let x = some_function_for_ai_llm("test");
        dbg!(x);
    }
}
