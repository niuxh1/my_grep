pub mod read_config {
    use std::error::Error;
    use std::fs;

    #[derive(Debug)]
    pub struct Config {
        pub search_words: Vec<String>,
        pub file_path: String,
        pub ignore_case: bool,
        pub invert_match: bool,
        pub line_number: bool,
        pub count: bool,
        pub files_with_matches: bool,
        pub before_context: i32,
        pub after_context: i32,
        pub context: i32,
    }

    impl Config {
        fn new(
            search_words: Vec<String>,
            file_path: String,
            ignore_case: bool,
            invert_match: bool,
            line_number: bool,
            count: bool,
            files_with_matches: bool,
            before_context: i32,
            after_context: i32,
            context: i32,
        ) -> Config {
            Config {
                search_words,
                file_path,
                ignore_case,
                invert_match,
                line_number,
                count,
                files_with_matches,
                before_context,
                after_context,
                context,
            }
        }

        pub fn get_args(args: &Vec<String>) -> Result<Config, Box<dyn Error>> {
            if args[0] == "exit" {
                println!("Exiting...");
                std::process::exit(0);
            }
            if args.len() < 2 {
                return Err("Not enough arguments".into());
            }

            let mut search_words = Vec::new();
            let mut file_path = String::new();
            let mut ignore_case = false;
            let mut invert_match = false;
            let mut line_number = false;
            let mut count = false;
            let mut files_with_matches = false;
            let mut before_context = 0;
            let mut after_context = 0;
            let mut context = 0;

            let mut i = 0;
            while i < args.len() {
                if args[i].starts_with('-') {
                    match args[i].as_str() {
                        "-i" | "--ignore-case" => ignore_case = true,
                        "-v" | "--invert-match" => invert_match = true,
                        "-n" | "--line-number" => line_number = true,
                        "-c" | "--count" => count = true,
                        "-l" | "--files-with-matches" => files_with_matches = true,
                        "-B" | "--before-context" => {
                            if i == args.len() - 1 {
                                return Err("No context provided".into());
                            }
                            before_context = args[i + 1].parse().expect("Invalid context");
                            i += 1;
                        }
                        "-A" | "--after-context=" => {
                            if i == args.len() - 1 {
                                return Err("No context provided".into());
                            }
                            after_context = args[i + 1].parse().expect("Invalid context");
                            i += 1;
                        }
                        "-C" | "--context" => {
                            if i == args.len() - 1 {
                                return Err("No context provided".into());
                            }
                            context = args[i + 1].parse().expect("Invalid context");
                            i += 1;
                        }
                        _ => return Err(format!("Unknown option: {}", args[i]).into()),
                    }
                } else {
                    if i == args.len() - 1 {
                        file_path = args[i].clone();
                    } else {
                        search_words.push(args[i].clone().trim_matches('"').to_string());
                    }
                }
                i += 1;
            }

            if search_words.is_empty() {
                return Err("No search words provided".into());
            }

            if file_path.is_empty() {
                return Err("No file path provided".into());
            }

            Ok(Config::new(
                search_words,
                file_path,
                ignore_case,
                invert_match,
                line_number,
                count,
                files_with_matches,
                before_context,
                after_context,
                context,
            ))
        }
        pub fn read_file(&self) -> Result<String, Box<dyn Error>> {
            let context = fs::read_to_string(&self.file_path).expect("No such file");
            Ok(context)
        }
    }
}
