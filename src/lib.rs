//TODO:
//Create stack data structure
//Infix -> postfix translator
//Check if valid input string
use std::collections::HashMap;

struct Stack<T> {
    stack: Vec<T>,
}

impl<T> Stack<T> {
    fn new() -> Self {
        Stack { stack: Vec::new() }
    }

    fn pop(&mut self) -> Option<T> {
        self.stack.pop()
    }

    fn push(&mut self, item: T) {
        self.stack.push(item)
    }

    fn is_empty(&self) -> bool {
        self.stack.is_empty()
    }

    fn peek(&self) -> Option<&T> {
        self.stack.last()
    }
}

pub fn translate_infix_to_postfix(input: String) -> String {
    let priority: HashMap<char, u32> = HashMap::from([
        ('+', 50),
        ('-', 50),
        ('*', 100),
        ('/', 100),
        ('^', 150),
        ('(', 0),
    ]);

    let mut stack: Stack<String> = Stack::new();
    let mut result: String = String::new();

    let mut iterator = input.chars();
    let mut input_next = iterator.next();

    let mut current_number: String = String::new();

    while input_next.is_some() {
        match input_next.unwrap() {
            '0'..='9' | '.' => current_number.push(input_next.unwrap()),
            '+' | '-' | '*' | '/' | '^' | '(' => {
                if !current_number.is_empty() {
                    result.push_str(&current_number.as_str());
                    result.push_str(" ");
                    current_number = String::new();
                } else {
                    panic!("A number is missing before the operator!");
                };

                if stack.is_empty() {
                    stack.push(input_next.unwrap().to_string());
                } else if priority.get(&input_next.unwrap())
                    > priority.get(&stack.peek().unwrap().chars().nth(0).unwrap())
                {
                    stack.push(input_next.unwrap().to_string());
                } else if priority.get(&input_next.unwrap())
                    <= priority.get(&stack.peek().unwrap().chars().nth(0).unwrap())
                {
                    while priority.get(&stack.peek().unwrap().chars().nth(0).unwrap())
                        >= priority.get(&input_next.unwrap())
                    {
                        result.push_str(&stack.pop().unwrap());
                    }

                    stack.push(input_next.unwrap().to_string());
                };
            }
            ')' => {
                while stack.peek().unwrap() != &String::from('(') {
                    result.push_str(&stack.pop().unwrap());
                }
                stack.pop();
            }
            _ => {}
        };

        input_next = iterator.next();
    }
    while !stack.is_empty() {
        result.push_str(&stack.pop().unwrap());
    }

    result
}
