use std::io;

fn main() {
	let mut tasks: Vec<String> = Vec::new();
	
	loop {
		println!("\n Mini Todo CLI");
		println!("1.add");
		println!("2.exit");
		println!("Enter your choice");
		let mut choice = String::new();
		io::stdin().read_line(&mut choice).expect("Failed input");
		let choice = choice.trim();
		if choice == "1"{
			println!("Enter task:");
			let mut task = String::new();
			io::stdin().read_line(&mut task).expect("Failed task");
			
			let task = task.trim().to_string();
			tasks.push(task);
			
			println!("task added");
			println!("task:");

			for (i, t)in tasks.iter().enumerate(){println!("{}. {}",i + 1, t);}
		}else if choice == "2"{
			println!("Exit");
			break;
		}else {
			println!("try again");
		}		
	}
}
