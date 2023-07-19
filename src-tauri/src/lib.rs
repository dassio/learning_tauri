pub fn greet(name: &str) -> String {
    if name.len() == 0 || name.trim().len() == 0{
        return "please input your name".to_string();
    }
   format!("Hello, {}!", name)
}