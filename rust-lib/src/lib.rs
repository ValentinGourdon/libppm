// This crate is a library
#![crate_type = "lib"]
// This crate is named "pixel"
#![crate_name = "pixel"]

// Use +nightly to overpass this
#![feature(test)]

#[cfg(test)]
mod tests;

extern crate rand;

use std::ops::Not;
use std::path::Path;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use rand::Rng;

#[derive(Debug, Copy, Clone, PartialEq)]
/// A Color is represented here by three colors, each of them are u8.
pub struct Color {
    /// Red 
    r: u8,
    /// Green
    g: u8,
    /// Blue
    b: u8
}

/// Implements some functions for the struct Color
impl Color {
    /// Constructor
    pub fn new(red: u8, green: u8, blue:u8) -> Color {
        return Color {r : red, g : green, b : blue};
    }

    /// Conctructor with random values for each color
    pub fn new_random() -> Color {
        let mut r = rand::thread_rng();
        
        return Color {
            r : r.gen::<u8>(),
            g : r.gen::<u8>(),
            b : r.gen::<u8>()
        }
    }

    /// Default constructor
    pub fn empty_color() -> Color {
        return Color {r : 0, g : 0, b : 0};
    }

    /// Red's getter
    pub fn red(&self) -> u8 {
        return self.r;
    }

    /// Green's getter
    pub fn green(&self) -> u8 {
        return self.g;
    }

    /// Blue's getter
    pub fn blue(&self) -> u8 {
        return self.b;
    }

    /// toString() to display a Color
    pub fn display(&self) {
        println!("r : {}, g : {}, b : {}", self.r, self.g, self.b);
    }

    /// Equals to determine if the two Color in parameters are equals.
    /// Return true if self and other and equals
    /// (the r, g and b of self are equals to the r, g and b of other)
    /// 
    ///  # Arguments
    /// * `self` - a Color to be compared
    /// * `other` - a second Color to compare the first one
    /// 
    /// # Return
    /// * `bool` - corresponding to the equality (or not) of the two arguments
    pub fn eq(&self, other: Color) -> bool {
        if(self.red() == other.red()) 
            && (self.green() == other.green()) 
            && (self.blue() == other.blue() ) {
            return true;
        }
        return false;
    }

    /// Transform a RGB pixel (Color) to a grayscale pixel (between 0 and 255).
    /// Use an intermediate u32 var to calculate the average without u8 overflow.
    /// 
    /// # Arguments
    /// * `self` - a Color to be converted
    /// 
    /// # Return
    /// * `u8` - an integer corresponding to the converted Color
    /// 
    /// # Example
    /// If a Color(30, 28, 255) is passed as a parameter 
    /// the function will return 104.
    pub fn grayscale(&self) -> u8 {
        let average: u32 = (self.r as u32 + self.g as u32 + self.b as u32)/3;
        return average as u8;
    }
}

/// Impl block to implement the not() function
impl Not for Color {
    type Output = Color;
    
    /// Revert a pixel's color with !color
    /// 
    /// #Arguments
    /// * `self` - a Color to be reverted
    /// 
    /// #Return
    /// * `Self` - a Color reverted
    /// 
    /// #Example
    /// If a Color(100, 50, 75) is passed as a parametr 
    /// the function will return a Color(155, 205, 180).
    fn not(self) -> Self::Output {
        let mut c = self;
        c.r = 255 - c.r;
        c.g = 255 - c.g;
        c.b = 255 - c.b;
        return c;
    }
}

#[derive(Debug)]
/// An image is defined with a width, a height and a pixels.
pub struct Image {
    /// A width is an u32
    width: u32,
    /// A height is an u32
    height: u32,
    /// A pixels is a Vec<Color>
    pixels: Vec<Color>  // 2D array dynamic
}

/// Used to call every Image's functions
impl Image {
    /// Constructor
    pub fn new(width:u32, height:u32, pixels:Vec<Color>) -> Image {
        return Image {width : width, height : height, pixels : pixels};
    }

    /// Width's getter
    pub fn width(&self) -> u32 {
        return self.width;
    }

    /// Height's getter
    pub fn height(&self) -> u32 {
        return self.height;
    }

    /// Pixels getter
    pub fn pixels(&self) -> &Vec<Color> {
        return &self.pixels;
    }

    /// Equals()
    pub fn eq(&self, other: Image) -> bool {
        if self.height != other.height {
            return false;
        }
        if self.width != other.width {
            return false;
        }
        if self.pixels != other.pixels {
            return false;
        }
        return true;
    }


