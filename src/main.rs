// TODO: Add more comments to if the number is negative and the user input section
use std::io;

fn main() {
    let (a, b, c) = ask_for_nums();
    if formula(a, b, c) != (0, 0) {
        println!("{:?}", formula(a, b, c));
    } else {
        println!("This quadratic cannot be factored");
    }
    
}

fn formula(a: i32, b: i32, c: i32) -> (i32, i32) {
    // Get the top number for the diamond
    let ac = a * c;
    // Variable that is the top number divided by the current `i` number
    let mut mul_num: i32;
    // Start at one and check through every number up to the top one to find the multiples
    if ac > 0 {
        for i in 1..=ac {
            // Find even divisibles
            if ac % i == 0 {
                // Top number divided by current `i` number
                mul_num = ac / i;
                // If it matches return the tuple that contains the two numbers
                if mul_num + i == b {
                   return (mul_num, i); 
                }
            }
        }
    } else if ac < 0 {
        for i in ac..=1 {
            if ac % i == 0 {
                mul_num = ac / i;
                if mul_num * i == ac && mul_num + i == b {
                    return (mul_num, i);
                }
            }
        }
    }
    
    // If it doesnt work return a tuple of zeros
    (0, 0)
}


fn ask_for_nums() -> (i32, i32, i32) {
    let mut a = String::new();
    let mut b = String::new();
    let mut c = String::new();
    println!("Enter a: ");
    io::stdin()
        .read_line(&mut a)
        .expect("failed to read the input");
    let a: i32 = a.trim().parse().expect("invalid input");
    println!("Enter b: ");
    io::stdin()
        .read_line(&mut b)
        .expect("failed to read the input");
    let b: i32 = b.trim().parse().expect("invalid input");
    println!("Enter c: ");
    io::stdin()
        .read_line(&mut c)
        .expect("failed to read the input");
    let c: i32 = c.trim().parse().expect("invalid input"); 
    (a, b, c)
}