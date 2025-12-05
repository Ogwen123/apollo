use crate::style::{Colour, Size};
use gpui::{
    App, Bounds, Element, ElementId, GlobalElementId, InspectorElementId, IntoElement, LayoutId,
    Pixels, Render, RenderOnce, SharedString, Window, canvas,
};
use std::panic::Location;

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
            size: Size::Px(100.0)
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

impl RenderOnce for PieChart {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let data = self.data;

        canvas(move |_, _, _| {}, move |bounds, _, window, cx| {
            for slice in data {
                
            }
        })
    }
}