    /// Create a new Image from a .ppm File
    /// # Arguments
    /// * filename: &Path - The path corresponding to the file to be read.
    /// 
    /// # Return
    /// * Option<Image> - The Image created through the file read. It is Optionnal 
    /// to handle the case where a problem occurs during the reading of the file.
    pub fn new_with_file(filename: &Path) -> Option<Image> {
        let mut width: u32 = 0;
        let mut height: u32 = 0;
        let mut pixels: Vec<Color> = Vec::new();
        let file = File::open(filename).expect("Unable to open the File");
        let buf_reader = BufReader::new(file);

        for (i, line) in buf_reader.lines().enumerate().by_ref() {
            // Treatment for the first line, if not P3 it's not a RGB picture => exit.
            if i == 0 {
                if &line.unwrap() != "P3" {
                    return None;
                }
                // The second line is the dimensions of the picture.
            } else if i == 1 {
                let list_num: Vec<u32> = get_number32_from_string(&line.unwrap());
                width = list_num[0];
                height = list_num[1];
            } else {
                // If the line begin with # it's a commentary
                // Or line 2 (the max size of a color), we ignore both.
                let s: &String = &line.unwrap();
                if (s.chars().next().unwrap() != '#') || (i != 2) {
                    let colors = get_number8_from_string(&s);
                    if colors.len() == 3 {
                        let c = Color::new(colors[0], colors[1], colors[2]);
                        pixels.push(c);
                    }
                    
                }
            }
        }
        return Some(Image::new(width, height, pixels));
    }

    /// Invert the Colors of an Image using c.not()
    /// to invert each color of a pixel
    /// 
    /// # Arguments
    /// * image: Image - the image to be inverted
    /// # Return
    /// * Image - the image inverted
    pub fn invert(image: &Image) -> Image {
        let mut inv: Vec<Color> = Vec::new();
        for c in &image.pixels {
            inv.push(c.not());
        }

        return Image::new(image.width, image.height, inv);
    }

    /// Write the image passed as parameter in a file.
    /// # Arguments
    /// * image:Image - the image to write in the file
    /// * path:&Path - the path where the file will be saved plus it's name.
    /// 
    /// # Containts
    /// * The first line is the type of picture : P3 is for the RGB system color
    /// * The second line is the size of the picture (in pixels). 
    ///     Two integers define the file's width and height.
    /// * The third line is the max value of each color (255).
    /// * The rest of the file is the colors. There is (width * height) lines
    ///     of three values (RGB) for each pixel.
    pub fn save_file_from_image(image: &Image, path: &Path) -> std::io::Result<()> {
        let mut file = File::create(path).expect("Unable to create the file");
        file.write_all(b"P3\n").expect("Unable to write P3.");
        file.write_fmt(format_args!("{} {}\n", image.width(), image.height()))
                                        .expect("Unable to write width and height.");
        file.write_all(b"255\n").expect("Unable to write max value for Colors.");
        for c in &image.pixels {
            file.write_fmt(format_args!("{} {} {} \n", c.red(), c.green(), c.blue()))
                                        .expect("Unable to write colors.");
        }
        Ok(())
    }

    /// Return a grayscale Image from a RGB Image.
    /// Each pixel of the grayscale Image is the sum of the RGB pixel / 3.
    /// 
    /// # Arguments
    /// * image:Image - The RGB Image to be converted
    /// # Return
    /// * Image - The grayscale Image converted
    pub fn grayscale(image: &Image) -> Image {
        let mut gray: Vec<Color> = Vec::new(); 

        for i in &image.pixels {
            let c: u8 = Color::grayscale(i);
            gray.push(Color::new(c, c, c));
        }

        return Image::new(image.width, image.height, gray);
    }

}

/// Transform a String with numbers in it into 
/// a Vector<u32> for the picture size.
/// # Example :
/// * "1 23 45" passed as parameters will return Vec{1, 23, 45}. 
/// * "1 23 azer //& &é45" passed as parameters will return Vec{1, 23, 45}
fn get_number32_from_string(line: &String) -> Vec<u32> {
    let mut list_num: Vec<u32> = Vec::new();
    let mut n = String::new();
    for c in line.chars() {
        if c == ' ' || c == '\0' {
            if !n.is_empty() {
                list_num.push(n.parse().unwrap());
                n.clear();
            }
        } else if c.is_digit(10){
            n.push(c);
        }
    }
    // Add if a numer is at the end of the line
    if !n.is_empty() {
        list_num.push(n.parse().unwrap());
    }
    return list_num;
}

/// Transform a String with numbers in it into 
/// a Vector<u8> for the colors.
/// # Example :
/// * "1 23 45" passed as parameters will return Vec{1, 23, 45}. 
/// * "1 23 azer //& &é45" passed as parameters will return Vec{1, 23, 45}
fn get_number8_from_string(line: &String) -> Vec<u8> {
    let mut list_num: Vec<u8> = Vec::new();
    let mut n = String::new();
    for c in line.chars() {
        if c == ' ' || c == '\0' {
            if !n.is_empty() {
                list_num.push(n.parse().unwrap());
                n.clear();
            }
        } else if c.is_digit(10){
            n.push(c);
        }
    }
    // Add if a numer is at the end of the line
    if !n.is_empty() {
        list_num.push(n.parse().unwrap());
    }
    return list_num;
}

