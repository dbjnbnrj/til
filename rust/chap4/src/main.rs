fn main() {
    let s = String::from("Hello, world!");
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
	if item == b' ' {
		print!("{} \n", &s[0..i]);
	}
    }
    print!("{} \n",&s[..]);
}
