use std::path::Path;

use crate::data::BenchMark;

pub mod data;
pub mod graph;
pub mod partition;

fn main() {
    let data = Path::new("benchmark_data\\request_time_Large_nginx.csv");

    println!("{:?}", &data);

    let _data: BenchMark = data.try_into().unwrap();
    println!("{:?}", &_data);
}
