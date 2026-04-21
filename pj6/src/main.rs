fn main() {
    let number = [5,8,2,10,3];
	let mut max = number[0];
	
	for n in number{
		if n > max{
			max = n;			
		}
	}
	println!("Max = {}",max);
}
