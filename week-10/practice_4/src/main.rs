fn main() {
    // a list of nos
    let v = vec![15, 25, 35, 45, 55];
    print_vector(v);
    println!("{}",v[0]); // this line gieves error
}

fn print_vector(x:Vec<i32>){

    println!("Inside print_vectorfunction{:?}",x);
}
