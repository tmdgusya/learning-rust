fn main() {
	let mut number = 3;
	
	while number != 0 {
		println!("number: {}", number);
		number -= 1;
	}
	
	iter_i();
}

fn iter_i() {
	let a = [1, 2, 3, 4, 5];

	for ele in a.iter() {
		println!("{}", ele);
	}
}
