// Multiple impl Blocks
// Each struct is allowed to have multiple impl blocks 

#[derive(Debug)] 

struct Rectangle { 
    width: u32, 
    height: u32,
} 

impl Rectangle { 
    fn area(&self) -> u32 { 
        self.width * self.height
    }
} 

impl Rectangle { 
    fn can_hold(&self, other: &Rectangle) -> bool { 
        self.width > other.width && self.height > other.height
    }
} 

pub fn my_struct() { 
    let rect1 = Rectangle { 
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

    println!("Can rect1 hold rect2? {} ", rect1.can_hold(&rect2)); 
    println!("Can rect1 hold rect3? {} ", rect1.can_hold(&rect3));
} 

// There's no reason to separate these methods into multiple blocks here, but this is valid syntax
// 