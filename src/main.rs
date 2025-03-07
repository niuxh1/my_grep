use my_grep::{parse_command_line, Config};
use std::io::Write;
fn main() {
    loop{
        print!("my_grep > ");
        std::io::stdout().flush().unwrap();
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let args: Vec<String> = parse_command_line(&input);
        let conf =match Config::get_args(&args){
            Ok(conf) => conf,
            Err(e) => {
                println!("Error: {}", e);
                continue;
            }
        };
        if let Err(e) = my_grep::run(&conf){
            println!("Error: {}", e);
        }
    }
}
