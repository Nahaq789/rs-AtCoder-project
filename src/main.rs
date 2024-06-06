fn main() {
    proconio::input! {
        a: i32, b: i32, c: i32, x: i32
    }
    let mut count = 0;
    for i in 0..=a {
        for j in 0..=b {
            for k in 0..=c {
                let mut total = i * 500 + j * 100 + k * 50;
                if total == x {
                    count += 1;
                }
            }
        }
    }
    println!("{}", count)
}