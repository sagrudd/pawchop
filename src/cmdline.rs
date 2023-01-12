use std::env;
use std::collections::HashMap;


// putting config information into a struct
pub struct PawchopCmd {
    parameters: HashMap<String, Vec<String>>,
    param_specs: Vec<(String, Option<char>, Option<String>, bool, u8, String, Option<String>)>,
    args: Vec<String>,
    orphans: Vec<String>,
    valid_params: bool,
    fubar: Option<String>,
}

impl PawchopCmd {
    
    fn init() -> PawchopCmd {
        PawchopCmd { 
        	parameters: HashMap::new(),
        	param_specs: PawchopCmd::get_params(),
        	args: env::args().collect(),
        	orphans: Vec::new(),
        	valid_params: true,
        	fubar: None,
        }
    }

    pub fn has_flag(&self, key: String) -> bool {
    	if self.parameters.contains_key(&key) &&
    		self.parameters.get(&key).clone().unwrap().len() == 0 {
    			return true;
    		}
    		return false;
    }


	pub fn has_mono_field(&self, key: String) -> Option<String> {
    	if self.parameters.contains_key(&key) &&
    		self.parameters.get(&key).clone().unwrap().len() == 1 {
				let vect = self.parameters.get(&key).clone().unwrap();
				return vect.get(0).cloned();
    		}
    		return None;
    }


	pub fn show_version() {
		println!("{}", "pawchop - porechop reimagined using rust");
		println!("{}", "----------------------------------------");
		println!("pawchop v{}", PawchopCmd::get_version());

	}

	pub fn show_help() {
		
		println!("{}", "pawchop - porechop reimagined using rust");
		println!("{}", "----------------------------------------");

		for item in PawchopCmd::get_params().iter() {
			println!("{:?}", item);
		}

		println!("pawchop v{}", PawchopCmd::get_version());
	}




    fn add_parameter(&mut self, key: &str, value: Option<&str>) {
    	if !&self.parameters.contains_key(key) {
    		let _ = &self.parameters.insert(String::from(key), Vec::new());
    	}
    	// now add the value ...
    	if value.is_some() {
	    	let vector = &mut self.parameters.get_mut(key).unwrap();
	    	// vector is a Some 
	    	vector.push(String::from(value.unwrap()));
    	}
    }

    /*
    fn log(&self) {
    	println!("logging parsed parameters ...");
    	for (key, value_vec) in &self.parameters {
    		println!("{key:?} has been assigned");
    		for value in value_vec {
    			println!("\twith value [{value}]");
    		}
		}
    }
	*/

    fn get_params() -> Vec<(String, Option<char>, Option<String>, bool, u8, String, Option<String>)> {
		/* what does parameters require?
			key = internal identifier
			character = short id e.g. -h
			string = long id e.g. --help
			boolean = required
			int u8 = number of parameters to consume
			string = description
			string = default value
		*/
		let mut parameters: Vec<(String, Option<char>, Option<String>, bool, u8, String, Option<String>)> = Vec::new();
		parameters.push((String::from("help"), Some('h'), Some(String::from("help")), false, 0, String::from("print help for method"), None)); 
		parameters.push((String::from("version"), Some('v'), Some(String::from("version")), false, 0, String::from("display version information"), None)); 
		parameters.push((String::from("subsample"), Some('s'), Some(String::from("subsample")), true, 1, String::from("The fraction of sequences to subsample during data review"), Some(String::from("0.05")))); 

		parameters.push((String::from("input"), Some('i'), Some(String::from("input")), true, 1, String::from("one or more input file(s) or directories"), None)); 
		parameters.push((String::from("library"), Some('l'), Some(String::from("library")), false, 1, String::from("JSON format file describing kits and their adapter and barcode sequences"), None)); 

		return parameters;
	}


	fn get_version() -> String {
		return String::from(env!("CARGO_PKG_VERSION"));
	}

	fn is_fully_named_parameter(&self, arg: &str) -> Option<String> {
		if arg.chars().count() >= 4 && &arg[..2] == "--" {
			return Some(String::from(&arg[2..]));
		}
		return None;
	}


	fn is_compound_abbreviated_parameter(&self, arg: &str) -> Option<String> {
		let mut valid = true;
		if arg.chars().count() > 2 && &arg[..1] == "-"  && &arg[1..2] != "-" {
			for i in arg[1..].chars() {
	    		if i == '-' {
	    			valid = false;
	    		}
			}
		} else {
			valid = false;
		}
		if valid {
			return Some(String::from(&arg[1..]));
		}
		return None;
	}


	fn is_atomic_abbreviated_parameter(&self, arg: &str) -> Option<String> {
		if arg.chars().count() == 2 && &arg[..1] == "-" && &arg[1..2] != "-" {
			return Some(String::from(&arg[1..]));
		}
		return None;
	}


	fn is_known_parameter(&self, field: &str) -> (Option<String>, Option<u8>) {
		for item in self.param_specs.iter() {
			if field.len() > 1 && item.2.is_some() && item.2.clone().unwrap() == field {
				return (Some(item.0.clone()), Some(item.4.clone()));
			} else if field.len() == 1 && item.1.is_some() && String::from(item.1.unwrap()) == field {
				return (Some(item.0.clone()), Some(item.4.clone()));
			} 
		}
		return (None, None);
	}



