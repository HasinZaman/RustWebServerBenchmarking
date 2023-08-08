use std::path::Path;

use data::MemoryData;

pub mod data;


fn main() {
    let data = Path::new("benchmark_data\\memory_usage_Large_flask.csv");
    
    println!("{:?}", &data);

    let _data: MemoryData = data.try_into().unwrap();
    println!("{:?}", &_data);
}
