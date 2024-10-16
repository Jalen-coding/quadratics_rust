use std::io;
use std::f64;


fn main() {
    println!("Enter coefficent for A");
        let mut a = String::new();
        io::stdin()
            .read_line(&mut a)
            .expect("Failed to read line error");
        let a = a.trim();
        let a: f64 = a.parse().expect("Invalid Input");

        println!("Enter coeffiecent for B");
        let mut b = String::new();
        io::stdin()
            .read_line(&mut b)
            .expect("Failed to read line error");
        let b = b.trim();
        let b: f64 = b.parse().expect("Invalid Input");

        println!("Enter coeffiecent for C");
        let mut c = String::new();
        io::stdin()
            .read_line(&mut c)
            .expect("Failed to read line error");
        let c = c.trim();
        let c: f64 = c.parse().expect("Invalid Input");

        let mut direction = "Nothing";
        if a > 0.0 {
            direction = "The Graph is facing UP";
        } else {
            direction = "The Graph is facing DOWN";
        }

        let h = -b / (2.0 * a);
        let k = (4.0 * a * c * (b*b)) / (4.0 * a);
        let vertex = (h, k);

        let p = 1.0 / (4.0 * a);
        let abs_p = p.abs();

        let mut pos = "Nothing";
        if a != 0.0 {
            pos = "Vertical";
        } else {
            pos = "Horizontal";
        }

        let mut focus = (1.0, 0.0);
        if pos == "Vertical" {
            focus = (h, k+abs_p);
        } else {
            focus = (h+abs_p, k);
        }

        let mut directrix = 0.0;
        if pos == "Vertical" {
            directrix = k - p;
        } else {
            directrix =  h - p;
        }

        let mut quadratic_formula = (0.0, 0.0);
        if a != 0.0 {
            let discriminant = (b*b) - 4.0 * a * c;
            if discriminant >= 0.0 {
                let x1 = (-b + discriminant.sqrt()) / (2.0 * a);
                let x2 = (-b - discriminant.sqrt()) / (2.0 * a);
                quadratic_formula = (x1, x2);
            } else {
                quadratic_formula = (0.0, 0.0);
            }
        } else {
            quadratic_formula = (-c/b, 0.0);
        }

        println!("\nParabola Information:");
        println!("f(x) = {}x^2 + {}x + {}", a,b,c);
        println!("Direction: {}", direction);
        println!("Vertex: {:.2} {:.2}", vertex.0, vertex.1);
        println!("Focus: {:.2} {:.2}", focus.0, focus.1);
        println!("Directrix: y = {:.2}", directrix);
        println!("Quardratic Formula: x = {:.2} or x = {:.2}", quadratic_formula.0, quadratic_formula.1);
}