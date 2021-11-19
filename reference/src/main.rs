// STRUCTS, ENUMS
#[derive(Debug)]
struct Struct {
	x: i32,
	y: bool,
}

enum Enum {
	Variant1,
	Variant2(String), // Enum variants can have values (tagged union) - This is how Option works.
}

// Option, but worse:
enum IntOption {
	Some(i32),
	None,
}

fn main() {
	// VARIABLES
	println!("VARIABLES");
	let x = 5;
	println!("x = {}", x);

	// Variables are immutable by default
	let mut y = 7; // y is declared as mutable
	println!("y = {}", y);

	y = 10;
	println!("y = {}", y);

	// Null doesn't exist (everything must have a value), so we can use Option<T> instead:
	let option: Option<i32> = None;
	println!("option = {:?}", option);

	// Variables can be redeclared with the same name.
	let option = Some(32); // NOTE: We are not setting option. We are declaring a new variable with the same name, overwriting the previous option.
	println!("option = {:?}", option);

	// DATA TYPES
	// Notice: None of our variables have a type declaration except for "option".
	// Types are inferred whenever possible.
	println!("DATA TYPES");

	// Numeric Types
	// Integer types are either signed or unsigned.
	// They are notated with iBITS or uBITS, for signed
	// and unsigned, respectively.
	// e.g.: i32; 32-bit signed integer. This is the default for type inference.
	let z = 5; // z is of type i32
	println!("z is an i32 and z = {}", z);

	// Alternatively,
	let w: u8 = 1; // w is an unsigned byte.
	println!("w is a u8 and w = {}", w);

	// All bit sizes:
	// 8, 16, 32, 64, 128

	// Exception: size type. Either isize or usize.
	// The size of this type is the pointer size of your computer.
	let v: usize = 42; // This is 64-bits on a 64-bit machine, 32-bits on a 32-bit machine, etc.
	println!("v is a usize and v = {}", v);

	// Floating point types are f32, f64. f64 is the default for type inference.
	let u = 3.14; // f64
	println!("u is an f64 and u = {}", u);

	// Boolean type (true or false)
	let b = true;
	let a: bool = false;
	println!("a = {}, b = {}", a, b);

	// Character type (UTF-32 scalar value)
	let c = 'カ';
	let d: char = 'Z';
	println!("c = {}, d = {}", c, d);

	// Tuples
	let tuple = (8, '8', "Eight");
	println!("Tuple: {:?}", tuple);

	let num8 = tuple.0;
	let char8 = tuple.1;
	let str8 = tuple.2;
	println!("num8 = {}, char8 = {}, str8 = {}", num8, char8, str8);

	let (n8, c8, s8) = tuple;
	println!("Deconstructed: ({}, {}, {})", n8, c8, s8);

	// Arrays
	let array = [1, 2, 3, 4];
	let other_array = [0; 5];
	println!("Array: {:?}, Other Array: {:?}", array, other_array);

	let two = array[1];
	let zero = other_array[3]; // All values in this array are zero
	println!("Array[1] = {}, Other Array[3] = {}", two, zero);

	// Unit type
	let nothing = (); // This is literally nothing. Basically equivalent to "void" in C/C++.
	println!("Nothing = {:?}", nothing);

	// FUNCTIONS
	println!("FUNCTIONS");
	let hello = hello();
	println!("Function says {}", hello);

	// CONTROL FLOW
	println!("CONTROL FLOW");

	// If
	if b {
		println!("🅱️"); // Your terminal might not support emojis, but they're allowed.
	}

	// For
	for elem in array {
		print!("{} ", elem);
	}

	println!();

	for i in 0..5 {
		print!("{} ", i);
	}

	println!();

	// While
	let mut i = 0;
	while i < 10 {
		i += 1;
		print!("{} ", i);
	}

	println!();

	// Loop
	let mut i = 0; // Redeclare i
	loop {
		i += 1;
		print!("{} ", i);
		if i >= 10 {
			break;
		}
	}

	println!();

	// You can also return values from control flow statements:
	let some = if option.is_some() {
		option.unwrap() // This panics (crashes the program) is option is none.
	} else {
		0
	};

	println!("Some: {}", some);

	// To return value from a loop:
	let five = loop {
		break 5; // NOTE: This is a pointless example.
	};

	println!("Five = {}", five);

	// OWNERSHIP
	println!("OWNERSHIP");

	let s = String::from("I get taken!");
	take(s); // s can no longer be used.

	// Borrowing
	// You can have however many immutable borrows as you want!
	let s = String::from("I get borrowed!");
	borrow(&s);
	println!("s can still be used: {}", s); // s is simply borrowed

	// Mutable borrowing
	// Only ONE mutable borrow is allowed at a time.
	let mut i = 5;
	// Clone & Copy
	// Stack types are "copy-able"
	copy(i); // i gets bassed by value and copied.
	println!("i can still be used: {}", i);

	// Heap types can implement clone
	let s = String::from("I get cloned!");
	take(s.clone()); // Clones s and passes ownership of the clone to take
	println!("s can still be used: {}", s);

	// Slices
	let array_slice = &array[2..4]; // Immutable reference into a "slice" of array
	println!("Slice of array: {:?}", array_slice);

	// STRUCTS, ENUMS, PATTERN MATCHING
	println!("STRUCTS, ENUMS, AND PATTERN MATCHING");

	let st = Struct { x: 5, y: true };
	println!("{:?}", st);

	let en = Enum::Variant1;

	// Pattern matching with match
	match en {
		Enum::Variant1 => println!("Variant 1!"),
		Enum::Variant2(value) => println!("Variant 2: {}", value),
	}

	// Pattern matching with if let
	let en = Enum::Variant2("Something!".to_string());
	if let Enum::Variant2(s) = en {
		println!("en is variant 2! Value: {}", s);
	}

	// ERROR HANDLING
	// Panic
	let something_bad = false;
	if something_bad {
		panic!("Something bad happened!"); // This will terminate the program with an error.
	}

	// Result
	let res = this_could_be_bad();
	match res {
		Ok(v) => println!("Everything was fine! Value: {}", v),
		Err(e) => println!("Something bad happened. Message: {}", e),
	}
}

// FUNCTIONS
fn hello() -> String {
	String::from("Hello!") // NOTE: A literal string (str) and a String are different.
}

fn take(s: String) {
	println!("{}", s);
} // s is dropped here

fn borrow(s: &str) {
	println!("{}", s);
}

fn mutable_borrow(i: &mut i32) {
	*i = 100;
}

fn copy(i: i32) {
	println!("Copy of i is {}", i);
}

fn this_could_be_bad() -> Result<i32, String> {
	Ok(5)
}