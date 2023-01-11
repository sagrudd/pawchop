mod cmdline;

fn main() {
    let _pawchop = cmdline::parse_cmd_line();

    match _pawchop {
        Ok(pc)  => run_pawchop(pc),
        Err(e) => error(e),
    };

}

fn error(e: String) {
    println!("Error: {}\n", e);
    cmdline::PawchopCmd::show_help();
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
    }
}