	fn is_a_parameter(&self, arg: &str) -> (bool, Option<String>, Option<String>) {
		let mut is_parameter = false;
		let mut parameter_type: Option<String> = None;
		let mut parameter_value: Option<String> = None; 

		let fnp: Option<String> = self.is_fully_named_parameter(arg);
	   	let aap: Option<String> = self.is_atomic_abbreviated_parameter(arg);
	    let cap: Option<String> = self.is_compound_abbreviated_parameter(arg);

		if fnp.is_some() {
			is_parameter = true;
			parameter_type = Some(String::from("fnp"));
			parameter_value = Some(String::from(&fnp.unwrap()));
		} else if aap.is_some() {
	    	is_parameter = true;
	    	parameter_type = Some(String::from("aap"));
	    	parameter_value = Some(String::from(&aap.unwrap()));
	    } else if cap.is_some() {
	    	is_parameter = true;
	    	parameter_type = Some(String::from("cap"));
	    	parameter_value = Some(String::from(&cap.unwrap()));
	    } 

	    return (is_parameter, parameter_type, parameter_value);
	}


	fn pick_parameter_values(&mut self, arg: &str, offset: usize) {
		//println!("looking for value(s) for {} from offset {}", arg, offset);
		let myslice = &self.args.clone()[offset+1..];
		//println!("{:?}", myslice);
		for x in myslice.iter() {
			let param_context = self.is_a_parameter(x);
	    	// test if the parameter is a parameter
	    	if !param_context.0 {
	    		self.add_parameter(&arg, Some(x));
	    	} else {
	    		return
	    	}
		}
	}


	fn process_compounded_atomic_parameter(&mut self, parameter_value: String) {
		// compound atomic parameters are a tricky beast! 
		//	-ont could be interpreted as -o -n -t or -o nt
		let mut is_parameter = false;
		for (ci, cx)  in parameter_value.chars().enumerate() {
			if !is_parameter {
				// there are two options - is this a flag or a parameter?
				let kp = self.is_known_parameter(&String::from(cx));
				if kp.0.is_some() && kp.1.is_some() { // obligate?
					if kp.1.unwrap() > 0 {
						// this must be a parameter
						is_parameter = true;
						self.add_parameter(&kp.0.unwrap(), Some(&parameter_value[ci+1..]));
					} else {
						// this is a known flag
						self.add_parameter(&kp.0.unwrap(), None);
					}
				} else {
					// looks like an orphan ...
					self.orphans.push(String::from(cx));
				}
			}
		}
	}


	fn scan_params(&mut self) {
		for (i, x) in self.args.clone().iter().enumerate() {
	    	let param_context = self.is_a_parameter(x);

	    	// test if the parameter is a parameter
	    	if param_context.0 {
	    		let parameter_type = param_context.1.unwrap();
	    		let parameter_value = param_context.2.unwrap();
	    		if parameter_type == "fnp" || parameter_type == "aap" {
	    			let kp = self.is_known_parameter(&parameter_value);
	    			if kp.0.is_some() {
		    			// println!("{} is fully qualified parameter", kp.clone().unwrap());
		    			if kp.1.unwrap() == 0 {
		    				// this is a known flag
							self.add_parameter(&kp.0.unwrap(), None);
		    			} else {
		    				self.pick_parameter_values(&kp.0.unwrap(), i);
		    			}
		    		} else {
						self.orphans.push(parameter_value);
		    		}
	    		} else if parameter_type == "cap" {
	    			self.process_compounded_atomic_parameter(parameter_value);
	    		}
	    	}
		}
	}


	fn log_orphans(&mut self) {
		if self.orphans.len() > 0 {
			for item in &self.orphans.clone() {
				let _ = &self.raise_error(String::from("[{")+
						item + 
						&String::from("}] is an undefined parameter")
				);
			}
		}
	}


	fn sanity_check(&mut self) {
		for item in self.param_specs.clone().iter() {
			let key = &item.0;
			let required = item.3;
			let multiplicity = item.4;
			let included = self.parameters.contains_key(key);
			let default = &item.6;

			if required && !included && default.is_some() {
				let _ = &self.add_parameter(key.as_str(), Some(default.clone().unwrap().as_str()));
			} else if required && !included && default.is_none() {
				// println!("{} is a required parameter", key);
				let _ = &self.raise_error(String::from("[{")+
						key + 
						&String::from("}] is a required parameter"));
			} else if included {
				let observed_fields = self.parameters.get(key).clone().unwrap().len();
				// println!("{} of {} fields observed", observed_fields, multiplicity);
				if observed_fields > multiplicity.into() || observed_fields < multiplicity.into() {
					let _ = &self.raise_error(
						format!("[{}] requires {:?} parameters with {:?} observed", key, &multiplicity, &observed_fields));
				}
			}
		}
	}


	fn raise_error(&mut self, error_message: String) {
		self.valid_params = false;
		if self.fubar.is_none() {
			self.fubar = Some(error_message);
		} else {
			let message = self.fubar.clone().unwrap() + "\n.      " + error_message.as_str();
			self.fubar = Some(message);
		}
	}

}


// create PawchopCmd struct of commandline parameters
pub fn parse_cmd_line() -> Result<PawchopCmd, String> {
    // read in command line parameters
    let mut cmdline_params = PawchopCmd::init();

    cmdline_params.scan_params();

    cmdline_params.sanity_check();
    
    cmdline_params.log_orphans();

    // cmdline_params.log();

    if cmdline_params.valid_params {
    	return Ok(cmdline_params);
	} else {
		return Err(cmdline_params.fubar.unwrap());
	}
}