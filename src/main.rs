mod cmdline;
mod adapter_scan;
mod adapter_library;

//static mut _pawchop: Option<Result<cmdline::PawchopCmd, String>> = None;


fn main() {
    let pawchop: Result<cmdline::PawchopCmd, String> = cmdline::parse_cmd_line();

    match pawchop {
        Ok(pc)  => run_pawchop(pc),
        Err(e) => error(e),
    };

}

fn error(e: String) {
    println!("Error: {}\n", e);
    cmdline::PawchopCmd::show_help();
}


fn load_adapters(pawchop: &cmdline::PawchopCmd) -> Option<adapter_library::AdapterCatalog> {
    println!("loading adapter sequences");
    let ext_library = pawchop.has_mono_field(String::from("library"));

    if ext_library.is_some() {
        adapter_library::load_extrinsic_library(ext_library.unwrap())
    } else {
        adapter_library::load_intrinsic_library()
    }
}

fn run_pawchop(pawchop: cmdline::PawchopCmd) {
    
    if pawchop.has_flag(String::from("help")) {
        println!("!!!HELP");
        cmdline::PawchopCmd::show_help();
    } else if pawchop.has_flag(String::from("version")) {
        println!("!!!VERSION");
        cmdline::PawchopCmd::show_version();

    } else {
        println!("Pawchop has been parsed");

        let adapters = load_adapters(&pawchop);
        if adapters.is_none() {
            return;
        }
        let adapter_info = &adapters.unwrap();
        let sanity = adapter_info.sanity_check();
        
        /*
        match sanity {
            Ok(_pc)  => println!("looks sane ..."),
            Err(e) => println!("Error: {}\n", e),
        }; */

        if !sanity.is_ok() {
            println!("Error: {}\n", sanity.err().unwrap());
            return;
        }

        let ascan = adapter_scan::scan_for_adapter_sequences(&pawchop, &adapter_info);
        match ascan {
            Ok(_c)  => println!("scan OK"),
            Err(e) => println!("Error: {}\n", e),
        };
    }
}