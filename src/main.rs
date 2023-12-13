use std::fs::File;
use std::io::Write;

const WIDTH: usize = 256;
const HEIGHT: usize = 256;

fn caclulcate_crc32(data: &[u8]) -> u32 {
  const CRC32_POLY: u32 = 0xEDB88320;
  let mut crc = 0xFFFFFFFF;
  for &byte in data {
    crc ^= byte as u32;
    for _ in 0..8 {
      let mask = !(crc & 1);
      crc = (crc >> 1) ^ (CRC32_POLY & mask);
    }
  }
  crc ^ 0xFFFFFFFF
}

fn append_chunk(buffer: &mut Vec<u8>, chunk_type: &[u8; 4], data: &[u8]) {
  buffer.extend_from_slice(&(data.len() as u32).to_be_bytes());
  buffer.extend_from_slice(chunk_type);
  buffer.extend_from_slice(data);
  let crc32_chksum = caclulcate_crc32(&[
    chunk_type[0],
    chunk_type[1],
    chunk_type[2],
    chunk_type[3],
  ]);
  buffer.extend_from_slice(&crc32_chksum.to_be_bytes());
}

fn main() {
  let mut png_data = Vec::new();
  // .png file signature
  png_data.extend_from_slice(&[137, 80, 78, 71, 13, 10, 26, 10]);
  // .png file IHDR chunk
  append_chunk(&mut png_data, b"IHDR", &[WIDTH as u8, HEIGHT as u8, 8, 2, 0, 0, 0]);
  // .png file IDAT chunk
  let mut idat_data = Vec::new();
  for _ in 0..HEIGHT {
    idat_data.push(0);
    for _ in 0..WIDTH {
      idat_data.push(255);
    }
  }
  append_chunk(&mut png_data, b"IDAT", &idat_data);
  // .png file IEND chunk
  append_chunk(&mut png_data, b"IEND", &[]);
  if let Ok(mut file) = File::create("png_out/output.png") {
    file.write_all(&png_data).unwrap();
  } else {
    println!("Error occurred while writing to png_out/output.png");
  }
}
