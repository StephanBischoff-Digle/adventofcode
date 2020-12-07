use serde::Deserialize;
use std::collections::BTreeMap;
use std::fs;
use svg::node::element::Circle;
use svg::node::element::Line;
use svg::node::element::Rectangle;
use svg::node::Text;
use svg::Document;

#[derive(Debug, Deserialize)]
enum State {
    Done,
    Started,
}

impl State {
    pub fn color(&self) -> &'static str {
        match self {
            Self::Done => "lime",
            Self::Started => "white",
        }
    }
}

#[derive(Debug, Deserialize)]
enum Part {
    A(State),
    B(State),
}

fn main() {
    // Some config stuff
    // TODO: read from config file to render more dynamic
    let border: i32 = 30;
    let linespacing: i32 = 20;
    let fontsize: i32 = 10;
    let mark_radius: i32 = (fontsize as f32 * 0.66).round() as i32;
    let column_space: i32 = 100;

    // Read data file
    let input = fs::read_to_string("data.yml").expect("Failed to read input file");
    let data: BTreeMap<u32, BTreeMap<usize, Vec<Part>>> = serde_yaml::from_str(&input).unwrap();

    ////////////////////////////////////////////////////////////////////////
    // Setup SVG root element
    let mut document = Document::new()
        .set(
            "viewBox",
            (
                0,
                0,
                border + data.len() as i32 * column_space,
                border + (fontsize + linespacing) * 25 + border,
            ),
        )
        .set("width", border + data.len() as i32 * column_space)
        .set("height", border + (fontsize + linespacing) * 25 + border);

    // Set background to white
    document = document.add(
        Rectangle::new()
            .set("width", "100%")
            .set("height", "100%")
            .set("fill", "white"),
    );

    ////////////////////////////////////////////////////////////////////////
    // Render data content
    for (i, (year, days)) in data.iter().enumerate() {
        let i = i as i32;

        let year_txt = svg::node::element::Text::new()
            .add(Text::new(format!("{}", year)))
            .set("x", 2 * border + i * column_space)
            .set("y", 20)
            .set("text-anchor", "middle")
            .set("font-size", 2 * fontsize);
        document = document.add(year_txt);

        // Draw all the circles
        for (d, parts) in days.iter() {
            let d = *d as i32;
            for part in parts.iter() {
                let (offset, color) = match part {
                    Part::A(state) => (-10, state.color()),
                    Part::B(state) => (10, state.color()),
                };

                let c = Circle::new()
                    .set("r", mark_radius)
                    .set("stroke", "black")
                    .set("fill", color)
                    .set("cx", 2 * border + i * column_space + offset)
                    .set("cy", 25 + (linespacing + fontsize) * d);
                document = document.add(c);
            }
        }
    }

    ////////////////////////////////////////////////////////////////////////
    // Draw the day number text on the left side
    for d in 1..=25 {
        let t = svg::node::element::Text::new()
            .add(Text::new(format!("{:02}", d)))
            .set("font-size", fontsize)
            .set("x", 10)
            .set("y", (border + (linespacing + fontsize) * d) as usize);
        document = document.add(t);

        // black dashed line for each week
        if d % 7 == 0 {
            let line = Line::new()
                .set("stroke", "black")
                .set(
                    "stroke-dasharray",
                    format!("{},{}", fontsize / 2, fontsize / 2),
                )
                .set("x1", 10)
                .set("x2", data.len() * column_space as usize)
                .set(
                    "y1",
                    linespacing / 2 + border + (linespacing + fontsize) * d,
                )
                .set(
                    "y2",
                    linespacing / 2 + border + (linespacing + fontsize) * d,
                );
            document = document.add(line);
        }

        // blue line every 5 days
        if d % 5 == 0 {
            let line = Line::new()
                .set("stroke", "blue")
                .set("x1", 10)
                .set("x2", data.len() * column_space as usize)
                .set(
                    "y1",
                    linespacing / 2 + border + (linespacing + fontsize) * d,
                )
                .set(
                    "y2",
                    linespacing / 2 + border + (linespacing + fontsize) * d,
                );
            document = document.add(line);
        }
    }

    svg::save("image.svg", &document).unwrap();
}
