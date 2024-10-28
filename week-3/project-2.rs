fn main() {
	//amount
	let t:f64 = 450_000.00;
	let m:f64 = 1_500_000.00;
	let h:f64 = 750_000.00;
	let d:f64 = 2_850_000.00;
	let a:f64 = 250_000.00;

	//qty
	let qt:f64 = 2.0;
	let qm:f64 = 1.0;
	let qh:f64 = 3.0;
	let qd:f64 = 3.0;
	let qa:f64 = 1.0;

	//sum and average
	Let s = ((t*qt)+(m*qm)+(h*qh)+(d*qd)+(a*qa));
	println!("Sum is {}",s );
	Let tq = (qt+qm+qh+qd+qa);
	println!("Total Quantity is {}",tq );
	Let a = (s/tq);
	println!("Average is {}",a );
}