fn main() {
    println!("Hello, world!");

    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    let x = 5;
    let x = x + 1;
    let x = x * 2;

    println!("The value of x is: {}", x);

    let x = 2.0; // f64
    let y: f32 = 3.0; // f32

    println!("The value of x is: {}, y is {}", x, y);

}
