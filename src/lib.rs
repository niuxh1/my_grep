mod read_config;
pub use read_config::read_config::Config;
use std::error::Error;

pub fn run(conf: &Config) -> Result<(), Box<dyn Error>> {
    let content = conf.read_file()?;
    let lines: Vec<&str> = content.lines().collect();
    let mut matches_count = 0;
    for (i, line) in lines.iter().enumerate() {
        let mut is_match = false;
        for word in &conf.search_words {
            if conf.ignore_case {
                if line.to_lowercase().contains(&word.to_lowercase()) {
                    is_match = true;
                    break;
                }
            } else {
                if line.contains(word) {
                    is_match = true;
                    break;
                }
            }
        }
        if conf.before_context > 0 && is_match {
            let check = i as i32 - conf.before_context;
            if check < 0 {
                return Err(format!(
                    "Index out of bounds,because don't have before {} context  ",
                    check
                )
                .into());
            }

            println!("i: {}", i as i32 - conf.before_context);

            for j in i as i32 - conf.before_context..i as i32 {
                if j < 0 {
                    return Err(format!(
                        "Index out of bounds,because don't have before {} context  ",
                        j
                    )
                    .into());
                } else {
                    println!("{}", lines[j as usize]);
                }
            }
        }
        if conf.after_context > 0 && is_match {
            let check = i as i32 + conf.before_context + 1;
            if check >= lines.len() as i32 {
                return Err(format!(
                    "Index out of bounds,because don't have before {} context  ",
                    check
                )
                .into());
            }
            for j in i as i32 + 1..i as i32 + conf.after_context + 1 {
                if j >= lines.len() as i32 {
                    return Err(format!(
                        "Index out of bounds,because don't have after {} context  ",
                        j
                    )
                    .into());
                }

                println!("{}", lines[j as usize]);
            }
        }
        if conf.context > 0 && is_match {
            let check1 = i as i32 - conf.context;
            let check2 = i as i32 + conf.context + 1;
            if check1 < 0 {
                return Err(format!(
                    "Index out of bounds,because don't have before {} context  ",
                    check1
                )
                .into());
            }
            if check2 >= lines.len() as i32 {
                return Err(format!(
                    "Index out of bounds,because don't have after {} context  ",
                    check2
                )
                .into());
            }
            for j in i as i32 - conf.context..i as i32 + conf.context + 1 {
                println!("{}", lines[j as usize]);
            }
        }
        if conf.invert_match {
            is_match = !is_match;
        }

        if is_match {
            matches_count += 1;

            if conf.files_with_matches {
                println!("{}", conf.file_path);
                return Ok(());
            }

            if conf.count {
                continue;
            }

            if conf.line_number {
                print!("{}: ", i + 1);
            }

            println!("{}", line);
        }
    }

    if conf.count {
        println!("{}", matches_count);
    }

    Ok(())
}

pub fn parse_command_line(input: &str) -> Vec<String> {
    let mut args = Vec::new();
    let mut current_arg = String::new();
    let mut in_quotes = false;
    let mut escaped = false;
    
    for c in input.chars() {
        if escaped {
            current_arg.push(c);
            escaped = false;
        } else if c == '\\' {
            escaped = true;
        } else if c == '"' {
            in_quotes = !in_quotes;
        } else if c.is_whitespace() && !in_quotes {
            if !current_arg.is_empty() {
                args.push(current_arg);
                current_arg = String::new();
            }
        } else {
            current_arg.push(c);
        }
    }
    
    // 确保最后一个参数也被添加
    if !current_arg.is_empty() {
        args.push(current_arg);
    }
    
    args
}