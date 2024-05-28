#[test]
fn test_hello(){
    let value = String::from("Hello, Fadhil");
    let greet = greeting::hello("Fadhil");
    assert_eq!(greet, value);
}