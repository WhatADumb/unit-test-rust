fn main() {
    println!("Hello, world!");
}

#[allow(dead_code)]
fn add(x: i32, y: i32) -> i32{
    x + y
}

#[allow(dead_code)]
fn connect_server(service: &str){
    if service == "localhost" {
        panic!("ENV is PRODUCTION not DEVELOPMENT");
    }else {
        println!("Connected");
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_simple() {
        println!("Hello World");
    }

    #[test]
    fn test_assertion() {
        let is_true: bool = true;
        assert!(is_true, "This status must be true no matter what happen");

        let result = add(5, 5);
        assert_eq!(result, 10, "5 + 5 = 10");
    }

    #[test]
    #[should_panic]
    fn test_panic() {
        connect_server("localhost");
    }

    #[test]
    #[ignore = "Not done yet unit test"]
    fn test_ignore() {
        panic!("PANIC");
    }

    #[test]
    fn test_enum_result() -> Result<(), String> {
        let value = add(10, 10);
        if value == 20 {
            Ok(())
        }else {
            Err(String::from("Value isn't equal to 20"))
        }
    }
}