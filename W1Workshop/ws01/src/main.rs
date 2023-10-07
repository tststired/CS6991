use bmp::{Image, Pixel};

fn main() {
    let path = std::env::args().nth(1).expect("You must provide a path.");
    let operation = std::env::args().nth(2).expect("You must provide an operation.");

    if operation.as_str() == "pixel" {
        draw_pixel(path.as_str());
    } else if operation.as_str() == "diag" {
        diagonal_line(path.as_str());
    } else if operation.as_str() == "something_else" {
        // Add more cases here!
    } else {
        eprintln!("The operation {operation} was not recognised!");
    }
}

fn draw_pixel(path: &str) {
    let mut image = Image::new(100, 100);
    image.set_pixel(50, 50, Pixel::new(255, 255, 255));
    image.save(path).expect("This should save correctly.");
}

fn diagonal_line(path: &str) {
    let mut image = Image::new(100, 100);
    
    for x in 0..99 {
        for y in 0..99 {
            if (y%2 == 1) ^ (x%2 == 1) {
                image.set_pixel(x, y, Pixel::new(255,0,0))
            }
        }
    }
    

    image.save(path).expect("This should save correctly.");
}