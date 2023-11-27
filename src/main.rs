use std::fmt;
use std::{io};
use std::f32::consts::PI; //for using PI

#[derive(Debug)]
struct Rectangle {
  width: u32,
  height: u32,
}

struct Triangle {
  width: u32,
  height: u32,
}

struct Circle {
  radius: u32,
}

//Method
impl Rectangle {
  fn area(&self) -> f32 {
    let width = self.width as f32;
    let height = self.height as f32;
    width * height
  }

  fn new(width: u32, height: u32) -> Rectangle {
    Rectangle {
      width,
      height,
    }
  }
}

impl Triangle {
  fn area(&self) -> f32 {
    let width = self.width as f32;
    let height = self.height as f32;
    width * height / 2.0
  }

  fn new(width: u32, height: u32) -> Triangle {
    Triangle {
      width,
      height,
    }
  }
}

impl Circle {
  fn area(&self) -> f32 {
    let radius = self.radius as f32;
    radius * radius * PI
  }

  fn new(radius: u32) -> Circle {
    Circle {
      radius,
    }
  }
}

//Related Functions
impl Rectangle {
  fn show(&self) {
    println!("{} and {} with area: {}", self.width, self.height, self.area());
  }
}

impl Triangle {
  fn show(&self) {
    println!("{} and {} with area: {}", self.width, self.height, self);
  }
}

impl Circle {
    fn show(&self) {
        println!("{} radius area: {}", self.radius, self.area());
    }
}

//Debug formatting
impl fmt::Display for Rectangle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "width and height : ({}, {})", self.width, self.height)
    }
}

impl fmt::Display for Triangle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "width and height : ({}, {})", self.width, self.height)
    }
}

impl fmt::Display for Circle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "radius : ({})", self.radius)
    }
}

fn main() {
    println!("Type any number.");
    println!("1.Rectangle, 2.Triangle, 3.Circle"); //selct shape
    
    let mut shape = String::new();
    io::stdin()
    .read_line(&mut shape)
    .expect("Failed to read line");
    let shape: i32 = shape.trim().parse().expect("Invalid input for the first number");

    if shape == 1 { //the shape is rectangle
        println!("Type two numbers.");
        
        let mut w1 = String::new();
        io::stdin().read_line(&mut w1).expect("Failed to read line");
        
        let mut h1 = String::new();
        io::stdin().read_line(&mut h1).expect("Failed to read line");
        
        let w1: u32 = w1.trim().parse().expect("Invalid input for the first number");
        let h1: u32 = h1.trim().parse().expect("Invalid input for the first number");
        
        let o = Rectangle {
            width: w1,
            height: h1,
        };
        o.show();
        println!("{}", o);
    }

    if shape == 2 { //the shape is triangle
        println!("Type two numbers.");
        
        let mut w1 = String::new();
        io::stdin().read_line(&mut w1).expect("Failed to read line");
        
        let mut h1 = String::new();
        io::stdin().read_line(&mut h1).expect("Failed to read line");
        
        let w1: u32 = w1.trim().parse().expect("Invalid input for the first number");
        let h1: u32 = h1.trim().parse().expect("Invalid input for the first number");
        
        let o = Triangle {
            width: w1,
            height: h1,
        };

        o.show();
        println!("{}", o);
    }

    if shape ==3 { //the shape is circle
        println!("Type one number.");

        let mut r1 = String::new();
        io::stdin().read_line(&mut r1).expect("Failed to read line");

        let r1: u32 = r1.trim().parse().expect("Invalid input for the first number");
        
        let o = Circle {
            radius: r1,
        };
         
        o.show();
        println!("{}", o);
    }
}
