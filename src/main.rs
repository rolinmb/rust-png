//use std::env;
use std::path::Path;
use image::{GenericImageView, ImageBuffer, RgbaImage};

//const WIDTH: u32 = 1000;
//const HEIGHT: u32 = 1000;

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
  /*let pngname = if env::args().count() == 3 {
    env::args().nth(1).unwrap()
  } else {
    panic!("Please select a file from /png_in/ to use");
  };
  */let pngname = "png_in/IMG_0950.png";
  let outname = "png_out/test.png";
  pngcopy(pngname, outname);
  /*let img: RgbImage = ImageBuffer::new(WIDTH, HEIGHT);
  let img = ImageBuffer::from_fn(WIDTH, HEIGHT, |x, _| {
    if x % 2 == 0 {
      image::Luma([0u8])
    } else {
      image::Luma([255u8])
    }
  });
  img.save(outname).unwrap();
  println!("successfully created {}", outname);*/
}
