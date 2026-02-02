use card::prelude::*;

fn main() {
	let v = vec![1, 2, 3];
	let hoge = v.iter().into_iter();

	for i in hoge {
		println!("{i}");
	}
}
