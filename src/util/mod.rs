pub mod frame_counter;

macro_rules! max {
	($a:expr, $b:expr) => ({
		let a = $a;
		let b = $b;
		if a > b { a } else { b }
	})
}

macro_rules! min {
	($a:expr, $b:expr) => ({
		let a = $a;
		let b = $b;
		if a < b { a } else { b }
	})
}

// These should be in the debug module, but whatever (I'll take time out to refactor this one day):

pub static mut PYRITE_DYN_DEBUG_ENABLED: bool = false;

// (last_start, last_time, total_samples, total_time)
pub static mut PYRITE_MEASURES: [(u64, u64, u64, u64); 16] = [(0u64, 0u64, 0u64, 0u64); 16];

macro_rules! pyrite_measure_start {
	($index:expr) => (
		unsafe {
			::util::PYRITE_MEASURES[$index].0 = ::time::precise_time_ns();
		}
	)
}

macro_rules! pyrite_measure_end {
	($index:expr) => (
		unsafe {
			::util::PYRITE_MEASURES[$index].1 = ::time::precise_time_ns() - ::util::PYRITE_MEASURES[$index].0;
			::util::PYRITE_MEASURES[$index].3 += ::util::PYRITE_MEASURES[$index].1;
			::util::PYRITE_MEASURES[$index].2 += 1;
		}
	)
}

macro_rules! pyrite_measure_print {
	($index:expr) => (
		unsafe {
			println!("------ PYRITE MEASURE [{}] ------", $index);
			println!("Last Time: {}ns", ::util::PYRITE_MEASURES[$index].1);
			println!("Total Samples: {}", ::util::PYRITE_MEASURES[$index].2);
			println!("Total Time: {}ns", ::util::PYRITE_MEASURES[$index].3);
			if ::util::PYRITE_MEASURES[$index].2 > 0 {
				println!("Avg. Time: {:.3}ns", (::util::PYRITE_MEASURES[$index].3 as f64) / (::util::PYRITE_MEASURES[$index].2 as f64));
			}
		}
	)
}

macro_rules! set_pyrite_dyn_debug {
	($value:expr) => (
		unsafe {
			::util::PYRITE_DYN_DEBUG_ENABLED = $value;
		}
	)
}

macro_rules! pyrite_debugging {
	($b:block) => (
		if unsafe {::util::PYRITE_DYN_DEBUG_ENABLED} {
			$b
		}
	);

	($b:block, $c:block) => (
		if unsafe {::util::PYRITE_DYN_DEBUG_ENABLED} {
			$b
		} else {
			$c
		}
	);
}

#[macro_export]
macro_rules! debug_print {
	($($x:expr),*) => (
		print!("[DEBUG] [{}:{}] ", file!(), line!());
		$(
			print!(stringify!($x));
			print!(" = {}; ", $x);
		)*
		println!("");
	)
}

#[macro_export]
macro_rules! debug_println {
	($($x:expr),*) => (
		println!("[DEBUG] [{}:{}] ", file!(), line!());
		$(
			print!("\t");
			print!(stringify!($x));
			println!(" = {}; ", $x);
		)*
	)
}

macro_rules! pd_println {
	($($x:expr),*) => (
		pyrite_debugging!({
			println!("[DEBUG] [{}:{}] ", file!(), line!());
			$(
				print!("\t");
				print!(stringify!($x));
				println!(" = {}; ", $x);
			)*
		});
	)
}

