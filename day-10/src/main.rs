extern crate utilities;

fn main() {
    let lines = utilities::lines_from_file("input.txt").unwrap();
    let parser = Parser::new();
    let part1: i32 = lines.iter()
        .flat_map(|line| match parser.parse_line(line) {
            ParseResult::Empty => None,
            ParseResult::Success => None,
            ParseResult::IncompleteLine(_) => None,
            ParseResult::Error(x) => Some(x),
        })
        .sum();   
    println!("Part one solution: {}", part1);

    let mut autocomplete_scores: Vec<i64> = lines.iter()
        .flat_map(|line| match parser.parse_line(line) {
            ParseResult::Empty => None,
            ParseResult::Success => None,
            ParseResult::IncompleteLine(x) => Some(x),
            ParseResult::Error(_) => None,
        })
        .collect();    
    autocomplete_scores.sort();
    println!("Part two solution: {:?}", autocomplete_scores[autocomplete_scores.len() / 2]);

}

enum ParseResult {
    Success,
    Error(i32),
    Empty,
    IncompleteLine(i64)
}

struct Token {
    open: char,
    close: char,
    error_value: i32,
    autocomplete_value: i32
}

impl Token {
    fn new(open: char, close: char, error_value: i32, autocomplete_value: i32) -> Self {
        Token{ open, close, error_value, autocomplete_value }
    }    
}

struct Parser {
    tokens: Vec<Token>, 
    open_tokens: Vec<char>,
}

impl Parser {
    fn new() -> Self {
        let tokens = vec![Token::new('(', ')', 3, 1), Token::new('[', ']', 57, 2), Token::new('{', '}', 1197, 3), Token::new('<', '>', 25137, 4)];
        let open_tokens = vec!['(', '[', '{', '<'];
        Parser{ tokens, open_tokens }
    }
    
    fn is_valid_token(&self, open: char, close: char) -> bool {        
        for token in &self.tokens {
            if open == token.open && close == token.close {
                return true;
            }
        }
        return false;
    }

    fn get_error_value(&self, token: char) -> i32 {
        self.tokens.iter()
            .flat_map(|t| if token == t.close { Some(t.error_value) } else { None } )
            .next()
            .unwrap()        
    }

    fn get_autocomplete_value(&self, token: char) -> i32 {
        self.tokens.iter()
            .flat_map(|t| if token == t.close { Some(t.autocomplete_value) } else { None } )
            .next()
            .unwrap()        
    }

    fn autocomplete_line(&self, tokens: &mut Vec<char>) -> i64 {  
        tokens.reverse();      
        tokens.iter()
            .fold(0, |sum, open| sum * 5 + self.get_autocomplete_value(*open) as i64)
    }

    fn parse_line(&self, line: &str) -> ParseResult {        
        let mut tokens: Vec<char> = Vec::new();        
        for (index, c) in line.chars().enumerate() {            
            if self.open_tokens.contains(&c) {
                tokens.push(c);                
            } else {
                match tokens.pop() {
                    Some(t) => if !self.is_valid_token(t, c) { return ParseResult::Error(self.get_error_value(c)) },
                    None => return ParseResult::Empty
                }                
            }
            if index == line.len() - 1 {                
                return ParseResult::IncompleteLine(self.autocomplete_line(&mut tokens));
            }
        }        
        ParseResult::Success
    }
}
