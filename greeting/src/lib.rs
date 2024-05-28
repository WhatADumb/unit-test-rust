pub fn hello(name: &str) -> String{
    format!("Hello, {}", name)
}

#[cfg(test)]
mod tests{
    use super::hello;

    #[test]
    fn test_hello(){
        let value = String::from("Hello, Fadhil");
        let greet = hello("Fadhil");
        assert_eq!(greet, value);
    }
}