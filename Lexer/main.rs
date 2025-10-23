
//Programmed By Michael McGivern
use std::env;
use std::fs;
//use std::io;
use std::process;
#[derive(Debug, Clone, PartialEq)]
// suppress cammelCase warnings
#[allow(non_camel_case_types)]
pub enum Token {
    //control flow / grouping
    PARENS_L, PARENS_R, BRACKETS_L, BRACKETS_R, BRACES_L, BRACES_R,
    //seperators
    POINT, COMMA, COLON, SEMICOLON, ARROW_R,
    //arithmetic operators
    ADD, SUB, MUL, DIV,
    //relational ops
    EQ, LT, GT, NEQ, NLT, NGT,
    //no >= or <= ... NGT and NLT are equivalent
    //Logical operators
    NOT, AND, OR,
    //no XOR NAND NOR... (They can be expresed with these ops anyway)
    ASSIGN,
    //keys
    FUNC, LET, IF, ELSE, WHILE, PRINT, RETURN,
    ID(String),
    //Types
    TYPE_INT32, TYPE_FLT32, TYPE_CHAR,
    //no voids or bools...
    LIT_INT32(i32), LIT_FLT32(f32), LIT_CHAR(char), LIT_STRING(String), //will having a lit string by no string type be an issue
    EOI
}

#[derive(Debug, Clone, PartialEq)]
pub enum LexerState {
    // Initial state
    Start,
    InEOI,
    // Identifier and keyword states
    InIdentifier,
    
    // Number literal states
    InIntLit,
    InFltLit,
    
    // String literal state
    InString,
    
    // Character literal state
    InCharLit,

    InFunc,
    InReturn,

    // Multi-character operator states
    InMinus,        // After '-' (could be SUB or ARROW_R if followed by '>')
    InExclamation,  // After '!' (could be NOT, NEQ, NLT, or NGT)
    InLessThan,     // After '<'
    InGreaterThan,  // After '>' could be NGT, GT or Arrow_R
    // need to peek next char and see if it is a keyword or ID
    InCharType,
    InIntType,
    InFltType,

    InLet,
    InIf,
    InElse,
    InWhile,
    InPrint,

}
// suppress cammelCase warnings
#[derive(Debug, Clone, PartialEq)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct Lexer {
    input:String,
    currState:LexerState,
    inputPos:usize,
    currToken:Token,
    buffer:String,
}

impl Lexer {

    pub fn new() -> Self {
        Lexer {
            input: String::new(),
            currState: LexerState::Start,
            inputPos: 0,
            currToken: Token::EOI,
            buffer: String::new(),
        }
    }

    fn set_input(&mut self, input: String) {
        self.input = input;
        self.inputPos = 0;
        self.currState = LexerState::Start;
        self.currToken = Token::EOI;
        (&mut self.buffer).clear();

        //get first token
        self.advance();
    }
    pub fn get_next_token(&mut self) -> Token {
        self.advance()
    }
    fn peek_char(&self) -> Option<char> {
        if self.inputPos < self.input.len() {
            self.input.chars().nth(self.inputPos)
        } else {
            None
        }
    }

