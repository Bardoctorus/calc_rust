use std::env;
fn main() {
    let mut user_input: Vec<String> = env::args().collect();
    println!("Vector as user typed it: {:?}", user_input);
    user_input.remove(0);
    println!("Vector without 0 element: {:?}", user_input);
}



// If I gave it 4+5 * 6
// This would need to happen
//  - get user input
//  - add all to string vector
//  - remove all whitespace
//  - go through each character and determine if it is a number - if so parse it - if not check if
//  it's an operator.
//  - if the next character is also a number, approproately combine them (i.e if the first number
//  is 1 and the second is 2, you'd need to do firstnum*10+second num to get 12)
//  - once you've parsed the entire thing, carry out the arithmatic in the right order
//  - return it to the user
