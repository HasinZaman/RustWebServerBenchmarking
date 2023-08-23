use color_art::Color;

pub trait DataRender {
    fn render(&self, axis_range: (f32, f32), document: svg::Document);
}

pub struct ColourPalette {
    pub background: Color,

    pub axis: Color,

    pub major_line: Color,
    pub minor_line: Color,

    pub title: Color,
    pub unit: Color,
}

impl Default for ColourPalette {
    fn default() -> Self {
        Self {
            background: Color::from_rgb(255, 0, 0).unwrap(),
            axis: Color::from_rgb(0, 255, 0).unwrap(),
            major_line: Color::from_rgb(0, 0, 255).unwrap(),
            minor_line: Color::from_rgb(255, 255, 0).unwrap(),
            title: Color::from_rgb(255, 0, 255).unwrap(),
            unit: Color::from_rgb(0, 255, 255).unwrap(),
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
