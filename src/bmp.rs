use std::fs;
use std::io::Write;

struct BmpPixel {
	pub r: u8,
	pub g: u8,
	pub b: u8,
}

pub struct BmpImage {
	width: u32,
	height: u32,
	data: Vec<BmpPixel>
}

impl BmpImage {
	pub fn new(width: u32, height: u32) -> BmpImage {
		let mut data = Vec::with_capacity((width * height) as usize);
		for _ in 0..width*height {
			data.push(BmpPixel{r:0, g:0, b:0});
		}

		BmpImage {
			width: width,
			height: height,
			data: data
		}
	}

	pub fn write_to_file(&self, file_path: String) {
		let size_of_file_header: u32 = 14;
		let size_of_bitmap_header: u32 = 40;
		let size_of_header: u32 = size_of_file_header + size_of_bitmap_header;

		let size_of_padding: u32 = (4 - (self.width*3) % 4) % 4;
		let filesize = size_of_header + self.height * (3*self.width + size_of_padding);

   		let mut stream = Vec::with_capacity(filesize as usize);

		stream.push('B' as u8);
		stream.push('M' as u8);
		stream.push(((filesize >>  0) & 0xFF) as u8);
		stream.push(((filesize >>  8) & 0xFF) as u8);
		stream.push(((filesize >> 16) & 0xFF) as u8);
		stream.push(((filesize >> 24) & 0xFF) as u8);
		stream.push(0);
		stream.push(0);
		stream.push(0);
		stream.push(0);
		stream.push(size_of_header as u8);
		stream.push(0);
		stream.push(0);
		stream.push(0);

		stream.push(size_of_bitmap_header as u8);
		stream.push(0);
		stream.push(0);
		stream.push(0);
		stream.push(((self.width  >>  0) & 0xFF) as u8);
		stream.push(((self.width  >>  8) & 0xFF) as u8);
		stream.push(((self.width  >> 16) & 0xFF) as u8);
		stream.push(((self.width  >> 24) & 0xFF) as u8);
		stream.push(((self.height >>  0) & 0xFF) as u8);
		stream.push(((self.height >>  8) & 0xFF) as u8);
		stream.push(((self.height >> 16) & 0xFF) as u8);
		stream.push(((self.height >> 24) & 0xFF) as u8);
		stream.push(1);
		stream.push(0);
		stream.push(24);
		stream.push(0);

		while size_of_header as usize > stream.len() {
			stream.push(0);
		}

		for row in 0..self.height {
			for col in 0..self.width {
				let ind = (row*self.width + col) as usize;
				let p = &self.data[ind];
				stream.push(p.b);
				stream.push(p.g);
				stream.push(p.r);
			}
			for _ in 0..size_of_padding {
				stream.push(0);
			}
		}

		let mut file = fs::File::create(file_path).unwrap();
		file.write(&stream).unwrap();
	}

	pub fn set_pixel(&mut self, row:u32, col:u32, r:f64, g:f64, b:f64) {
		// row is inverted
		self.data[((self.height-1-row)*self.width+col) as usize] = BmpPixel{
			r: BmpImage::float_to_byte(r),
			g: BmpImage::float_to_byte(g),
			b: BmpImage::float_to_byte(b)
		};
	}

	pub fn float_to_byte(value: f64) -> u8 {
		if value >= 1.0 {
			return 255 as u8;
		} else if value <= 0.0 {
			return 0 as u8;
		} else {
			return (value * 255 as f64) as u8;
		}
	}
}
