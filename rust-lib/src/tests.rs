extern crate test;

use super::*;
use test::Bencher;

#[test]
fn two_colors_are_equals() {
    let c = Color::new(30, 128, 255);
    let c1 = Color::new(30, 128, 255);

    assert_eq!(true, Color::eq(&c, c1));
}

#[test]
fn two_colors_are_not_equals() {
    let c = Color::new(30, 128, 255);
    let c1 = Color::new(0, 0, 0);

    assert_ne!(true, Color::eq(&c, c1));
}

#[test]
fn two_images_are_equals() {
    let mut c: Vec<Color> = Vec::new();
    for _i in 0..6 {
        c.push(Color::new(255, 255, 255));
    }
    let i:Image = Image::new(3, 2, c);

    let mut c2: Vec<Color> = Vec::new();
    for _i in 0..6 {
        c2.push(Color::new(255, 255, 255));
    }
    let i2:Image = Image::new(3, 2, c2);

    assert_eq!(true, i.eq(i2));
}

#[test]
fn two_images_with_different_sizes_are_not_equals() {
    let mut c: Vec<Color> = Vec::new();
    for _i in 0..6 {
        c.push(Color::new(255, 255, 255));
    }
    let i:Image = Image::new(3, 2, c);

    let mut c2: Vec<Color> = Vec::new();
    for _i in 0..6 {
        c2.push(Color::new(255, 255, 255));
    }
    let i2:Image = Image::new(2, 2, c2);

    assert_ne!(true, i.eq(i2));
}

#[test]
fn two_images_with_all_different_colors_are_not_equals() {
    let mut c: Vec<Color> = Vec::new();
    for _i in 0..6 {
        c.push(Color::new(255, 255, 255));
    }
    let i:Image = Image::new(3, 2, c);

    let mut c2: Vec<Color> = Vec::new();
    for _i in 0..6 {
        c2.push(Color::new(0, 0, 0));
    }
    let i2:Image = Image::new(2, 2, c2);

    assert_ne!(true, i.eq(i2));
}

#[test]
fn two_images_with_one_different_color_are_not_equals() {
    let mut c: Vec<Color> = Vec::new();
    for _i in 0..6 {
        c.push(Color::new(255, 255, 255));
    }
    let i:Image = Image::new(3, 2, c);

    let mut c2: Vec<Color> = Vec::new();
    c2.push(Color::new(0, 0, 0));
    for _i in 0..5 {
        c2.push(Color::new(255, 255, 255));
    }
    let i2:Image = Image::new(2, 2, c2);

    assert_ne!(true, i.eq(i2));
}

#[test]
fn two_images_with_one_different_digit_are_not_equals() {
    let mut c: Vec<Color> = Vec::new();
    for _i in 0..6 {
        c.push(Color::new(255, 255, 255));
    }
    let i:Image = Image::new(3, 2, c);

    let mut c2: Vec<Color> = Vec::new();
    for _i in 0..5 {
        c2.push(Color::new(255, 255, 255));
    }
    c2.push(Color::new(255, 255, 254));
    let i2:Image = Image::new(2, 2, c2);

    assert_ne!(true, i.eq(i2));
}


#[test]
fn rgb_color_transformation_succesfull_in_grayscale() {
    let c = Color::new(30, 128, 255);

    assert_eq!(137, Color::grayscale(&c));
}

#[test]
fn rgb_color_transformation_unsuccesfull_in_grayscale() {
    let c = Color::new(30, 128, 255);

    assert_ne!(255, Color::grayscale(&c));
}

#[test]
fn rgb_color_revert_succesfull() {
    let c = Color::new(30, 128, 255);

    assert_eq!(Color::new(225, 127, 0), !c);
}

#[test]
fn rgb_color_revert_unsuccesfull() {
    let c = Color::new(30, 128, 255);

    assert_ne!(Color::new(0,0,0), !c);
}

#[test]
fn string_with_number32_transformed_in_vector() {
    let s: String = String::from("1 23 45");
    let mut vec: Vec<u32> = Vec::new();
    vec.push(1);
    vec.push(23);
    vec.push(45);

    assert_eq!(vec, get_number32_from_string(&s));
}

#[test]
fn string_with_number32_transformed_in_wrong_vector() {
    let s: String = String::from("1 23");
    let mut vec: Vec<u32> = Vec::new();
    vec.push(1);
    vec.push(23);
    vec.push(45);

    assert_ne!(vec, get_number32_from_string(&s));
}

#[test]
fn string_with_number32_and_char_transformed_in_vector() {
    let s: String = String::from("1 23 &// a 45");
    let mut vec: Vec<u32> = Vec::new();
    vec.push(1);
    vec.push(23);
    vec.push(45);

    assert_eq!(vec, get_number32_from_string(&s));
}

#[test]
fn string_with_number32_and_char_transformed_in_wrong_vector() {
    let s: String = String::from("1 23 & a");
    let mut vec: Vec<u32> = Vec::new();
    vec.push(1);
    vec.push(23);
    vec.push(45);

    assert_ne!(vec, get_number32_from_string(&s));
}

#[test]
fn string_with_number8_transformed_in_vector() {
    let s: String = String::from("1 23 45");
    let mut vec: Vec<u8> = Vec::new();
    vec.push(1);
    vec.push(23);
    vec.push(45);

    assert_eq!(vec, get_number8_from_string(&s));
}

#[test]
fn string_with_number8_transformed_in_wrong_vector() {
    let s: String = String::from("1 23");
    let mut vec: Vec<u8> = Vec::new();
    vec.push(1);
    vec.push(23);
    vec.push(45);

    assert_ne!(vec, get_number8_from_string(&s));
}

