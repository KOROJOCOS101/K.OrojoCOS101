fn main(){
	let p:f64 = 520000000.00;
	let r:f64 = 10.0;
	let t:f64 = 5.0;

	//Amount
	let a = p*{1.0+(r/100.0)}*t;
	println!("Amount is {}", a);

	//Compound Interest
	let c = a-p;
	println!("Compound interest is {}", c);







}