fn main() {
	Let p:f64 = 520_000_000.0;
	Let r:f64 = 10.0;
	Let n:f64 = 5.0;

	//Compound interest
	Let a = p * ( 1.0 +(r / 100.0)) * n;
	println!("Amount is {}", a);
	Let cI = a - p;
	println!("Compound Interest is {}", cI);
}