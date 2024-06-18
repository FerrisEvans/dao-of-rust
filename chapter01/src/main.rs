fn main() {
    greet_world();
}

fn greet_world() {
    let southern_germany = "Grüß Gott!";
    let chinese = "世界，你好";
    let english = "World, hello";

    let regions = [southern_germany, chinese, english];

    // Rust 的集合类型不能直接进行循环，需要变成迭代器才能用于迭代循环。
    // 在2021版之后，可以直接写成 for region in regions 用于循环。因为for隐式地将regions转成迭代器
    for region in regions.iter() {
        println!("{}", &region);
    }
}