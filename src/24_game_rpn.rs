// Implements http://rosettacode.org/wiki/24_game
// Uses RPN expression 

#[cfg(not(test))]
fn main() {
    use std::rand::{task_rng, Rng};
    use std::io;

    let mut rng = task_rng();
    let mut reader = io::stdin();

    // generating 4 numbers
    let choices = Vec::from_fn(5, |_| rng.gen_range(1u, 10));
    println!("Make 24 with the following numbers");
    
    // start the game loop
    loop {
        print!("Your numbers: {}, {}, {}, {}\n", choices[0], choices[1], choices[2], choices[3]);
        let expr = reader.read_line().ok().expect("Failed to read line!");
        match check_input(expr, &choices) {
            Ok(()) => { println!("Good job!"); break; },
            Err(e) => println!("{}", e)
        }
        print!("Try again? (y/n): ");
        let choice = reader.read_line().ok().expect("Failed to read line!");
        if choice.as_slice().trim() != "y" { break; }
    }
}

fn check_input(expr: String, choices: &Vec<uint>) -> Result<(), String> {
    let mut stack: Vec<uint> = Vec::new();
    for token in expr.as_slice().words() {
        if is_operator(&token) {
            let (a, b) = (stack.pop(), stack.pop());
            match (a, b) {
                (Some(x), Some(y)) => stack.push(evaluate(y, x, token)),
                (_, _) => return Err("Not a valid RPN expression!".to_string())
            }
        } else {
            let v: Option<uint> = from_str(token);
            match v {
                Some(n) => {
                    // check if the number is valid
                    if !choices.contains(&n) {
                        return Err(format!("Cannot use {}", n));
                    }
                    stack.push(n) 
                },
                None => return Err(format!("Invalid input: {}", token))
            }
        }
    }

    let ans = stack.pop();
    if stack.len() > 0 {
        return Err("Not a valid RPN expression!".to_string());
    }
    match ans {
        Some(x) => {
            if x == 24 { return Ok(()); }
            return Err(format!("Wrong answer. Result: {}", x));
        }
        None => return Err("Error encountered!".to_string()),
    }
}

// since evaluate is wrapped in is_operator the last
// pattern has to be "/"
fn evaluate(a: uint, b: uint, op: &str) -> uint {
    match op {
        "+" => a + b,
        "-" => a - b,
        "*" => a * b,
        "/" => a / b,
        _   => unreachable!()
    }
}

fn is_operator(op: &&str) -> bool {
    ["*", "-", "+", "/"].contains(op)
}

#[test]
fn test_check_input() {
    let v1: Vec<uint> = vec![4, 3, 6, 2];

    // correct result
    match check_input("4 3 * 6 2 * +".to_string(), &v1) {
        Ok(()) => assert!(true),
        Err(_) => assert!(false)
    }

    // incorrect result
    match check_input("4 3 * 2 6 + -".to_string(), &v1) {
        Ok(()) => assert!(false),
        Err(e) => assert_eq!(e, "Wrong answer. Result: 4".to_string())
    }
    
    // wrong numbers in input
    match check_input("4 5 + 6 2 * -".to_string(), &v1) {
        Ok(()) => assert!(false),
        Err(e) => assert_eq!(e, "Cannot use 5".to_string())
    }

    // invalid chars in input
    match check_input("4 ) + _ 2 * -".to_string(), &v1) {
        Ok(()) => assert!(false),
        Err(e) => assert_eq!(e, "Invalid input: )".to_string())
    }

    // invalid RPN expression
    match check_input("4 3 + 6 2 *".to_string(), &v1) {
        Ok(()) => assert!(false),
        Err(e) => assert_eq!(e, "Not a valid RPN expression!".to_string())
    }
}
