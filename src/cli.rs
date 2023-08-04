

pub mod console {
    use std::io::{self, Write};
 
    pub struct Input {
        pub number: Option<i32>,
        pub string: Option<String>
    }

    pub fn get_input(message: &str) -> Input {
        let text_input = get_str(message);
        Input{
            string: Some(text_input.clone()),
            number: match text_input.trim().parse::<i32>() {
                Ok(num) => Some(num),
                Err(_) => None
            }
        }
    }

    pub fn get_text(message: &str) -> String {
        String::from(get_str(message).trim())
    }

    pub fn get_number(message: &str) -> i32 {
        match get_str(message).trim().parse::<i32>() {
            Ok(inp) => inp,
            Err(_) => panic!("couldn't read number from terminal :/")
        }
    }

    fn get_str(message: &str) -> String {
        print!("{}", message);
        let _ = io::stdout().flush();
        let mut input: String = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("failed to get input");
        input
    }

    pub fn yes_no(message: &str) -> bool {
        let response = get_str(message);
        response.contains("y") || response.trim().is_empty()
    }

    pub fn endl(n: i32) {
        for _ in 0..n {
            print!("\n"); 
        }
    } 

    pub fn clear() {
        print!("\x1B[2J\x1B[1;1H");
    }
}