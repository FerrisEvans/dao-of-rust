fn main() {
    ch01();
}

fn ch01() {
    let x = 1;
    // x = 2; error!
    println!("x: {}", x);
    let mut y = 2;
    println!("y: {}", y);
    y = 3;
    println!("y: {}", y);
}