

fn main() {
<<<<<<< HEAD
   println!("{:?}", formula(3, 10, 3));
=======
   println!("{:?}", formula(0, -5, -2)); // Should return (9, 1)
>>>>>>> negatives
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