    pub fn consume_char(&mut self) {
        if self.inputPos < self.input.len() {
            self.inputPos += 1;
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.peek_char() {
            if ch.is_whitespace() {
                self.consume_char();
            } else {
                break;
            }
        }
    }

    pub fn advance(&mut self) -> Token {
        self.skip_whitespace();

        if self.inputPos >= self.input.len() {
            self.currToken = Token::EOI;
            return Token::EOI;
        }

        self.currState = LexerState::Start;
        self.buffer.clear();

        let mut attempts = 0;
        while attempts < 1000 {
            match self.transition() {
                Some(token) => {
                    self.currToken = token.clone();
                    return token;
                }
                None => {
                    attempts += 1;
                }
            }
        }

        panic!(
            "Lexer stuck in transition loop at position {} with state {:?} and char {:?}",
            self.inputPos,
            self.currState,
            self.peek_char()
        );
    }
    fn match_keyword_or_type(&self) -> Token {
        match self.buffer.as_str() {
            //put the strings of keywords or data types here and map them to tokens
            "func" => Token::FUNC,
            "let" => Token::LET,
            "if" => Token::IF,
            "else" => Token::ELSE,
            "while" => Token::WHILE,
            "print" => Token::PRINT,
            "int32" => Token::TYPE_INT32,
            "flt32" => Token::TYPE_FLT32,
            "char" => Token::TYPE_CHAR,
            _ => Token::ID(String::from(self.buffer.as_str())),
        }
    }

    pub fn transition(&mut self) -> Option<Token> {
        //nested match or if statements
        let ch = self.peek_char();

        match self.currState {
            LexerState::Start => {
                match ch {
                    None => {
                        return Some(Token::EOI);
                    }

                    Some('(') => {
                        self.consume_char();
                        return Some(Token::PARENS_L);
                    }
                    Some(')') =>{
                        self.consume_char();
                        return Some(Token::PARENS_R);
                    }
                    Some('[') => {
                        self.consume_char();
                        return Some(Token::BRACKETS_L);
                    }
                    Some(']') => {
                        self.consume_char();
                        return Some(Token::BRACKETS_R);
                    }

                    Some('{') => {
                        self.consume_char();
                        return Some(Token::BRACES_L);
                    }
                    Some('}') => {
                        self.consume_char();
                        return Some(Token::BRACES_R);
                    }
                    Some('.') => {
                        self.consume_char();
                        return Some(Token::POINT);
                    }
                    Some(',') => {
                        self.consume_char();
                        return Some(Token::COMMA);
                    }
                    Some(':') => {
                        self.consume_char();
                        return Some(Token::COLON);
                    }
                    Some(';') => {
                        self.consume_char();
                        return Some(Token::SEMICOLON);
                    }
                    Some('=') => {
                        self.consume_char();
                        if self.peek_char() == Some('=') {
                            self.consume_char();
                            return Some(Token::EQ);
                        }
                        return Some(Token::ASSIGN);
                    }
                    Some('+') => {
                        self.consume_char();
                        return Some(Token::ADD);
                    }
                    Some('-') => {
                        self.consume_char();
                        self.currState = LexerState::InMinus;
                        return None; // Continue processing
                    }
                    Some('*') => {
                        self.consume_char();
                        return Some(Token::MUL);
                    }
                    Some('/') => {
                        self.consume_char();
                        return Some(Token::DIV);
                    }
                    Some('!') => {
                        self.consume_char();
                        self.currState = LexerState::InExclamation;
                        return None; // Continue processing
                    }
                    Some('<') => {
                        self.consume_char();
                        self.currState = LexerState::InLessThan;
                        return None; // Continue processing
                    }
                    Some('>') => {
                        self.consume_char();
                        self.currState = LexerState::InGreaterThan;
                        return None; // Continue processing
                    }
                    Some('&') => {
                        self.consume_char();
                        if self.peek_char() == Some('&') {
                            self.consume_char();
                            return Some(Token::AND);
                        }
                        else {
                            panic!("Expected '&' after '&'");
                        }
                    }
                    Some('|') => {
                        self.consume_char();
                        if self.peek_char() == Some('|') {
                            self.consume_char();
                            return Some(Token::OR);
                        }
                        else {
                            panic!("Expected '|' after '|'");
                        }
                    }
                    Some('"') => {
                        self.consume_char(); // consume opening quote
                        self.currState = LexerState::InString;
                        return None; // Continue processing
                    }
                    Some('\'') => {
                        self.consume_char(); // consume opening quote
                        self.currState = LexerState::InCharLit;
                        return None; // Continue processing
                    }
                    Some(c) if c.is_ascii_digit() => {
                        self.buffer.push(c);
                        self.consume_char();
                        self.currState = LexerState::InIntLit;
                        return None; // Continue processing
                    }
                    Some(c) if c.is_ascii_alphabetic() || c == '_' => {
                        self.buffer.push(c);
                        self.consume_char();
                        self.currState = LexerState::InIdentifier;
                        return None; // Continue processing
                    }
                    Some(c) => {
                        panic!("Unexpected character: '{}'", c);
                    }
                }
            }
            
            LexerState::InIdentifier => {
                match ch {
                    Some(c) if c.is_ascii_alphanumeric() || c == '_' => {
                        // Continue building identifier
                        self.buffer.push(c);
                        self.consume_char();
                        return None; // Keep processing
                    }
                    _ => {
                        // End of identifier - use helper to check if keyword/type
                        let token = self.match_keyword_or_type();
                        return Some(token);
                    }
                }
            }

            LexerState::InIntLit => {
                match ch {
                    Some(c) if c.is_ascii_digit() => {
                        self.buffer.push(c);
                        self.consume_char();
                        return None; // Keep processing
                    }
                    Some('.') => {
                        self.buffer.push('.');
                        self.consume_char();
                        self.currState = LexerState::InFltLit;
                        return None; // Keep processing
                    }
                    _ => {
                        // End of integer
                        let value = self.buffer.parse::<i32>().unwrap_or(0);
                        return Some(Token::LIT_INT32(value));
                    }
                }
            }

            LexerState::InFltLit => {
                match ch {
                    Some(c) if c.is_ascii_digit() => {
                        self.buffer.push(c);
                        self.consume_char();
                        return None; // Keep processing
                    }
                    _ => {
                        // End of float
                        let value = self.buffer.parse::<f32>().unwrap_or(0.0);
                        return Some(Token::LIT_FLT32(value));
                    }
                }
            }

            LexerState::InString => {
                match ch {
                    Some('"') => {
                        self.consume_char();
                        return Some(Token::LIT_STRING(self.buffer.clone()));
                    }
                    Some('\\') => {
                        self.consume_char();
                        if let Some(escaped) = self.peek_char() {
                            self.consume_char();
                            let escaped_char = match escaped {
                                'n' => '\n',
                                't' => '\t',
                                '\\' => '\\',
                                '"' => '"',
                                _ => escaped,
                            };
                            self.buffer.push(escaped_char);
                        }
                        return None;
                    }
                    Some(c) => {
                        self.buffer.push(c);
                        self.consume_char();
                        return None;
                    }
                    None => {
                        panic!(
                            "Unterminated string literal starting at position {}",
                            self.inputPos - self.buffer.len()
                        );
                    }
                }
            }

            LexerState::InCharLit => {
                match ch {
                    Some('\\') => {
                        self.consume_char();
                        if let Some(escaped) = self.peek_char() {
                            self.consume_char();
                            let escaped_char = match escaped {
                                'n' => '\n',
                                't' => '\t',
                                '\\' => '\\',
                                '\'' => '\'',
                                _ => escaped,
                            };
                            self.buffer.push(escaped_char);
                        }
                        return None; // Keep processing
                    }
                    Some(c) if c != '\'' => {
                        self.buffer.push(c);
                        self.consume_char();
                        // Expect closing quote
                        if let Some('\'') = self.peek_char() {
                            self.consume_char();
                            let char_val = self.buffer.chars().next().unwrap_or('\0');
                            return Some(Token::LIT_CHAR(char_val));
                        } else {
                            panic!("Character literal must contain exactly one character");
                        }
                    }
                    _ => {
                        panic!("Invalid character literal");
                    }
                }
            }

            LexerState::InMinus => {
                match ch {
                    Some('>') => {
                        self.consume_char();
                        return Some(Token::ARROW_R);
                    }
                    _ => {
                        // Don't consume next char, just return SUB
                        return Some(Token::SUB);
                    }
                }
            }

            LexerState::InExclamation => {
                match ch {
                    Some('=') => {
                        self.consume_char();
                        return Some(Token::NEQ);
                    }
                    Some('<') => {
                        self.consume_char();
                        return Some(Token::NLT);
                    }
                    Some('>') => {
                        self.consume_char();
                        return Some(Token::NGT);
                    }
                    _ => {
                        // Don't consume next char, just return NOT
                        return Some(Token::NOT);
                    }
                }
            }

            LexerState::InLessThan => {
                match ch {
                    Some('=') => {
                        self.consume_char();
                        return Some(Token::EQ);
                    }
                    _ => {
                        // Don't consume next char, just return LT
                        return Some(Token::LT);
                    }
                }
            }

            LexerState::InGreaterThan => {
                // GT is always just '>'
                return Some(Token::GT);
            }

            // These states are not needed - handled by InIdentifier + match_keyword_or_type()
            LexerState::InFunc |
            LexerState::InReturn |
            LexerState::InLet |
            LexerState::InIf |
            LexerState::InElse |
            LexerState::InWhile |
            LexerState::InPrint |
            LexerState::InCharType |
            LexerState::InIntType |
            LexerState::InFltType => {
                // These should never be reached because keywords/types
                // are handled in InIdentifier state
                panic!("Invalid state: {:?} - keywords should be handled in InIdentifier", self.currState);
            }

            LexerState::InEOI => {
                return Some(Token::EOI);
            }
        }
    }
    pub fn curr(&self) -> Token {
        //returns current token
        self.currToken.clone()
    }
    fn print_tokens(&mut self) {
        loop {
            let token = self.curr();
            println!("{:?}", token);

            if token == Token::EOI {
                break;
            }

            self.advance();
        }
    }

    pub fn expect(&mut self,expected:Token) {
        if !self.token_matches(&self.curr(), &expected) { // if curr doesnt return a token
            panic!("token not found");
        }
        self.advance();
    }

    pub fn accept(&mut self, expected: Token) -> bool {
        if self.token_matches(&self.curr(), &expected) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn token_matches(&self, token1: &Token, token2: &Token) -> bool {
        match (token1, token2) {
            (Token::ID(_), Token::ID(_)) => true,
            (Token::LIT_INT32(_), Token::LIT_INT32(_)) => true,
            (Token::LIT_FLT32(_), Token::LIT_FLT32(_)) => true,
            (Token::LIT_CHAR(_), Token::LIT_CHAR(_)) => true,
            (Token::LIT_STRING(_), Token::LIT_STRING(_)) => true,
            _ => token1 == token2,
        }
    }



}
fn main() {
    let args: Vec<String> = env::args().collect();
    //creates a vector of strings from the arguments given at the command line

    if args.len() < 2 { //checks if arguments are given
        print_general_help();
        process::exit(0);
    }

    let command = &args[1];

    match command.as_str() {
        "help" => {
            if args.len() > 2 { //asking for help for certain command
                print_command_help(&args[2]);
            }
            else { //wants general help
                print_general_help();

            }
        }
        "print" => {
            handle_print_command(&args[2..])
            //prints arguments given from index 2 upto length of arguments
        }
        "list" => {
            list_commands();
        }
        "listTokens" => {
            list_tokens();
        }
        "tokenize" => {
            handle_tokenize_command(&args[2..]);
        }
        _ => { //default case of switch statement
            println!("Unknown command: {}", command);
            println!("Try 'help' for a list of commands.");
            process::exit(0);
        }
    }
}
pub fn list_tokens() {
    use Token::*;
    let tokens = vec![
        PARENS_L, PARENS_R, BRACKETS_L, BRACKETS_R, BRACES_L, BRACES_R,
        POINT, COMMA, COLON, SEMICOLON, ARROW_R,
        ADD, SUB, MUL, DIV,
        EQ, LT, GT, NEQ, NLT, NGT,
        NOT, AND, OR,
        ASSIGN,
        FUNC, LET, IF, ELSE, WHILE, PRINT, RETURN,
        ID("example_id".to_string()),
        TYPE_INT32, TYPE_FLT32, TYPE_CHAR,
        LIT_INT32(42), LIT_FLT32(3.14), LIT_CHAR('x'), LIT_STRING("hello".to_string()),
        EOI,
    ];

    for token in tokens {
        println!("{:?}", token);
    }
}

fn print_general_help() {
    println!("A command line utility for Rust");
    println!("Usage: cargo run -- <command> [arguments]");
    println!();
    println!("Commands:");
    println!("    help        Print this help message");
    println!("    help [command]       shows help information for a command");
    println!("    print <file> [numbered]       Print arguments given");
    println!("    list        List all commands");
    println!("    tokenize <file>       Lexically analyze a file");
    println!("listTokens    List all tokens")
}

fn print_command_help(command:&str) {
    match command {
        "help" => {
            println!("help - Show help information");
            println!();
            println!("Usage: ");
            println!("cargo run -- help [command]");
            println!();
            println!("Description: ");
            println!("    Prints help information for a command");
            println!("Arguments: ");
            println!("    [command] - The command to get help for (OPTIONAL)");
        }

        "print" => {
            println!("print file contents");
            println!();
            println!("Usage: ");
            println!("cargo run -- print <file> [numbered]");
            println!();
            println!("Description: ");
            println!("prints the contents of the specified file");
            println!("Arguments: ");
            println!("    <file> - The path of the file to print (REQUIRED)");
            println!("    [numbered] - Whether to number the lines (OPTIONAL)");
        }

        "list" => {
            println!("list - List all commands");
            println!();
            println!("Usage: ");
            println!("cargo run -- list");
            println!();
            println!("Description: ");
            println!("lists all commands");
        }

        "tokenize" => {
            println!("tokenize - Lexically analyze a file");
            println!();
            println!("Usage:");
            println!("cargo run -- tokenize <file>");
            println!();
            println!("Description:");
            println!("Reads the specified file and prints each token using the custom lexer.");
        }

        _ => { //default case of switch statement
            println!("Unknown command: {}", command);
            println!("Try 'help' to learn how to use this tool or list for a list of commands.");
            process::exit(1);
        }
    }
}

fn handle_print_command(args: &[String]) {
    if args.is_empty() {
        println!("No file specified");
        println!("USEAGE: cargo run -- print <file> [--numbered]");
        println!("Try 'help print' for more information");
        process::exit(0);
    }

    let file_path:&String = &args[0];
    let numbered:bool = args.len() > 1 && args[1] == "--numbered";

    match fs::read_to_string(file_path) {
        Ok(contents)  => {
            if numbered {
                for(line_number, line) in contents.lines().enumerate() {
                    println!("{:5} {}", line_number + 1, line);
                }
            } else {
                println!("{}", contents);
            }
        }
        Err(error) => {
            println!("Error reading file {}: {}", file_path, error);
            process::exit(0);
        }
    }
}
fn handle_tokenize_command(args: &[String]) {
    if args.is_empty() {
        println!("No file specified");
        println!("USAGE: cargo run -- tokenize <file>");
        println!("Try 'help tokenize' for more information");
        process::exit(0);
    }

    let file_path = &args[0];

    match fs::read_to_string(file_path) {
        Ok(contents) => {
            let mut lexer = Lexer::new();
            lexer.set_input(contents);
            lexer.print_tokens();
        }
        Err(error) => {
            println!("Error reading file {}: {}", file_path, error);
            process::exit(1);
        }
    }
}
fn list_commands() {
    println!("Available commands:");
    println!("list - List all commands");
    println!("help - Show help information");
    println!("print - Print arguments given");
    println!("listTokens - List all tokens");
    println!("tokenize - Lexically analyze a file");
}
//paste in commandline tool here and add in the functions of the lexer to the tool
