use std::io;

fn main() {
    println!("This a program that calculates if three numbers form a pythagorean theorem and shows the triangle area.");

    loop {
        println!();
    
        let mut a = String::new();
        let mut b = String::new();
        let mut c = String::new();

        println!("Insert the first number: ");

        io::stdin()
            .read_line(&mut a)
            .expect("Failed to read input.");
        let a: u32 = a.trim().parse().expect("Please, type a number!");

        println!("Insert the second number:");

        io::stdin()
            .read_line(&mut b)
            .expect("Failed to read input.");
        let b: u32 = b.trim().parse().expect("Please, type a number!");

        println!("And lastly, the third number: ");

        io::stdin()
            .read_line(&mut c)
            .expect("Failed to read input.");
        let c: u32 = c.trim().parse().expect("Please, type a number!");

        if a*a == b*b + c*c || b*b == c*c + a*a || c*c == a*a + b*b {
            println!("Yes, these three numbers form a pythagorean triple!");
            let area = areacalculator(a, b, c);
            println!("The area of this triangle is {area}");
        } else {
            println!("No, these numbers do not form a pythagorean triple.");
        }
    }
}

fn areacalculator(x: u32, y: u32, z: u32) -> u32{
    let mut attributes = vec![x, y, z];
    attributes.sort();
    let area = (attributes[0] * attributes[1]) / 2;
    return area;
}