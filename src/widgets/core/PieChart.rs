use std::panic::Location;
use gpui::{App, Bounds, Element, ElementId, GlobalElementId, InspectorElementId, IntoElement, LayoutId, Pixels, SharedString, Window};
use crate::style::{Colour, Size};

pub struct PieChartData {
    /// The name for the segment
    pub name: SharedString,
    /// The number of elements this section represents
    pub count: f64,
    /// The colour the segment will be
    pub colour: Option<Colour>
}

impl PieChartData {
    pub fn new<N, C, T>(name: N, count: C, colour: T) -> Self
    where
        N: Into<SharedString>,
        C: Into<f64>,
        T: Into<Colour>
    {
        PieChartData {
            name: SharedString::new(""),
            count: 1.0,
            colour: Some(Colour::Rgb(0x00ff00))
        }
    }

    pub fn from<N, C, T>(names: Vec<N>, count: Vec<C>, colours: Vec<T>) -> Vec<Self>
    where
        N: Into<SharedString>,
        C: Into<f64>,
        T: Into<Colour>
    {
     Vec::new()
    }
}

pub struct PieChart {
    /// The data the pie chart will represent
    pub data: Vec<PieChartData>,
    /// The diameter of the pie chart
    pub size: Size,
}

impl IntoElement for PieChart {
    type Element = ();

    fn into_element(self) -> Self::Element {
        todo!()
    }
}

impl Element for PieChart {
    type RequestLayoutState = ();
    type PrepaintState = ();

    fn id(&self) -> Option<ElementId> {
        todo!()
    }

    fn source_location(&self) -> Option<&'static Location<'static>> {
        todo!()
    }

    fn request_layout(&mut self, id: Option<&GlobalElementId>, inspector_id: Option<&InspectorElementId>, window: &mut Window, cx: &mut App) -> (LayoutId, Self::RequestLayoutState) {
        todo!()
    }

    fn prepaint(&mut self, id: Option<&GlobalElementId>, inspector_id: Option<&InspectorElementId>, bounds: Bounds<Pixels>, request_layout: &mut Self::RequestLayoutState, window: &mut Window, cx: &mut App) -> Self::PrepaintState {
        todo!()
    }

    fn paint(&mut self, id: Option<&GlobalElementId>, inspector_id: Option<&InspectorElementId>, bounds: Bounds<Pixels>, request_layout: &mut Self::RequestLayoutState, prepaint: &mut Self::PrepaintState, window: &mut Window, cx: &mut App) {
        todo!()
    }
}