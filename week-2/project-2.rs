fn main(){
	//Quantity

	let qt:f64 = 2.0;
	let qm:f64 = 1.0;
	let qhp:f64 = 3.0;
	let qd:f64 = 3.0;
	let qa:f64 = 1.0;

	//Amount

	let at:f64 = 450_000.0;
	let am:f64 = 1_500_000.0;
	let ahp:f64 = 750_000.0;
	let ad:f64 = 2_850_000.0;
	let aa:f64 = 250_000.0;

	//Sum
	let s = (qt*at)+(qm*am)+(qhp*ahp)+(qd*ad)+(qa*aa);
	println!("Sum is {}", s);

	//Average
	let a = ((qt*at)+(qm*am)+(qhp*ahp)+(qd*ad)+(qa*aa))/(qt+qm+qhp+qd+qa);
	println!("Average is {}", a);
}