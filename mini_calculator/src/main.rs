// Reads text from command line and parses it as a simple mathematical equation.
use std::io;
use std::collections::LinkedList;

fn get_input_from_user() -> String {
    let mut user_input = String::new();

    io::stdin().read_line(&mut user_input).expect("Failed to read line.");

    user_input.trim().to_string()
}

fn is_left_associative(operator: &str) -> bool {
    return operator != "^";
}

fn precedence(operator: &str) -> i32 {
    let mut precedence = 0;
    if operator == "^" {
        precedence = 4;
    } else if operator == "*" || operator == "/" {
        precedence = 3;
    } else if operator == "+" || operator == "-" {
        precedence = 2;
    }

    precedence
}

fn tokenize(equation: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut number = String::new();

    for c in equation.chars() {
        if c.is_ascii_digit() {
            number.push(c);
        } else {
            if !number.is_empty() {
                tokens.push(number.clone());
                number.clear();
            }
            if !c.is_whitespace() {
                tokens.push(c.to_string());
            }
        }
    }

    if !number.is_empty() {
        tokens.push(number);
    }

    tokens
}

fn is_integer(s: &str) -> bool {
    s.parse::<i64>().is_ok()
}

fn main() {
    println!("Enter your equation:");
    let equation = get_input_from_user();
    let tokenized_equation = tokenize(&equation);
    println!("Your equation is: {}", equation);

    let mut stack: Vec<String> = Vec::new();
    let mut queue: LinkedList<String> = LinkedList::new();

    let operators = "+-*^/";
    //let numbers = "1234567890";

    // Implements the shuning yard algorithm.
    for token in tokenized_equation {
        if is_integer(&token) {
            queue.push_back(token.clone());
        }
        if operators.contains(&token) {
            while !stack.is_empty() {
                if let Some(op) = stack.last() {
                    if operators.contains(&*op) && 
                        (precedence(op) > precedence(&token) || precedence(op) == precedence(&token) && is_left_associative(&token)) {
                            if let Some(oper) = stack.pop() {
                                queue.push_back(oper);
                            }
                    } else {
                        break;
                    }
                }
            }
            stack.push(token.clone());
        }
        if token == "(" {
            stack.push(token.clone());
        }

        if token == ")" {
            while let Some(top) = stack.last() {
                if top == "(" {
                    break;
                }
                queue.push_back(stack.pop().unwrap());
            }

            if stack.is_empty() {
                println!("Error: mismatched parentheses");
            } else {
                stack.pop(); // pop '('
            }
        }
    }

    // Put final operators in the stack.
    while !stack.is_empty() {
        let Some(last) = stack.last() else { return };
        if "()".contains(last) {
            println!("Error: mismatched parentheses");
        }
        queue.push_back(stack.pop().unwrap());
    }

    // Process reverse polish notation.
    // tokens contains postfix expression
    let tokens: LinkedList<String> = queue.clone().into_iter().collect();
    for token in tokens {
        if is_integer(&token) {
            stack.push(token.to_string());
        }
        if operators.contains(token.to_string().as_str()) {
            let a: i32 = stack.pop().unwrap().parse().unwrap();
            let b: i32 = stack.pop().unwrap().parse().unwrap();
            // TODO Implement sum of a and b here. Use swich for operators.
            let mut result = 0;
            match token.as_str() {
                "+" => result = a + b,
                "-" => result = a - b,
                "*" => result = a * b,
                "/" => result = a / b,
                _ => (),
            }
            stack.push(result.to_string())
        }
    }
    let final_result = stack.pop().unwrap();
    println!("Your result is {}", final_result);
}
