// Rust Program When Pattern is Matched
fn main() {
	
	let gfg = ("helo","world");
	if let ("G", "f") = gfg {
		println!("Pattern matched with scrutinee expression");
	} else {
		println!("Pattern does not match");
	}
}
