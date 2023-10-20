fn main(){
	let i:f64 = 210_000.00;
	let r:f64 = 5.0;
	let n:f64 = 3.0;

	//depreciation
	let d = i * (1.0 - (r/100.0)).powf(n);

	//value
	let v = i-d;
	println!("value is {}", v);


}