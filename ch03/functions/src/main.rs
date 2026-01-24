fn main() {
    println!("Hello, world!");
    
    //another_fn(4);
    let y = {
        let x = 5;
        x + 1
    };
    
    println!("value is: {y}");
    
    let five = five();
    println!("value of five() is: {five}");
    
    let plus_one = plus_one(4);
    println!("value of plus_one is: {plus_one}");
}

// fn another_fn(x: i32) {
//     println!("another fun {x}");
// }

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}