#[test]
fn string_with_number8_and_char_transformed_in_vector() {
    let s: String = String::from("1 23 &// a 45");
    let mut vec: Vec<u8> = Vec::new();
    vec.push(1);
    vec.push(23);
    vec.push(45);

    assert_eq!(vec, get_number8_from_string(&s));
}

#[test]
fn string_with_number8_and_char_transformed_in_wrong_vector() {
    let s: String = String::from("1 23 & a");
    let mut vec: Vec<u8> = Vec::new();
    vec.push(1);
    vec.push(23);
    vec.push(45);

    assert_ne!(vec, get_number8_from_string(&s));
}

#[test]
fn an_image_inverted_equals_its_opposite() {
    let mut c: Vec<Color> = Vec::new();
    for _i in 0..6 {
        c.push(Color::new(255, 255, 255));
    }
    let i:Image = Image::new(3, 2, c);

    let mut c2: Vec<Color> = Vec::new();
    for _i in 0..6 {
        c2.push(Color::new(0, 0, 0));
    }
    let i2:Image = Image::new(3, 2, c2);

    assert_eq!(true, i.eq(Image::invert(&i2)));
}

#[test]
fn an_image_inverted_is_not_equals_to_its_opposite() {
    let mut c: Vec<Color> = Vec::new();
    for _i in 0..6 {
        c.push(Color::new(255, 255, 255));
    }
    let i:Image = Image::new(3, 2, c);

    let mut c2: Vec<Color> = Vec::new();
    for _i in 0..6 {
        c2.push(Color::new(255, 255, 255));
    }
    let i2:Image = Image::new(3, 2, c2);

    assert_ne!(true, i.eq(Image::invert(&i2)));
}

#[test]
fn rgb_image_to_grayscale_image_work() {
    let mut c: Vec<Color> = Vec::new();
    for _i in 0..6 {
        c.push(Color::new(30, 128, 255));
    }
    let mut im:Image = Image::new(3, 2, c);
    im = Image::grayscale(&im);

    let mut c2: Vec<Color> = Vec::new();
    for _i in 0..6 {
        c2.push(Color::new(137, 137, 137));
    }
    let im2:Image = Image::new(3, 2, c2);

    assert_eq!(true, im.eq(im2));
}

#[test]
fn rgb_image_to_grayscale_image_dont_work() {
    let mut c: Vec<Color> = Vec::new();
    for _i in 0..6 {
        c.push(Color::new(30, 128, 255));
    }
    let mut im:Image = Image::new(3, 2, c);
    im = Image::grayscale(&im);

    let mut c2: Vec<Color> = Vec::new();
    for _i in 0..6 {
        c2.push(Color::new(255, 255, 255));
    }
    let im2:Image = Image::new(3, 2, c2);

    assert_ne!(true, im.eq(im2));
}

/// Benchmarks
#[bench]
fn bench_color_new(b: &mut Bencher) {
    b.iter(|| Color::new(255, 255, 255));
}

#[bench]
fn bench_color_new_random(b: &mut Bencher) {
    b.iter(|| Color::new_random());
}

#[bench]
fn bench_colors_equals(b: &mut Bencher) {
    let c = Color::new(30, 128, 255);
    let c1 = Color::new(30, 128, 255);

    b.iter(|| c.eq(c1));
}

#[bench]
fn bench_color_grayscale(b: &mut Bencher) {
    let c = Color::new(30, 128, 255);

    b.iter(|| c.grayscale());
}

#[bench]
fn bench_color_not(b: &mut Bencher) {
    let c = Color::new(30, 128, 255);

    b.iter(|| !c);
}
/*
#[bench]
fn bench_image_new(b: &mut Bencher) {
    let mut c: Vec<Color> = Vec::new();
    for _i in 0..6 {
        c.push(Color::new(255, 255, 255));
    }

    b.iter(|| Image::new(3, 2, &c));
}

#[bench]
fn bench_image_eq(b: &mut Bencher) {
    let mut c: Vec<Color> = Vec::new();
    for _i in 0..6 {
        c.push(Color::new(255, 255, 255));
    }
    let im:Image = Image::new(3, 2, c);

    let mut c2: Vec<Color> = Vec::new();
    for _i in 0..6 {
        c2.push(Color::new(255, 255, 255));
    }
    let im2:Image = Image::new(3, 2, c2);

    b.iter(|| im.eq(im2));
}

#[bench]
fn bench_new_with_file(b: &mut Bencher) {
    let open_path = Path::new("src/example.ppm");*

    b.iter(|| new_with_file(open_path));
}

#[bench]
fn bench_get_number_32_from_string(b: &mut Bencher) {
    let s: String = String::from("13 128 255");

    b.iter(|| get_number32_from_string(s));
}

#[bench]
fn bench_get_number_8_from_string(b: &mut Bencher) {
    let s: String = String::from("13 128 255");
    
    b.iter(|| get_number8_from_string(s));
}

#[bench]
fn bench_invert(b: &mut Bencher) {
    let open_path = Path::new("src/example.ppm");
    let image = new_with_file(open_path);

    b.iter(|| invert(&image));
}

#[bench]
fn bench_save_file_from_image(b: &mut Bencher) {
    let mut c: Vec<Color> = Vec::new();
    for _i in 0..6 {
        c.push(Color::new(255, 255, 255));
    }
    let im:Image = Image::new(3, 2, c);
    let open_path = Path::new("src/example.ppm");
    
    b.iter(|| save_file_from_image(&im, &open_path));
}

#[bench]
fn bench_grayscale_image(b: &mut Bencher) {
    let mut c: Vec<Color> = Vec::new();
    for _i in 0..6 {
        c.push(Color::new(255, 255, 255));
    }
    let im:Image = Image::new(3, 2, c);

    b.iter(|| grayscale(&im));
}
*/