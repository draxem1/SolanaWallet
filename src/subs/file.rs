pub mod serve{
use std::io::Read;
use std::fs::File;
use std::error::Error;
use std::fs;

	fn open_file(file: &str) -> Result<String, Box<dyn Error>> {
	    let mut file = File::open(file)?;
	    let mut contents = String::new();

	    file.read_to_string(&mut contents).expect("No Value to Return!!!");

	    Ok(contents)
	}

	pub fn get_state() -> u32{
		let file = "frame.txt";

		let content;
		loop {
			content = match open_file(&file){
	            Ok(s) => String::from(s),
	            Err(_) => { 
	                File::create(file).expect("Failed to create file");
	                fs::write(file, b"1").expect("Cannot Write To File!!"); 
	                continue },
	        };
	        break;
    	}

    	let result: u32 = content.trim().parse().expect("Can't convert to number!!");

    	result
	}

	pub fn send_state(x: &str) {
		let file = "frame.txt";
    	fs::write(file, x).expect("Unable to write data");
	}	
}