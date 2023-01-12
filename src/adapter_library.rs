use serde::Deserialize;
use serde_json::Result;
use std::collections::HashMap;


#[derive(Deserialize, Debug)]
struct Adapter {
    adapter_name: String,
    adapter_code: String,
    top_strand: String,
    bottom_strand: Option<String>,
}

#[derive(Deserialize, Debug)]
struct Barcode {
    component: String,
    forward_sequence: String,
    reverse_sequence: Option<String>,
}

#[derive(Deserialize, Debug)]
struct SequencingKit {
    kit_name: String,
    kit_code: String,
    barcodes: Option<Vec<Barcode>>,
    adapters: Option<Vec<Adapter>>,
}

#[derive(Deserialize, Debug)]
pub struct AdapterCatalog {
    document_title: String,
    document_authors: String,
    date_copied: String,
    sequencing_kits: Vec<SequencingKit>,
}

impl AdapterCatalog {

    pub fn info(&self) {
        println!("Adaptor catalog [{}] containing information from [{}] kits", self.document_title, self.sequencing_kits.len());
        println!("\tdated [{}] - by [{}]", self.date_copied, self.document_authors);
    }


    pub fn sanity_check(&self) -> std::result::Result<bool, String> {
        self.info();
        let mut kit_map: HashMap<String, u32> = HashMap::new();
        let mut error_str: Option<String> = None;

        for kit in self.list_kits() {
            if self.contains_kit(&kit) {
                if !kit_map.contains_key(&kit) {
                    let _ = kit_map.insert(kit.clone(), 0);
                }
                // now add the value ...
                *kit_map.entry(kit.to_owned()).or_default() += 1;
            }
        }

        for (key, value) in &kit_map {
            println!("{} / {}", key, value);
            if value > &1 {
                let error_message = format!("kit [{}] appears to have redundant entries - appears {} times", &key, &value);
                if error_str.is_none() {
                    error_str = Some(error_message);
                } else {
                    let message = error_str.clone().unwrap() + "\n.      " + error_message.as_str();
                    let _ = error_str.insert(message);
                }
            }
        }

        if error_str.is_some() && error_str.clone().unwrap().len() > 0 {
            return Err(String::from(error_str.clone().unwrap()));
        }
        return Ok(true);
    }

    pub fn list_kits(&self) -> Vec<String> {
        let mut vector: Vec<String> = Vec::new();
        for val in self.sequencing_kits.iter() {
            vector.push(val.kit_code.clone());
        }
        return vector;
    }



    pub fn contains_kit(&self, key: &String) -> bool {
        return self.list_kits().contains(&key);
    }
}


pub fn load_intrinsic_library() -> Option<AdapterCatalog> {
    println!("Trying to parse internal JSON adapter information");
    let my_str = include_str!("sequences.json");

    let p: Result<AdapterCatalog> = serde_json::from_str(&my_str);
    
    match p {
        Ok(v) => return Some(v),
        Err(e) => println!("Error {}", e)
    }

    return None;
}


pub fn load_extrinsic_library(fp: String) -> Option<AdapterCatalog> {
    println!("File [{}] to be parsed for JSON adapter information", fp);

    // check that the file specified exists

    // load text from file

    // convert text to JSON

    // evaluate errors

    /* 
    let p: Result<AdapterCatalog> = serde_json::from_str(&my_str);
    
    match p {
        Ok(v) => println!("ok"),
        Err(e) => println!("Error {}", e)
    }
    */
    return None;
}


