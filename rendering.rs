extern crate sfml;
use sfml::graphics::{Texture, Sprite};
use simulation::*;


#[derive(Copy, Clone)]
pub struct PixelValue {
	pub r: u8,
	pub g: u8,
	pub b: u8,
	pub a: u8,
}


pub struct Viewport<'a> {

	pub simulation: *const Simulation,
	pub mapping_function: fn(&FieldValue) -> PixelValue,
	pub pixels: Vec<u8>,
	pub texture: Texture,
	pub sprite: Sprite<'a>

}


impl<'a> Viewport<'a> {


	pub fn new<'function_lifetime>(simulation: &'function_lifetime Simulation, mapping_function: fn(&FieldValue) -> PixelValue) -> Viewport<'a> {

		let texture = Texture::new(simulation.width, simulation.height).unwrap();
		let mut sprite = Sprite::new().unwrap();
		
		// Forced to use an unsafe pointer here because the C API can use the reference at any time, possibly past the lifetime of the struct
		unsafe {
			sprite.set_texture(&( *( &texture as *const Texture ) ), false);
		}

		Viewport {
			simulation: simulation,
			mapping_function: mapping_function,
			pixels: vec![0; simulation.size * 4],
			texture: texture,
			sprite: sprite
		}

	}


	pub fn update(&mut self) {

		unsafe {
			map_field_to_pixels(&(*self.simulation).field, &mut self.pixels, self.mapping_function);
		}

		let size = self.texture.get_size();
		self.texture.update_from_pixels(&self.pixels, size.x, size.y, 0, 0);

	}


}


pub fn map_field_to_pixels(field: &[FieldValue], pixels: &mut [u8], mapping_function: fn(&FieldValue) -> PixelValue) {

	assert_eq!(field.len() * 4, pixels.len());

	for i in 0..field.len() {

		let pixel = mapping_function(&field[i]);

		pixels[i*4 + 0] = pixel.r;
		pixels[i*4 + 1] = pixel.g;
		pixels[i*4 + 2] = pixel.b;
		pixels[i*4 + 3] = pixel.a;

	}

}



// Pixel mappings


pub fn red_blue_components(val: &FieldValue) -> PixelValue {

	PixelValue {
		r: (128f64 + val.value.re*128f64) as u8,
		g: 0,
		b: (128f64 + val.value.im*128f64) as u8,
		a: 255,
	}

}

pub fn red_blue_components_alpha_norm(val: &FieldValue) -> PixelValue {

	let mut pixel = red_blue_components(val);
	pixel.a = (255f64 * val.value.norm()) as u8;
	pixel

}

pub fn white_potential(val: &FieldValue) -> PixelValue {

	let level = (255f64 * val.potential) as u8;
	PixelValue {
		r: level,
		g: level,
		b: level,
		a: 255,
	}

}