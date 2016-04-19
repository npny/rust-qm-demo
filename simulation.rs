use std::vec::Vec;
use num::complex::Complex64;



#[derive(Copy, Clone)]
pub struct FieldValue {
	
	// Complex amplitude
	pub value: Complex64,

	// Next complex amplitude, used as a temporary storage in order not to corrupt the current iteration data
	pub next_value: Complex64,

	// Potential
	pub potential: f64,

}

impl FieldValue {
	
	pub fn new() -> FieldValue {
		FieldValue {
			value: Complex64::new(0f64, 0f64),
			next_value: Complex64::new(0f64, 0f64),
			potential: 0f64,
		}
	}
}






pub struct Simulation {

	h: f64, // Reduced planck constant
	e: f64, // Elementary charge
	m: f64, // Field mass
	l: f64, // Timescale factor, closer to 0 means better stability (but longer simulation)

	t: u64, // Timestep, starts at 0 and increases
	//E: f64, // Total energy of the system, computed continuously
	k: f64, // Inverse normalization factor

	pub width: u32,
	pub height: u32,
	pub size: usize,
	pub field: Vec<FieldValue>, // Using a heap-allocated Vec because an array would quickly exceeds the stack size

}


impl Simulation {


	pub fn new(width: u32, height: u32) -> Simulation {
		Simulation {
			h: 1f64,
			e: 1f64,
			m: 1f64,
			l: 0.1f64,

			t: 0u64,
			//E: 0f64,
			k: 0f64,

			width: width,
			height: height,
			size: (width*height) as usize,
			field: vec![FieldValue::new(); (width*height) as usize],
		}
	}


	pub fn update(self: &mut Simulation) {

		let mut max_norm = 0f64;
		for y in 0..self.height {
			for x in 0..self.width {

				// Compute iteration
				let hamiltonian = self.hamiltonian(x, y);
				let i = self.at(x as i64, y as i64);
				
				let mut cell = &mut self.field[i];
				cell.next_value = cell.value + hamiltonian * Complex64::new(0f64, 1f64 / self.h) * self.l;

				// Maintain a max_norm variable for future normalization
				let norm = cell.next_value.norm_sqr();
				if norm > max_norm { max_norm = norm; }
			}
		}


		// Feed next_value back into the current value and normalize on the fly
		let max_norm_sqrt = max_norm.sqrt();
		for i in 0..self.size {

			self.field[i].value = self.field[i].next_value / max_norm_sqrt;

		}

		self.k = max_norm_sqrt;
		self.t += 1;

		println!("t={} \t k={:.3}", &self.t, &self.k);
	}



	pub fn hamiltonian(self: &Simulation, x: u32, y: u32) -> Complex64 {

		let d = |dx: i64, dy: i64| -> Complex64 {
			let i = self.at(x as i64 + dx, y as i64 + dy);
			self.field[i].value
		};

		// Laplacian on horizontal, vertical, and diagonal neighbors
		let lapl = d(1,0) + d(0,1) + d(-1,0) + d(0,-1)   -   d(0,0)*4f64 +
		         ( d(1,1) + d(-1,-1) + d(1,-1) + d(-1,1)   -   d(0,0)*4f64 ) / 1.414f64;


		let potential = self.field[ self.at(x as i64, y as i64) ].potential;		

		(self.h * self.h / 2f64 * self.m) * lapl  +  self.e * potential * d(0, 0)

	}


	pub fn initialize(self: &mut Simulation) {

		let wave_packet_x = 50;
		let wave_packet_y = 70;
		let wave_packet_r = 20;
		let wave_packet_k = 10; // One rotation every n pixels

		for y in 0..self.height {
			for x in 0..self.width {

				let phase = (x as f64 / wave_packet_k as f64) * 3.141f64;
				let dx = wave_packet_x as i64 - x as i64;
				let dy = wave_packet_y as i64 - y as i64;
				let d = (dx*dx + dy*dy) as f64 / wave_packet_r as f64;
				let amp = (-d).exp();

				let i = self.at(x as i64, y as i64);
				self.field[i].value = Complex64::new(phase.cos(), phase.sin()) * amp;
				
				let rx = (x as f64 / self.width as f64) - 0.5f64;
				let ry = (y as f64 / self.height as f64) - 0.5f64;
				self.field[i].potential = (rx*rx + ry*ry) * 2f64

			}
		}

	}


	pub fn at(self: &Simulation, x: i64, y: i64) -> usize {

		let w = self.width as i64;
		let h = self.height as i64;
		let _x = (x + w) % w;
		let _y = (y + h) % h;
		(_y * w + _x) as usize

	}

}