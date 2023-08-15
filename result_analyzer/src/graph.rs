use color_art::Color;

pub trait DataRender {
    fn render(&self, axis_range: (f32, f32), document: svg::Document);
}

pub struct ColourPalette {
    background: Color,

    axis: Color,

    major_line: Color,
    minor_line: Color,

    title: Color,
    unit: Color,
}

impl Default for ColourPalette {
    fn default() -> Self {
        Self {
            background: todo!(),
            axis: todo!(),
            major_line: todo!(),
            minor_line: todo!(),
            title: todo!(),
            unit: todo!(),
        }
    }
}

pub struct AxisInfo {
    pub title: String,
    pub unit: String,

    pub range: (f32, f32),
    pub unit_parser: fn(f32) -> String,
}

pub struct GraphBuilder {
    // colour palette
    palette: ColourPalette,

    // Background info
    major: (u32, u32),
    minor: Option<(u32, u32)>,

    // Axis info
    left_axis: Option<AxisInfo>,
    right_axis: Option<AxisInfo>,
    bottom_axis: Option<AxisInfo>,
    top_axis: Option<AxisInfo>,

    // data drawers
    draw: Vec<Box<dyn DataRender>>,
}

impl Default for GraphBuilder {
    fn default() -> Self {
        Self {
            palette: ColourPalette::default(),

            major: (1, 1),
            minor: None,

            left_axis: None,
            right_axis: None,
            bottom_axis: None,
            top_axis: None,

            draw: Vec::new(),
        }
    }
}
