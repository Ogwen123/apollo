# Apollo

A rust testing tool written in rust using [gpui](https://gpui.rs) and my custom cargo test parsing tool [cargo-ptest](https://crates.io/crates/cargo-ptest/1.0.1).

## TODO
 - [x] generalise the sizing for the button widget in the same way the colours are to allow for percentages, etc.
 - [x] get global state displaying in the workspace
 - [ ] make an SVG icon button widget
 - [x] add helper functions to state to make adding, removing projects etc
 - [ ] allow setting border width on individual sides in the button widget
 - [x] make generic modal widget
   - [x] add on_click callbacks to the modals buttons
   - [ ] add close button to modal
 - [x] switch passing the style to each component to use cx.global struct
 - [ ] add Into traits to Size
 - [ ] make cargo-ptest run async