mod Brick;

fn main2() {
    proconio::input! {
        n: i32,
        w: i32
    }
    let mut a: i32 = n;
    let mut count: i32 = 0;
    for i in 0..n {
        if a >= w {
            a = a - w;
            count = count + 1;
        }
    }
    println!("{}", count);
}