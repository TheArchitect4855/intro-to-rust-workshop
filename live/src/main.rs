fn main() {
	let mut name = "Kurtis";
	println!("Hello, {}!", name);

	name = "Joe";
	println!("Hello, {}!", name);

	// Some(value)
	// None
	let option: Option<i32> = None;
	if option.is_some() {
		let value = option.unwrap();
		println!("Option: {}", value);
	} else {
		println!("Option has no value!");
	}

	let option = "Option!";
	println!("Option: {:?}", option);

	// Integer sizes: 8, 16, 32, 64, 128
	// Unsigned/Signed: i = signed, u = unsigned
	let num = 5; // 32-bit signed integer'
	let byte: u8 = 7;
	let really_big_number = 999999999i64;

	// Boolean
	let b = false; // There is also true
	println!("Some bool: {}", b);

	// Character
	let c = 'A'; // UTF-32 "Scalar Value"

	// Tuples
	let tuple = (4, '4', "Four");
	println!("Tuple: {:?}", tuple);

	let (n4, c4, s4) = tuple;

	// Arrays
	let array = [1, 2, 3, 4];
	let other_array = [5u8; 7];

	println!("Array: {:?}", array);
	println!("Other Array: {:?}", other_array);

	// Unit type
	// void* ptr = asodfmdfkm
	let nothing = ();
	println!("Nothing: {:?}", nothing);

	// Functions
	hello("Kurtis");
	println!("1 + 2 = {}", add(1, 2));

	// Control Flow
	if array[2] == other_array[0] {
		println!("The arrays are equal!");
	} else {
		println!("The arrays are not equal!");
	}

	for i in array {
		print!("{} ", i);
	}

	println!();

	for i in 5..10 {
		print!("{} ", i);
	}

	println!();

	let mut counter = 0;
	while counter < 10 {
		counter += 1;
		print!("{} ", counter);
	}

	println!();

	let mut counter = 0;
	let x = loop {
		counter += 1;
		print!("{} ", counter);
		if counter >= 10 {
			break counter;
		}
	};

	println!("Counter = {}", x);

	let y = if x == 0 {
		10
	} else if x == 5 {
		5
	} else {
		0
	};

	// Ownership
	let something = String::from("Hello!");
	take(something);

	let something = String::from("Borrowed hello!");
	borrow(&something);
	println!("We can still use something: {}", something);

	let mut x = 10;
	println!("x = {}", x);
	mutable_borrow(&mut x);
	println!("x = {}", x);

	let b = true;
	take_copy(b);
	println!("Bool can still be used: {}", b);

	let s = String::from("A clone string!");
	take(s.clone());
	println!("S can still be used: {}", s);

	// Slices
	// [1, 2, 3, 4]
	// Take slice: [1..3] -> &[2, 3]
	let array_part = &array[1..3];
	println!("Array part: {:?}", array_part);

	let whole_array = &array[..];
	println!("Whole array: {:?}", whole_array);
	println!("Array part[1] = {}", array_part[1]);

	// Error handling
	let res = maybe_could_go_wrong();
	match res {
		Ok(v) => println!("Everything is OK: {}", v),
		Err(e) => println!("Everything is wrong! {}", e),
	}

	this_must_be_five(9000);
}

fn maybe_could_go_wrong() -> Result<i32, String> {
	Ok(5)
}

fn hello(name: &str) {
	println!("Hello, {}!", name);
}

fn add(a: i32, b: i32) -> i32 {
	a + b
}

fn take_copy(b: bool) {
	println!("Bool is {}", b);
}

fn take(s: String) {
	println!("I am moved: {}", s);
} // s is freed here

fn borrow(s: &String) {
	println!("I am borrowed: {}", s);
} // s, a REFERENCE to a string, is freed. The string is not.

fn mutable_borrow(i: &mut i32) {
	*i = 5;
} // i, the MUTABLE REFERENCE to an integer, is freed. The actual integer is not.

fn this_must_be_five(i: i32) {
	if i != 5 {
		panic!("i wasn't 5! It was actually {}", i);
	}
}