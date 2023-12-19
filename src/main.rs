use std::env;
use std::path::Path;
use image::{GenericImageView, ImageBuffer, RgbaImage};
//const WIDTH: u32 = 1000;
//const HEIGHT: u32 = 1000;
const INVALIDCHARS: &[&str; 16] = &[
  "`", "'", "?", "<", ">", ".", "+",
  "=", "*",":", ";", "&", "(", ")",
  "[", "]",
];

fn pngcopy(pngname: &str, outname: &str) {
  let srcpng = image::open(Path::new(&pngname)).unwrap();
  println!("\nmain(): Successfully loaded {}", pngname);
  let (pngwidth, pngheight) = srcpng.dimensions();
  println!("  -> {} Dimensions: {:?}", pngname, srcpng.dimensions());
  println!("  -> {} Pixel Color Type: {:?}", pngname, srcpng.color());
  let srcpng = srcpng.to_rgba8();
  let mut newpng: RgbaImage = ImageBuffer::new(pngwidth, pngheight);
  for x in 0..pngwidth {
    for y in 0..pngheight {
      let pxl = srcpng.get_pixel(x, y);
      newpng.put_pixel(x, y, *pxl); 
    }
  }
  newpng.save(outname).unwrap();
  println!("\nmain(): Successfully saved {}", outname);
}

fn main() {
  let pngname = if env::args().count() == 3 {
    for &c in INVALIDCHARS {
      if env::args().nth(1).unwrap().contains(c) {
        panic!("Invalid character in filename");
      }
    }
    env::args().nth(1).unwrap()
  } else {
    panic!("Please enter from src/png_in to use as the first argument");
  };
  let outname = if !env::args().nth(2).unwrap().trim().is_empty() {
    for &c in INVALIDCHARS {
      if env::args().nth(2).unwrap().contains(c) {
        panic!("Invalid character in filename");
      }
    }
    env::args().nth(2).unwrap()
  } else {
    panic!("Please enter a .png name to output to src/png_out as the second argument");
  };
  let pngname = format!("src/png_in/{}.png", &pngname);
  let outname = format!("src/png_out/{}.png", &outname);
  pngcopy(&pngname, &outname);
}
