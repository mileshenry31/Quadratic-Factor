

fn main() {
   println!("{:?}", formula(12, -5, -2));
}

fn formula(a: i32, b: i32, c: i32) -> (i32, i32) {
    // Get the top number for the diamond
    let ac = a * c;
    // Variable that is the top number divided by the current `i` number
    let mut mul_num: i32;
    // Start at one and check through every number up to the top one to find the multiples
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
    // If it doesnt work return a tuple of zeros
    (0, 0)
}


