# Apollo

A rust testing tool written in rust using [gpui](https://gpui.rs) and my custom cargo test parsing tool [cargo-ptest](https://crates.io/crates/cargo-ptest/1.0.1).

## TODO
 - [x] generalise the sizing for the button widget in the same way the colours are to allow for percentages, etc.
 - [x] get global state displaying in the workspace
 - [x] make an SVG icon button widget
 - [x] add helper functions to state to make adding, removing projects etc
 - [ ] allow setting border width on individual sides in the button widget
 - [x] make generic modal widget
   - [x] add on_click callbacks to the modals buttons
   - [x] add close button to modal
 - [x] switch passing the style to each component to use cx.global struct
 - [ ] add Into traits to Size
 - [x] make cargo-ptest run async
 - [ ] display the alerts from global state
 - [ ] make config modal for test running
 - [ ] fix tooltips not working
 - [ ] fix scrolling not working or add custom scrolling that scrolls through the list of tests rather than the div