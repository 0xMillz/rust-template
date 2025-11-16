const CONSTANT: &str  = "this is a constant";

fn main() {
    let y: i32 = 4;
    println!("{}", CONSTANT);
    for i in 0..=y {
        if i != 4 {
            println!("i is {}, ", i);
        } else {
            print!("4 is {}",i);
        }
    }
}
