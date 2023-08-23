use std::path::Path;

use color_art::Color;
use graph::GraphBuilder;

use crate::{data::BenchMark, graph::ColourPalette};

pub mod data;
pub mod graph;
pub mod partition;

fn main() {
    // let data = Path::new("benchmark_data\\request_time_Large_nginx.csv");

    // println!("{:?}", &data);

    // let _data: BenchMark = data.try_into().unwrap();
    // println!("{:?}", &_data);

    let graph = GraphBuilder::default()
        .palette(ColourPalette::default())
        .major((10, 10))
        .minor((5, 5));

    graph.draw_svg();
}
