fn main{
	let p:f64 = 510_000.0;
	let r:f64 = 5.0;
	let n:f64 = 3.0;

	//depreciation
	let a = p*(1.0 - (r / 100.0))^n;
	println!("Amount is {}",a );
	let d = a - p;
	println!("Depreciation is {}",d );

} 