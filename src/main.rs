

fn main() {
   println!("{:?}", formula(12, -5, -2));
}

fn formula(a: i32, b: i32, c: i32) -> (i32, i32) {
    // Get the top number for the diamond
    let ac = a * c;
    let mut mul_num: i32;
    for i in 1..=ac {
        if ac % i == 0 {
            mul_num = ac / i;
            if mul_num + i == b {
               return (mul_num, i); 
            }
        }
    }
    (0, 0)
}


