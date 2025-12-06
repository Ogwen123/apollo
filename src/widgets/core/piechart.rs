use crate::style::{Colour, Size};
use gpui::{App, Bounds, Element, ElementId, GlobalElementId, InspectorElementId, IntoElement, LayoutId, Pixels, Render, RenderOnce, SharedString, Window, canvas, PathBuilder, point, px, rgb, Path};
use std::panic::Location;

// creates a colour using a seed value between 1 and 100 e.g. the percent of the segment
fn make_colour(seed: f64) -> Colour {
    if seed < 0f64 || seed > 100f64 {
        return Colour::Rgb(0x000000);
    }

    let factor = 256f64;

    Colour::Rgb(
        (seed * 2.55 * factor * factor) as u32
            + (seed * 2.55 * factor) as u32
            + (seed * 2.55) as u32,
    )
}

pub struct PieChartData {
    /// The name for the segment
    pub name: SharedString,
    /// The number of elements this section represents
    pub count: f64,
    /// The colour the segment will be
    pub colour: Option<Colour>,
}

impl PieChartData {
    pub fn new<N, C, T>(name: N, count: C, colour: T) -> Self
    where
        N: Into<SharedString>,
        C: Into<f64>,
        T: Into<Colour>,
    {
        PieChartData {
            name: SharedString::new(""),
            count: 1.0,
            colour: Some(Colour::Rgb(0x00ff00)),
        }
    }

    pub fn from<N, C, T>(names: Vec<N>, count: Vec<C>, colours: Vec<T>) -> Vec<Self>
    where
        N: Into<SharedString>,
        C: Into<f64>,
        T: Into<Colour>,
    {
        Vec::new()
    }
}

pub struct PieChart {
    /// The data the pie chart will represent
    data: Vec<PieChartData>,
    /// The diameter of the pie chart
    size: Size,
}

impl PieChart {
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
            size: Size::Px(100.0),
        }
    }

    pub fn size(mut self, size: f32) -> Self {
        self.size = Size::Px(size);
        self
    }

    pub fn add_data(mut self, data: PieChartData) -> Self {
        self.data.push(data);
        self
    }

    pub fn set_data(mut self, data: Vec<PieChartData>) -> Self {
        self.data = data;
        self
    }
}

pub trait PCDHelper {
    fn normalise(&self) -> Vec<(SharedString, f64, Colour)>;
    fn sum(&self) -> f64;
}

impl PCDHelper for Vec<PieChartData> {
    fn normalise(
        &self,
    ) -> Vec<(
        SharedString, /*name*/
        f64,          /*percentage*/
        Colour,       /*colour*/
    )> {
        let total_count = self.sum();
        self.iter()
            .map(|x| {
                let percent = x.count / total_count;
                (
                    x.name.clone(),
                    percent,
                    x.colour.clone().unwrap_or(make_colour(percent)),
                )
            })
            .collect()
    }

    fn sum(&self) -> f64 {
        self.iter().map(|x| x.count).sum()
    }
}

impl RenderOnce for PieChart {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let data = self.data;

        canvas(
            move |_, _, _| {},
            move |bounds, _, window, cx| {
                let mut segments: Vec<(Path<Pixels>, Colour)> = Vec::new();

                for slice in data.normalise() {
                    let mut builder = PathBuilder::fill();
                    let center = point(px(530.), px(85.));
                    let radius = px(30.);
                    builder.move_to(point(center.x + radius, center.y));
                    builder.arc_to(
                        point(radius, radius),
                        px(0.),
                        false,
                        false,
                        point(center.x - radius, center.y),
                    );
                    builder.arc_to(
                        point(radius, radius),
                        px(0.),
                        false,
                        false,
                        point(center.x + radius, center.y),
                    );
                    builder.close();
                    let path = builder.build().unwrap();
                    segments.push((path, slice.2.into()));
                }
            },
        )
    }
}
