use crate::{cmdline, adapter_library::AdapterCatalog};
use std::path::Path;


pub struct ObservedAdapters {

}


impl ObservedAdapters {
    fn init() -> ObservedAdapters {
        ObservedAdapters {

        }
    }
}



pub fn input_scan(filepath: &String, adapterc: &AdapterCatalog, observed: &ObservedAdapters) {
    
}


pub fn scan_for_adapter_sequences(pawchop: &cmdline::PawchopCmd, adapterc: &AdapterCatalog) -> Result<ObservedAdapters, String> {
    println!("!scanning");
    let mut observed_adapters = ObservedAdapters::init();

    let file_input = pawchop.has_mono_field(String::from("input"));

    if file_input.is_some() {
        let file_str = file_input.unwrap();
        let pp = Path::new(file_str.as_str());
        if pp.exists() {
            if pp.is_dir() {
                println!("{} is a directory", &file_str);
            } else {

            }
        } else {
            return Err(String::from(format!("specified file path [{}] does not exist", &file_str)));
        }
        //input_scan(file_input.unwrap(), adapterc, &observed_adapters);
    } else {
        return Err(String::from("This is FUBAR"));
    }

    return Ok(observed_adapters);
}
