use std::fmt;
use std::{io};
use std::f32::consts::PI;

fn input(prompt: &str) -> f32{
  println!("{}", prompt);
  let mut s = String::new();
  std::io::stdin().read_line(&mut s).expect("Failed to read line");
  s.trim().parse().expect("Not a Number.")
}

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
    println!("{} and {} with area: {}", self.width, self.height, self.area());
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
    //selct shape
    let mut shape = input("1.Rectangle, 2.Triangle, 3.Circle")  

    if shape == 1 { //the shape is rectangle
      let mut w1 = input("Type width");
      let mut h1 = input("Type height");
        
        let o = Rectangle {
            width: w1,
            height: h1,
        };
        o.show();
        println!("{}", o);
    }

    else if shape == 2 { //the shape is triangle
        let mut w1 = input("Type width");
        let mut h1 = input("Type height");
        
        let o = Triangle {
            width: w1,
            height: h1,
        };

        o.show();
        println!("{}", o);
    }

    else if shape == 3 { //the shape is circle
        let mut r1 = input("Type radius");
        
        let o = Circle {
            radius: r1,
        };
         
        o.show();
        println!("{}", o);
    }

    else{
      println!("You type wrong number!!");
    }
}
