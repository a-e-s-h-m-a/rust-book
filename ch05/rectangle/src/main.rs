#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    //Associated functions -> called like Rectangle::square(30);
    fn square(size: u32) -> Self {
        Self { width: size, height: size }
    }
    
    fn area(&self) -> u32 {
        self.width * self.height
    }
    
    fn set_width(&mut self, width: u32) {
        self.width = width
    }
    
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let mut rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };
    
    let sq = Rectangle::square(20);
    
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    println!("Area of the square is -> {}", sq.area());
    
    // Same
    let area1 = rect1.area();
    let area2 = Rectangle::area(&rect1);
    assert_eq!(area1, area2);
    
    rect1.set_width(20);
    Rectangle::set_width(&mut rect1, 30);
}
