use std::{fs, simd::u8x2};

#[derive(Debug)]
enum TokenActions {
    AddCurrent,
    SubstractCurrent,
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,
    SetHeight,
    SetWidth,
    OutputAsNumber,
    OutputAsAscii,
    IfIsThisNumber
}

#[derive(Debug)]
struct Token<'a> {
    name: &'a str,
    uses_prev_num: bool,
    action: TokenActions,
}

impl<'a> Token<'_> {
    fn create(name: &'a str, uses_prev_num: bool, action: TokenActions) -> Token<'a> {
        Token {name, uses_prev_num, action}
    }

    fn do_action(&self, action_number: u8, cell_grid: &mut Vec<u8>) {
        match self.action {
            TokenActions::MoveLeft => {}
            TokenActions::MoveRight => {}
            TokenActions::MoveUp => {}
            TokenActions::MoveDown => {}
            TokenActions::AddCurrent => {println!("{}", action_number)}
            TokenActions::SubstractCurrent => {}
            TokenActions::SetWidth => {}
            TokenActions::SetHeight => {}
            TokenActions::OutputAsNumber => {}
            TokenActions::OutputAsAscii => {}
            TokenActions::IfIsThisNumber => {println!("this function is a complex one, and therefore it has not been implemented yet")}
        }
    }
}

fn main() {
    // < : go one memory cell left
    // > : go one memory cell right
    // ^ : go one memory cell up 
    // v : go one memory cell down
    // + : |add one to the current memory cell, put a number (0-9) before it for faster writing
    // - : |substract one from the current memory cell, put a number (0-9) before it for faster writing
    // x : |set the width of the memory cell grid, has to be at the start (example: set the width to 4: 4x)
    // y : |set the height of the memory cell grid, has to be at the start (example: set the height to 4: 4y)
    // @ : output the current cell value as a number
    // & : output the current cell value as an ascii character
    // ? : |if the current memory cell value equals a number, do the code in the brackets, (example:
    // if the current cell value is equal to 5, print it as an ascii character: 5?[&] )

    let mut cell_position_x : u8 = 0;
    let mut cell_position_y : u8 = 0;
    let mut cell_grid : Vec<u8> = Vec::new();
    let content : String = fs::read_to_string("main.zjn").unwrap();
    let accepted_numbers : Vec<&str> = vec!["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"];
    let tokens : Vec<Token> = vec![
        Token::create("<", false, TokenActions::MoveLeft),
        Token::create(">", false, TokenActions::MoveRight),
        Token::create("^", false, TokenActions::MoveUp),
        Token::create("v", false, TokenActions::MoveDown),
        Token::create("+", true, TokenActions::AddCurrent),
        Token::create("-", true, TokenActions::SubstractCurrent),
        Token::create("x", true, TokenActions::SetWidth),
        Token::create("y", true, TokenActions::SetHeight),
        Token::create("@", false, TokenActions::OutputAsNumber),
        Token::create("&", false, TokenActions::OutputAsAscii),
        Token::create("?", true, TokenActions::IfIsThisNumber),
    ];

    println!("{}", content);

    let dbgmd : bool = false;
    let mut index : usize = 0;
    // TODO: add error handling instead of just unwrapping it and expecting something
    content.as_str().chars().for_each(|token| {
        for real_token in &tokens {
            if token.to_string().as_str() == real_token.name {
                if dbgmd {
                    println!("----------------------------------------");
                    println!("real token: {:#?}", real_token);
                    println!("{:?}", real_token.action);
                }
                // do stuff here because this is after it has been verified as a token, the token will be real_token
                // check if token has extra functions if a number is preceeded
                if real_token.uses_prev_num && accepted_numbers.contains(&content.chars().nth(index-1).unwrap().to_string().as_str()) {
                    if dbgmd {
                        println!("index: {} before index: {}", index, index -1);
                        println!("number {}, index {}, caused by {}, current token {}", content.chars().nth(index-1).unwrap(), index-1, content.chars().nth(index).unwrap(), token);
                        println!("{}", content.chars().nth(index-1).unwrap().to_string().parse::<u8>().unwrap());
                    } else {
                        real_token.do_action(content.chars().nth(index-1).unwrap().to_string().parse::<u8>().unwrap(), &mut cell_grid)
                    }
                } else {
                    real_token.do_action(1, &mut cell_grid)
                }
            }
        }
        index += 1;
    })
}
