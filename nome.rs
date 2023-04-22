use std::io;

fn main() io::Result<()>{
    let mut user_input = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut user_input)?;
    println!("Nome: {} ", user_input.trim());
    
    Ok(())
}