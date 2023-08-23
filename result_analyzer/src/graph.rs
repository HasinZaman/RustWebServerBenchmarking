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
    //SVG stuff
    svg_size: [f32; 2],
    graph_size: [f32; 2],

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
    draw: HashMap<String, Box<dyn DataRender>>,
}

impl Default for GraphBuilder {
    fn default() -> Self {
        Self {
            svg_size: [800.0, 400.0],
            graph_size: [800.0 * 0.9, 400.0 * 0.9],

            palette: ColourPalette::default(),

            major: (1, 1),
            minor: None,

            left_axis: None,
            right_axis: None,
            bottom_axis: None,
            top_axis: None,

            draw: HashMap::new(),
        }
    }
}

macro_rules! setter {
    ($var_name:ident, $var_type:ty ) => {
        pub fn $var_name(mut self, input: $var_type) -> Self {
            self.$var_name = input;

            self
        }
    };
    ($var_name:ident, Some = $var_type:ty, None = ($default_x:expr, $default_y:expr) ) => {
        pub fn $var_name(mut self, input: $var_type) -> Self {
            self.$var_name = match input {
                ($default_x, $default_y) => None,
                _ => Some(input),
            };
            self
        }
    };
    ($var_name:ident, Some = $var_type:ty, None = $default_none:expr ) => {
        pub fn $var_name(mut self, input: $var_type) -> Self {
            self.$var_name = match input {
                $default_none => None,
                _ => Some(input),
            };
            self
        }
    };
    ($var_name:ident, Some = $var_type:ty) => {
        pub fn $var_name(mut self, input: $var_type) -> Self {
            self.$var_name = Some(input);

            self
        }
    };
}

impl GraphBuilder {
    //setters
    setter!(svg_size, [f32; 2]);
    setter!(graph_size, [f32; 2]);

    // colour palette
    setter!(palette, ColourPalette);

    // axis lines
    setter!(major, (u32, u32));

    setter!(minor, Some = (u32, u32), None = (0u32, 0u32));

    // axis info
    setter!(left_axis, Some = AxisInfo);
    setter!(right_axis, Some = AxisInfo);
    setter!(bottom_axis, Some = AxisInfo);
    setter!(top_axis, Some = AxisInfo);

    // data drawers
    setter!(draw, HashMap<String, Box<dyn DataRender>>);
    pub fn add_drawer(mut self, name: String, drawer: Box<dyn DataRender>) -> Self {
        self.draw.insert(name, drawer);

        self
    }

    // draw
    fn tuple_flatten<T1, T2, T3>(v: (T1, (T2, T3))) -> (T1, T2, T3) {
        (v.0, v.1 .0, v.1 .1)
    }

    fn create_grid(
        graph_size: &[f32; 2],
        offset: &[f32; 2],
        lines: &[u32; 2],
        colour: Color,
        width: f32,
    ) -> Box<dyn Node> {
        let mut element = Group::new();

        let iter = graph_size
            .iter()
            .zip(lines)
            .enumerate()
            .map(GraphBuilder::tuple_flatten);

        for (i1, size, lines) in iter {
            let delta: f32 = size / *lines as f32;

            let (mut start, mut end) = match i1 {
                0 => ([0.0, offset[1]], [0.0, graph_size[1] + offset[1]]),
                1 => ([offset[0], 0.0], [offset[0] + graph_size[0], 0.0]),
                _ => panic!(),
            };

            for i2 in 0..=*lines {
                start[i1] = offset[i1] + i2 as f32 * delta;
                end[i1] = offset[i1] + i2 as f32 * delta;

                element.append(
                    Line::new()
                        //start
                        .set("x1", start[0].to_string())
                        .set("y1", start[1].to_string())
                        //end
                        .set("x2", end[0].to_string())
                        .set("y2", end[1].to_string())
                        //colour
                        .set("stroke", colour.rgb())
                        .set(
                            "stroke-width",
                            match 0.0 <= width {
                                true => width,
                                false => panic!("Width must be greater than 0"),
                            },
                        ),
                );
            }
        }

        Box::new(element)
    }

    pub fn draw_svg(&self) {
        let offset: [f32; 2] = {
            let mut offset: [f32; 2] = [0.0, 0.0];

            self.svg_size
                .iter()
                .zip(self.graph_size.iter())
                .map(|(g, s)| (g - s) / 2.0)
                .enumerate()
                .for_each(|(index, val)| offset[index] = val);

            offset
        };

        let document = Document::new().set("viewBox", (0, 0, self.svg_size[0], self.svg_size[1]));
        //create background
        let document = document.add(
            Rectangle::new()
                .set("x", 0)
                .set("y", 0)
                .set("width", self.svg_size[0])
                .set("height", self.svg_size[1])
                .set("fill", self.palette.background.rgb()),
        );
        //create graph background
        let document = match self.minor {
            Some(minor) => document.add(GraphBuilder::create_grid(
                &self.graph_size,
                &offset,
                &[minor.0 * self.major.0, minor.1 * self.major.1],
                self.palette.minor_line,
                1.0,
            )),
            None => document,
        };

        let document = document.add(GraphBuilder::create_grid(
            &self.graph_size,
            &offset,
            &[self.major.0, self.major.1],
            self.palette.major_line,
            3.0,
        ));

        let document = document.add(
            Rectangle::new()
                .set("x", offset[0])
                .set("y", offset[1])
                .set("width", self.graph_size[0])
                .set("height", self.graph_size[1])
                .set("stroke", self.palette.major_line.rgb())
                .set("stroke-width", 5)
                .set("fill", "none"),
        );

        //create
        svg::save("image.svg", &document).unwrap();
        todo!();
        }
    }
}
