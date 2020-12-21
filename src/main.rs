use std::io;

fn main() {
    let mut N = String::new();
    io::stdin().read_line(&mut N)
        .expect("error");
    let mut num:Vec<u32> = N.trim().split(" ")
        .map(|x| {
            x.parse::<u32>()
                .expect("Not an integer!")
        })
        .collect();

    let mut sum:u32 = 0;
    while num[0] != 0 {
        let mut tmp:u32 = num[1];
        let src:u32 = num[0] % 10;
        while tmp != 0 {
            sum = sum + (src * (tmp % 10));
            tmp = tmp / 10;
        }
        num[0] = num[0] / 10;
    }

    print!("{}", sum);
}
