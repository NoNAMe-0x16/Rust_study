use std::io;

fn main() {
    let mut N = String::new();
    io::stdin().read_line(&mut N)
        .expect("error");
    let N: u32 = N.trim().parse()
        .expect("wrong type!");

    for i in 1..N + 1 {
        let mut k = i;
        while k > 0 {
            print!("*");
            k -= 1;
        }
        println!("");
    }
    for i in (1 .. N).rev() {
        let mut k = i;
        while k > 0 {
            print!("*");
            k -= 1;
        }
        println!("");
    }
}
