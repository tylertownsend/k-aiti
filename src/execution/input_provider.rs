use std::error::Error;
use std::io::Write;

pub async fn get_user_input() -> Result<String, Box<dyn Error>> {
    std::io::stdout().flush().unwrap();
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    Ok(input.trim().to_string())
}