extern crate pixel;

use std::path::Path;

fn main() {
    let c = pixel::Color::empty_color();
    let c1 = pixel::Color::new(30, 128, 255);
    let mut c2 = pixel::Color::new(100, 50, 75);
    
    println!("c : ");
    c.display();
    println!("c1 : ");
    c1.display();

    println!("c2 : ");
    c2.display();
    c2 = !c2;
    println!("Reverted c2 : ");
    c2.display();
    
    if c1.eq(c2) {
        println!("c1 == c2");
    } else {
        println!("c1 != c2");
    }

    let c3 = pixel::Color::new(1, 1, 1);
    if c1.eq(c3) {
        println!("c1 == c3");
    } else {
        println!("c1 != c3");
    }

    println!("c en grayscale : {}", c.grayscale());
    println!("c1 en grayscale : {}", c1.grayscale());
    println!("c2 en grayscale : {}", c2.grayscale());

    let open_path = Path::new("src/example.ppm");
    let image = pixel::Image::new_with_file(open_path);
    match image {
        Some(image) => {
            println!("Picture print : ");
            println!("widht : {}, height : {}", image.width(), image.height());
            for c in image.pixels() {
                c.display();
            }
        },
        None => println!("No Image found !")
    }
    let image2 = pixel::Image::new_with_file(open_path);
    println!("Inverted picture print : ");
    match image2 {
        Some(image2) => { 
            let inverted_image = pixel::Image::invert(&image2);
            println!("widht : {}, height : {}", inverted_image.width(), inverted_image.height());
            for c in inverted_image.pixels() {
                c.display();
            }
        },
        None => println!("No Image found !")
    }

    println!("\nRandom generated colors :");
    let mut colors: Vec<pixel::Color> = Vec::new();
    let w:u32 = 500;
    let h:u32 = 500;
    for _i in 0..(w*h) {
        let c: pixel::Color = pixel::Color::new_random();
        colors.push(c);
    }
    let image3: pixel::Image = pixel::Image::new(w, h, colors); 

    println!("Write Image in a file");
    let save_path = Path::new("src/example2.ppm");
    pixel::Image::save_file_from_image(&image3, save_path).expect("Unable to write simple file.");
    println!("File written !");

    println!("Write the inverted Image in a file");
    let image4: pixel::Image = pixel::Image::invert(&image3);
    let save_path = Path::new("src/example3.ppm");
    pixel::Image::save_file_from_image(&image4, save_path).expect("Unable to write inverted file.");
    println!("Inverted file written !");

    println!("Write the grayscale Image in a file");
    let image5: pixel::Image = pixel::Image::grayscale(&image4);
    let save_path = Path::new("src/example4.ppm");
    pixel::Image::save_file_from_image(&image5, save_path).expect("Unable to write grayscale file.");
    println!("Grayscale file written !");
}
