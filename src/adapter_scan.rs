

pub struct ObservedAdapters {

}


impl ObservedAdapters {
    fn init() -> ObservedAdapters {
        ObservedAdapters{

        }
    }
}


pub fn scan_for_adapter_sequences() -> Result<ObservedAdapters, String> {
    let mut observed_adapters = ObservedAdapters::init();

    return Ok(observed_adapters);
}