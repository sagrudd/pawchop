//use bio::io::fastq::{Reader, Record, FastqRead};
use rust_htslib::bgzf;
use crate::{cmdline, adapter_library::AdapterCatalog};
use std::{path::Path, fs::File, io::BufReader};

use fastq::{parse_path, Record};


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

    let gzipped = true;
    // let mut record = Record::new();

    if gzipped {
        // this is the bio:: approach with htslib decompression
        /* 
        let r1_reader = BufReader::new(bgzf::Reader::from_path(filepath).unwrap());
        let reader = Reader::new(r1_reader);

        for record in reader.records() {
            if record.is_ok() {
                println!("record {:?}", record.ok().expect("messed up FASTQ entry").id());
            }
        }
        //let mut freader = Reader::new(reader);
        */

        // this is the fastq library approach
        let parser = fastq::Parser::new(bgzf::Reader::from_path(filepath).unwrap());
        parser.each(|record| {
            println!("{}", String::from_utf8_lossy(record.head()));

            true
        }).expect("Invalid fastq file");
    }


    /* 
    let mut reader = Reader::from_file(filepath).expect("Unable to open");



    let mut record = Record::new();

    reader.read(&mut record).expect("reader fubar");

    println!("record {}", record.id()) */
    

    
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
