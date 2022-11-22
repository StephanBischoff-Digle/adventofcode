use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fs;
use svg::node::element::Circle;
use svg::node::element::Line;
use svg::node::element::Rectangle;
use svg::node::Text;
use svg::Document;

#[derive(Debug, Deserialize, Serialize, Copy, Clone)]
enum State {
    Done,
    Started,
    Improvable,
    Failing,
    Prototype,
}

struct ColoredState {
    state: State,
    color: &'static str,
}

impl State {
    pub fn color(&self) -> &'static str {
        match self {
            Self::Done => "#3e6d9c",
            Self::Started => "#001253",
            Self::Improvable => "#fd841f",
            Self::Failing => "#e14d2a",
            Self::Prototype => "#3e9c6d",
        }
    }

    pub fn colored_iterator() -> impl Iterator<Item = ColoredState> {
        [
            State::Done,
            State::Started,
            State::Prototype,
            State::Improvable,
            State::Failing,
        ]
        .iter()
        .copied()
        .map(|state| ColoredState {
            state,
            color: state.color(),
        })
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
                border
                    + (fontsize + linespacing) * 26
                    + border
                    + State::colored_iterator().count() as i32 * (fontsize + linespacing),
            ),
        )
        .set("width", border + data.len() as i32 * column_space)
        .set("height", border + (fontsize + linespacing) * 25 + border);

    // Set background to white
    document = document.add(
        Rectangle::new()
            .set("width", "100%")
            .set("height", "100%")
            .set("fill", "#1b1a17"),
    );

    ////////////////////////////////////////////////////////////////////////
    // Render data content
    for (i, (year, days)) in data.iter().enumerate() {
        let i = i as i32;

        let year_txt = svg::node::element::Text::new()
            .add(Text::new(format!("{}", year)))
            .set("x", 2 * border + i * column_space)
            .set("y", 20)
            .set("fill", "#F1EFDC")
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
            .set("fill", "#F1EFDC")
            .set("x", 10)
            .set("y", (border + (linespacing + fontsize) * d) as usize);
        document = document.add(t);

        // dashed line for each week
        if d % 7 == 0 {
            let line = Line::new()
                .set("stroke", "#576F72")
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

        // line every 5 days
        if d % 5 == 0 {
            let line = Line::new()
                .set("stroke", "#7D9D9C")
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

    let legend_start = border + (linespacing + fontsize) * 27;

    let legend_txt = svg::node::element::Text::new()
        .add(Text::new("Legend"))
        .set("x", border - mark_radius)
        .set("y", legend_start - 2 * fontsize)
        .set("fill", "#F1EFDC")
        .set("text-anchor", "left")
        .set("font-size", 2 * fontsize);
    document = document.add(legend_txt);

    for (idx, e) in State::colored_iterator().enumerate() {
        let c = Circle::new()
            .set("r", mark_radius)
            .set("fill", e.color)
            .set("cx", border)
            .set("cy", legend_start + (idx as i32) * (fontsize + linespacing));
        document = document.add(c);

        let t = svg::node::element::Text::new()
            .add(Text::new(format!(
                "{}",
                serde_yaml::to_string::<State>(&e.state)
                    .unwrap()
                    .strip_prefix("---")
                    .unwrap()
            )))
            .set("font-size", fontsize)
            .set("fill", "#F1EFDC")
            .set("x", border + fontsize + linespacing)
            .set("y", legend_start + (idx as i32) * (fontsize + linespacing));
        document = document.add(t);
    }

    svg::save("image.svg", &document).unwrap();
}
