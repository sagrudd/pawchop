use bio::io::fastq::{Reader, Record};
use edlib_rs::edlibrs::{edlibAlignRs, EdlibAlignConfigRs};
use rust_htslib::bgzf;
use crate::{cmdline, adapter_library::AdapterCatalog};
use std::{path::Path, io::BufReader};


pub struct ObservedAdapters {

}


impl ObservedAdapters {
    fn init() -> ObservedAdapters {
        ObservedAdapters {

        }
    }
}


pub fn scan_for_adapters(fastq: &Record, adapterc: &AdapterCatalog, observed: &ObservedAdapters) {
    //println!("record {:?}", fastq.id());

    //let seq = fastq.seq();
    let expected = std::str::from_utf8(fastq.seq()).unwrap();
    //println!("{:?}", expected);

    for kit in adapterc.list_kits() {
        let kit_obj = adapterc.get_kit(kit);
        if kit_obj.is_some() {
            let adapter_obj = kit_obj.unwrap().adapters;
            if adapter_obj.is_some() {
                for adapter in adapter_obj.unwrap() {
                    let align_res = edlibAlignRs(fastq.seq(), adapter.top_strand.as_bytes(), &EdlibAlignConfigRs::default());
                }
            }
        }
    }
}


pub fn scan_fastq_file(filepath: &Path, adapterc: &AdapterCatalog, observed: &ObservedAdapters) {
    println!("parsing file [{filepath:?}]");

    // this is the bio:: approach with htslib decompression
    let r1_reader = BufReader::new(bgzf::Reader::from_path(filepath).unwrap());
    let reader = Reader::new(r1_reader);

    for record in reader.records() {
        if record.is_ok() {
            let fastq_record = record.ok().expect("messed up FASTQ entry");
            scan_for_adapters(&fastq_record, adapterc, observed);
        }
    }
    
}

pub fn scan_directory(filepath: &Path, adapterc: &AdapterCatalog, observed: &ObservedAdapters, depth: u8) {

    let my_vec = vec!["fastq", "fq", "fastq.gz", "fq.gz"];

    for entry in filepath.read_dir().expect("read_dir call failed") {
        if let Ok(entry) = entry {
            let pstr = entry.path();
            let pp = pstr.as_path();
            println!("{:?}", pp);
            if pp.is_dir() && depth == 0 {
                println!("{:?} is a directory", pp);
                scan_directory(pp, adapterc, &observed, depth+1);
            } else {
                let mut file_processed = false;
                if pp.is_file() {
                    for v in &my_vec {
                        if !file_processed && pp.to_str().unwrap().to_ascii_lowercase().ends_with(v) {
                            file_processed = true;
                            scan_fastq_file(pp, adapterc, &observed);
                        }
                    }
                    
                }
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
                scan_fastq_file(pp, adapterc, &observed_adapters);
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