const RANDOM_FUCKING_COLORS: [(u8, u8, u8); 256] = [
	(0xEF, 0x53, 0x50), (0xCF, 0xD8, 0xDC), (0x75, 0x75, 0x75), (0x30, 0x4F, 0xFE), (0x00, 0x91, 0xEA),
	(0xFF, 0xB3, 0x00), (0x64, 0xB5, 0xF6), (0x00, 0x69, 0x5C), (0x38, 0x8E, 0x3C), (0x15, 0x65, 0xC0),
	(0x79, 0x86, 0xCB), (0xD8, 0x43, 0x15), (0x1A, 0x23, 0x7E), (0xEA, 0x80, 0xFC), (0xF0, 0xF4, 0xC3),
	(0xB2, 0xEB, 0xF2), (0xE5, 0x73, 0x73), (0xFF, 0x70, 0x43), (0x9E, 0x9E, 0x9E), (0xFF, 0xD7, 0x40),
	(0xD7, 0xCC, 0xC8), (0xAA, 0x00, 0xFF), (0xFF, 0xD6, 0x00), (0x95, 0x75, 0xCD), (0x29, 0xB6, 0xF6),
	(0xAE, 0xD5, 0x81), (0x00, 0xE6, 0x76), (0xF5, 0xF5, 0xF5), (0xFB, 0xE9, 0xE7), (0xB3, 0x9D, 0xDB),
	(0xA5, 0xD6, 0xA7), (0xC0, 0xCA, 0x33), (0x00, 0x83, 0x8F), (0xAD, 0x14, 0x57), (0xE3, 0xF2, 0xFD),
	(0xF9, 0xFB, 0xE7), (0xFF, 0xCC, 0x80), (0xDC, 0xE7, 0x75), (0xEC, 0xEF, 0xF1), (0xF4, 0x8F, 0xB1),
	(0xF0, 0x62, 0x92), (0xFF, 0xEB, 0x3B), (0xE0, 0x40, 0xFB), (0xAF, 0xB4, 0x2B), (0x29, 0x62, 0xFF),
	(0x00, 0xB0, 0xFF), (0xFD, 0xD8, 0x35), (0x45, 0x27, 0xA0), (0xFF, 0x6D, 0x00), (0x8C, 0x9E, 0xFF),
	(0xFF, 0x6E, 0x40), (0xCD, 0xDC, 0x39), (0xEF, 0x6C, 0x00), (0x45, 0x5A, 0x64), (0xFB, 0xC0, 0x2D),
	(0x9C, 0xCC, 0x65), (0x33, 0x69, 0x1E), (0x76, 0xFF, 0x03), (0xD4, 0xE1, 0x57), (0xC2, 0x18, 0x5B),
	(0xFF, 0x80, 0xAB), (0x81, 0xC7, 0x84), (0xFF, 0x57, 0x22), (0x55, 0x8B, 0x2F), (0x81, 0xD4, 0xFA),
	(0xE0, 0xF2, 0xF1), (0x00, 0x96, 0x88), (0xA7, 0xFF, 0xEB), (0x00, 0xC8, 0x53), (0x3F, 0x51, 0xB5),
	(0xEF, 0xEB, 0xE9), (0xFF, 0x52, 0x52), (0xF1, 0xF8, 0xE9), (0xE0, 0xE0, 0xE0), (0xBF, 0x36, 0x0C),
	(0xC5, 0xE1, 0xA5), (0x64, 0xDD, 0x17), (0xB2, 0xFF, 0x59), (0xA1, 0x88, 0x7F), (0xE0, 0xF7, 0xFA),
	(0x19, 0x76, 0xD2), (0xD3, 0x2F, 0x2F), (0xFF, 0xEB, 0x3B), (0x4C, 0xAF, 0x50), (0xE6, 0x51, 0x00),
	(0xEC, 0x40, 0x7A), (0x9F, 0xA8, 0xDA), (0xFF, 0xEC, 0xB3), (0x00, 0xBC, 0xD4), (0xFF, 0xC1, 0x07),
	(0xF3, 0xE5, 0xF5), (0x37, 0x47, 0x4F), (0x00, 0xE5, 0xFF), (0x28, 0x35, 0x93), (0x3F, 0x51, 0xB5),
	(0x00, 0x89, 0x7B), (0xFF, 0xEA, 0x00), (0xE1, 0xF5, 0xFE), (0xE9, 0x1E, 0x63), (0xD5, 0x00, 0x00),
	(0x39, 0x49, 0xAB), (0x80, 0xCB, 0xC4), (0x5C, 0x6B, 0xC0), (0xCE, 0x93, 0xD8), (0x1D, 0xE9, 0xB6),
	(0xC5, 0x11, 0x62), (0x69, 0xF0, 0xAE), (0x00, 0x4D, 0x40), (0x88, 0x0E, 0x4F), (0x79, 0x55, 0x48),
	(0xFF, 0xAB, 0x91), (0x82, 0xB1, 0xFF), (0x18, 0xFF, 0xFF), (0x4A, 0x14, 0x8C), (0xFB, 0x8C, 0x00),
	(0x03, 0xA9, 0xF4), (0xFF, 0xEB, 0xEE), (0xFF, 0x6F, 0x00), (0x4C, 0xAF, 0x50), (0x4E, 0x34, 0x2E),
	(0xE1, 0xBE, 0xE7), (0xBC, 0xAA, 0xA4), (0xB3, 0x88, 0xFF), (0x7C, 0x4D, 0xFF), (0x7C, 0xB3, 0x42),
	(0x90, 0xCA, 0xF9), (0xB3, 0xE5, 0xFC), (0x1E, 0x88, 0xE5), (0xFF, 0x91, 0x00), (0x43, 0xA0, 0x47),
	(0xFF, 0xE0, 0xB2), (0x61, 0x61, 0x61), (0xE5, 0x39, 0x35), (0x54, 0x6E, 0x7A), (0x84, 0xFF, 0xFF),
	(0xD5, 0x00, 0xF9), (0xFF, 0x17, 0x44), (0x51, 0x2D, 0xA8), (0x00, 0x97, 0xA7), (0x9C, 0x27, 0xB0),
	(0x8B, 0xC3, 0x4A), (0xFF, 0xA0, 0x00), (0xD1, 0xC4, 0xE9), (0x21, 0x21, 0x21), (0x6D, 0x4C, 0x41),
	(0x67, 0x3A, 0xB7), (0xFF, 0xCA, 0x28), (0xFF, 0xC1, 0x07), (0xFF, 0xF3, 0xE0), (0x03, 0x9B, 0xE5),
	(0x42, 0x42, 0x42), (0x26, 0xC6, 0xDA), (0xC5, 0xCA, 0xE9), (0xC6, 0xFF, 0x00), (0xE8, 0xEA, 0xF6),
	(0x4F, 0xC3, 0xF7), (0x00, 0xAC, 0xC1), (0xFF, 0xA7, 0x26), (0x66, 0xBB, 0x6A), (0x02, 0x77, 0xBD),
	(0x5D, 0x40, 0x37), (0xFF, 0x8A, 0x80), (0xF4, 0x51, 0x1E), (0xDC, 0xED, 0xC8), (0xFF, 0xEE, 0x58),
	(0xEE, 0xFF, 0x41), (0xFC, 0xE4, 0xEC), (0x03, 0xA9, 0xF4), (0x80, 0xDE, 0xEA), (0x00, 0xB8, 0xD4),
	(0x4D, 0xB6, 0xAC), (0xFF, 0x57, 0x22), (0x62, 0x00, 0xEA), (0xF5, 0x7F, 0x17), (0xFF, 0x3D, 0x00),
	(0xF4, 0xFF, 0x81), (0x02, 0x88, 0xD1), (0x21, 0x96, 0xF3), (0xEF, 0x9A, 0x9A), (0xB0, 0xBE, 0xC5),
	(0xED, 0xE7, 0xF6), (0x00, 0xBC, 0xD4), (0xDD, 0x2C, 0x00), (0x2E, 0x7D, 0x32), (0x29, 0x79, 0xFF),
	(0xFF, 0xC4, 0x00), (0xE6, 0xEE, 0x9C), (0x4D, 0xD0, 0xE1), (0xFF, 0xE5, 0x7F), (0x00, 0xBF, 0xA5),
	(0x26, 0xA6, 0x9A), (0xFF, 0xF5, 0x9D), (0x60, 0x7D, 0x8B), (0xFF, 0x98, 0x00), (0xFF, 0xD1, 0x80),
	(0x6A, 0x1B, 0x9A), (0x21, 0x96, 0xF3), (0xF5, 0x7C, 0x00), (0x82, 0x77, 0x17), (0xAB, 0x47, 0xBC),
	(0x68, 0x9F, 0x38), (0xFF, 0xAB, 0x00), (0xBB, 0xDE, 0xFB), (0x8D, 0x6E, 0x63), (0xC6, 0x28, 0x28),
	(0xF4, 0x43, 0x36), (0xFF, 0xAB, 0x40), (0x00, 0x79, 0x6B), (0xCC, 0xFF, 0x90), (0xFF, 0xFF, 0x00),
	(0xB2, 0xDF, 0xDB), (0x60, 0x7D, 0x8B), (0x01, 0x57, 0x9B), (0xF5, 0x00, 0x57), (0xFA, 0xFA, 0xFA),
	(0x0D, 0x47, 0xA1), (0x8B, 0xC3, 0x4A), (0x00, 0x60, 0x64), (0xC8, 0xE6, 0xC9), (0x1B, 0x5E, 0x20),
	(0xB9, 0xF6, 0xCA), (0xFF, 0x9E, 0x80), (0x67, 0x3A, 0xB7), (0xFF, 0xCC, 0xBC), (0x9E, 0x9D, 0x24),
	(0x80, 0xD8, 0xFF), (0xFF, 0xFF, 0x8D), (0x30, 0x3F, 0x9F), (0x8E, 0x24, 0xAA), (0xB7, 0x1C, 0x1C),
	(0x00, 0x96, 0x88), (0xFF, 0xB7, 0x4D), (0xFF, 0xFD, 0xE7), (0x5E, 0x35, 0xB1), (0x53, 0x6D, 0xFE),
	(0x44, 0x8A, 0xFF), (0x31, 0x1B, 0x92), (0x65, 0x1F, 0xFF), (0xFF, 0xF1, 0x76), (0x64, 0xFF, 0xDA),
	(0x7B, 0x1F, 0xA2), (0xE6, 0x4A, 0x19), (0x40, 0xC4, 0xFF), (0x9E, 0x9E, 0x9E), (0x78, 0x90, 0x9C),
	(0xFF, 0x40, 0x81), (0x9C, 0x27, 0xB0), (0xFF, 0xF9, 0xC4), (0xFF, 0xD5, 0x4F), (0xBD, 0xBD, 0xBD),
	(0xCD, 0xDC, 0x39), (0xFF, 0x8A, 0x65), (0x7E, 0x57, 0xC2), (0x90, 0xA4, 0xAE), (0xFF, 0x8F, 0x00),
	(0xE9, 0x1E, 0x63)
];

