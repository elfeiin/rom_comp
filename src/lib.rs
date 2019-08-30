fn parse(input: String) -> Option<isize> {
	
	// Remove invalid characters and convert from nominals
	// to numbers.
	let mut r_vec: Vec<isize> = vec!();
	for c in input.chars() {
		match c.to_ascii_lowercase() {
			'i' => r_vec.push(1),
			'v' => r_vec.push(5),
			'x' => r_vec.push(10),
			'l' => r_vec.push(50),
			'c' => r_vec.push(100),
			'd' => r_vec.push(500),
			'm' => r_vec.push(1000),
			_ => (),
		}
	}
	
	// If the next number is great than ths one,
	// then negate all the ones before the next,
	// stopping at the first too big number.
	for i in 0..r_vec.len() {
		if i+1 < r_vec.len() {
			if r_vec[i] < r_vec[i + 1] {
				let flip_from = r_vec[i];
				for j in (0..i+1).rev() {
					if r_vec[j] > flip_from {
						break;
					}
					r_vec[j] = -r_vec[j];
				}
			}
		}
	}
	
	// Add all the numbers together.
	let mut total = 0isize;
	for v in r_vec {
		total += v;
	}
	
	Some(total)
}