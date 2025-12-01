use std::panic::Location;
use gpui::{App, Bounds, Element, ElementId, GlobalElementId, InspectorElementId, IntoElement, LayoutId, Pixels, SharedString, Window};
use crate::style::{Colour, Size};

pub struct PieChartData {
    /// The name for the segment
    name: SharedString,
    /// The number of elements this section represents
    count: i64,
    /// The colour the segment will be
    colour: Option<Colour>
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