pub fn rand_color(n: usize, a: u8) -> (u8, u8, u8, u8) {
	let c = RANDOM_FUCKING_COLORS[n & 255];
	return (c.0, c.1, c.2, a);
}

macro_rules! rand_color {
    ($n:expr) => ( ::util::rand_color(($n) as usize, 255) );
    ($n:expr, $a:expr) => ( ::util::rand_color(($n) as usize, $a) );
}

macro_rules! print_memory_table {
	($memory:expr, $start:expr, $end:expr, $columns:expr) => ({
		let columns = $columns;

		let mut cc = 0;
		let mut char_rep: [char; $columns] = ['.'; $columns];

		for addr in $start..($end + 1) {
			if cc == 0 {
				println!("");
				print!("{:08x}", addr);
			}

			let data = $memory.read8(addr);
			print!(" {:02x}", data);
			char_rep[cc] = data as char;

			cc += 1;
			if cc >= columns {
				print!(" ");
				for cidx in 0..$columns {
					let cr = char_rep[cidx];
					if (cr as u8) < 32 { print!("."); }
					else { print!("{}", cr); }
				}
				cc = 0;
			}
		}

		if cc > 0 {
			print!(" ");
			for cidx in 0..min!(cc, $columns) {
				let cr = char_rep[cidx];
				if (cr as u8) <= 32 { print!("."); }
				else { print!("{}", cr); }
			}
		}

		println!("");
	});
	($memory:expr, $start:expr, $end:expr) => ( print_memory_table!($memory, $start, $end, 16) );
}


pub static mut PYRITE_COUNTERS: [(u64); 16] = [(0u64); 16];

macro_rules! pyrite_counter_inc {
	($counter:expr) => ( pyrite_counter_inc!($counter, 1) );
	($counter:expr, $amt:expr) => (
		unsafe {
			::util::PYRITE_COUNTERS[$counter] += $amt;
		}
	);
}

macro_rules! pyrite_counter_dec {
	($counter:expr) => ( pyrite_counter_dec!($counter, 1) );
	($counter:expr, $amt:expr) => (
		unsafe {
			::util::PYRITE_COUNTERS[$counter] -= $amt;
		}
	);
}

macro_rules! pyrite_counter_set {
	($counter:expr, $value:expr) => (
		unsafe {
			::util::PYRITE_COUNTERS[$counter] = $value;
		}
	);
}

macro_rules! pyrite_counter_get {
	($counter:expr) => (
		unsafe {
			::util::PYRITE_COUNTERS[$counter]
		}
	)
}