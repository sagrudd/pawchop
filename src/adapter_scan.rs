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



pub fn input_scan(filepath: &Path, adapterc: &AdapterCatalog, observed: &ObservedAdapters) {
    println!("parsing file [{filepath:?}]");
}

pub fn scan_directory(filepath: &Path, adapterc: &AdapterCatalog, observed: &ObservedAdapters, depth: u8) {
    for entry in filepath.read_dir().expect("read_dir call failed") {
        if let Ok(entry) = entry {
            let pstr = entry.path();
            let pp = pstr.as_path();
            println!("{:?}", pp);
            if pp.is_dir() && depth == 0 {
                println!("{:?} is a directory", pp);
                scan_directory(pp, adapterc, &observed, depth+1);
            } else {
                input_scan(pp, adapterc, &observed);
            }
        }


    }
    /* 
    if 1 > 0 {
        scan_directory(filepath, adapterc, observed, depth+1);

    }
    */
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
                scan_directory(pp, adapterc, &observed_adapters, 0);
            } else {
                input_scan(pp, adapterc, &observed_adapters);
            }
        } else {
            return Err(String::from(format!("specified file path [{}] does not exist", &file_str)));
        }
        //input_scan(file_input.unwrap(), adapterc, &observed_adapters);
    } else {
        return Err(String::from("This is an unlikely exception - filepath not specified"));
    }

    return Ok(observed_adapters);
}
