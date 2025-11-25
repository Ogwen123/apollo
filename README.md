# Apollo

A rust testing tool written in rust using [gpui](https://gpui.rs) and my custom cargo test parsing tool [cargo-ptest](https://crates.io/crates/cargo-ptest/).

## TODO
 - [x] generalise the sizing for the button widget in the same way the colours are to allow for percentages, etc.
 - [x] get global state displaying in the workspace
 - [x] make an SVG icon button widget
 - [x] add helper functions to state to make adding, removing projects etc
 - [x] allow setting border width on individual sides in the button widget
 - [x] make generic modal widget
   - [x] add on_click callbacks to the modals buttons
   - [x] add close button to modal
 - [x] switch passing the style to each component to use cx.global struct
 - [ ] add Into traits to Size
 - [x] make cargo-ptest run async
 - [x] display the alerts from global state
    - [x] fix alert being behind ui
    - [x] make timer work
 - [x] make custom scrolling because overflow_scroll doesn't work
 - [ ] make config modal for test running
 - [ ] fix tooltips not working
 - [x] fix the test list pushing other elements of the page
 - [ ] fix spinner animation not working
 - [ ] make pie chart for summary
## Feature List
 - [x] Run tests with cargo-ptest
 - [ ] View the code that makes up each test
 - [ ] View a summary of the tests with graphs
 - 