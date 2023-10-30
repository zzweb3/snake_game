use snake_game;

fn main() {
	//
	let mut message = String::from("Hello");
	message = extend_message(message);
	//println!("{}", message);
	//
	let age = 30;
	extene_age(age);
	//println!("{}", age);
	//
	let tup= (10, 30.2, '9', "xxx");
	let tup_1 = extend_tup(tup);
	println!("{:?}", tup);
	println!("{:?}", tup_1);

	let ss = "xxx555果果";
	let ss1 = ss;
	println!("{}", ss);
	println!("{}",ss1);

}

fn extend_message(mut a: String) -> String{
	a.push_str(" World.");
	a
}

fn extene_age(mut a: u32) {
	a += 100;
} 

fn extend_tup(mut tup : (i32, f64, char, &str)) -> (i32, f64, char, &str) {
	tup.1 = 3.1415f64;
	tup.3 = "uuuu";
	tup
} 