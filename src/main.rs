use std::env;
use std::path::Path;
use image::{GenericImageView, ImageBuffer, Rgba, RgbaImage, imageops};

const SOBELHORIZ: [f32; 9] = [-1.0, 0.0, 1.0, -2.0, 0.0, 2.0, -1.0, 0.0, 1.0];
const SOBELVERTI: [f32; 9] = [-1.0, -2.0, -1.0, 0.0, 0.0, 0.0, 1.0, 2.0, 1.0];
const INVALIDCHARS: &[&str; 16] = &[
  "`", "'", "?", "<", ">", ".", "+",
  "=", "*",":", ";", "&", "(", ")",
  "[", "]",
];

fn pngcopy(pngname: &str, outname: &str) {
  let srcpng = image::open(Path::new(&pngname)).unwrap();
  println!("\npngcopy(): Successfully loaded {}", pngname);
  println!("  -> {} Dimensions: {:?}", pngname, srcpng.dimensions());
  println!("  -> {} Pixel Color Type: {:?}", pngname, srcpng.color());
  let srcpng = srcpng.to_rgba8();
  let mut newpng: RgbaImage = ImageBuffer::new(srcpng.width(), srcpng.height());
  for x in 0..srcpng.width() {
    for y in 0..srcpng.height() {
      let pxl = srcpng.get_pixel(x, y);
      newpng.put_pixel(x, y, *pxl); 
    }
  }
  newpng.save(outname).unwrap();
  println!("\npngcopy(): Successfully saved / rewrote {}", outname);
}

fn rgbainvert(clr: &Rgba<u8>) -> Rgba<u8> {
  let clri = Rgba([
    255 - clr[0],
    255 - clr[1],
    255 - clr[2],
    clr[3],
  ]);
  clri
}

fn pnginvert(pngname: &str, outname: &str) {
  let srcpng = image::open(Path::new(&pngname)).unwrap();
  println!("\npnginvert(): Successfully loaded {}", pngname);
  println!("  -> {} Dimensions: {:?}", pngname,  srcpng.dimensions());
  println!("  -> {} Pixel Color Type: {:?}", pngname, srcpng.color());
  let srcpng = srcpng.to_rgba8();
  let mut newpng: RgbaImage = ImageBuffer::new(srcpng.width(), srcpng.height());
  for x in 0..srcpng.width() {
    for y in 0..srcpng.height() {
      let pxl = srcpng.get_pixel(x, y);
      newpng.put_pixel(x, y, rgbainvert(pxl));
    }
  }
  newpng.save(outname).unwrap();
  println!("\npnginvert(): Successfully saved / rewrote {}", outname);
}

fn pngedges(pngname: &str, outname: &str) {
  let srcpng = image::open(&pngname).unwrap();
  println!("\nedgedetect(): Successfully loaded {}", pngname);
  println!("  -> {} Dimensions: {:?}", pngname, srcpng.dimensions());
  println!("  -> {} Pixel Color Type: {:?}", pngname, srcpng.color());
  let graypng = srcpng.to_luma8();
  let gradx = imageops::filter3x3(&graypng, &SOBELHORIZ);
  let grady = imageops::filter3x3(&graypng, &SOBELVERTI);
  let mut edgepng = RgbaImage::new(srcpng.width(), srcpng.height());
  for x in 0..srcpng.width() {
    for y in 0..srcpng.height() {
      let magx = gradx.get_pixel(x, y)[0] as f32;
      let magy = grady.get_pixel(x, y)[0] as f32;
      let mag = (magx.powi(2) + magy.powi(2)).sqrt() as u8;
      let pxl = Rgba([mag, mag, mag, 255]);
      edgepng.put_pixel(x, y, pxl);
    }
  }
  edgepng.save(outname).unwrap();
  println!("\nedgedetect(): Successfully saved / rewrote {}", outname);
}

fn custompng(
  outname: &str,
  width: u32,
  height: u32,
  f_r: impl Fn(u32, u32) -> u32,
  f_g: impl Fn(u32, u32) -> u32,
  f_b: impl Fn(u32, u32) -> u32,
) { 
  let mut newpng = RgbaImage::new(width, height);
  for x in 0..newpng.width() {
    for y in 0..newpng.height() {
      let pxl = Rgba([f_r(x, y) as u8, f_g(x, y) as u8, f_b(x, y) as u8, 255]);
      newpng.put_pixel(x, y, pxl);
    }
  }
  newpng.save(outname).unwrap();
  println!("\ncustompng(): Successfully saved / rewrote {}", outname);
}

fn main() {
  let pngname = if env::args().count() == 3 {
    for &c in INVALIDCHARS {
      if env::args().nth(1).unwrap().contains(c) {
        panic!("main(): Invalid character '{}' in pngname '{}'", c, env::args().nth(2).unwrap());
      }
    }
    env::args().nth(1).unwrap()
  } else {
    panic!("main(): Please enter from src/png_in to use as the first argument");
  };
  let outname = if !env::args().nth(2).unwrap().trim().is_empty() {
    for &c in INVALIDCHARS {
      if env::args().nth(2).unwrap().contains(c) {
        panic!("main(): Invalid character '{}' in outname '{}'", c, env::args().nth(2).unwrap());
      }
    }
    env::args().nth(2).unwrap()
  } else {
    panic!("main(): Please enter a .png name to output to src/png_out as the second argument");
  };
  //let pngname = format!("src/png_in/{}.png", &pngname);
  //let copyname = format!("src/png_out/{}.png", &outname);
  //let invname = format!("src/png_out/{}_i.png", &outname);
  //let edgname = format!("src/png_out/{}_e.png", &outname);
  let customname = format!("src/png_out/{}_c.png", &outname);
  //pngcopy(&pngname, &copyname);
  //pnginvert(&pngname, &invname);
  //pngedges(&pngname, &edgname);
  custompng(
    &customname, 1000, 1000,
    |x, y| (x*x + y*y) as u32,
    |x, y| (x + y).wrapping_mul(x.wrapping_sub(y)) as u32,
    |x, y| (x * y) as u32,
  );
}
