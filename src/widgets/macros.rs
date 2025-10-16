/// For a single value, x, outputs (x, x, x, x)
/// For 2 values, x and y, outputs (x, y, x, y) where x is horizontal padding and y is vertical padding
/// For 4 values, t, r, b, l, outputs (t, r, b, l) where t, r, b, l are top, right, bottom and left respectively
#[macro_export] macro_rules! padding {
    ($all:expr) => {
        ($all, $all, $all, $all)
    };
    ($x:expr, $y:expr) => {
        ($y, $x, $y, $x)
    };
    ($top:expr, $right:expr, $bottom:expr, $left:expr) => {
        ($top, $right, $bottom, $left)
    };
}

/// For a single value, x, outputs (x, x, x, x)
/// For 2 values, x and y, outputs (x, y, x, y) where x is horizontal padding and y is vertical padding
/// For 4 values, t, r, b, l, outputs (t, r, b, l) where t, r, b, l are top, right, bottom and left respectively
#[macro_export] macro_rules! margin {
    ($all:expr) => {
        ($all, $all, $all, $all)
    };
    ($x:expr, $y:expr) => {
        ($y, $x, $y, $x)
    };
    ($top:expr, $right:expr, $bottom:expr, $left:expr) => {
        ($top, $right, $bottom, $left)
    };
}

/// For a single value, x, outputs (x, x, x, x)
/// For 4 values, tl, tr, br, bl, outputs (tl, tr, br, bl) where tl, tr, br, bl are top left, top right, bottom right and bottom left respectively (clockwise starting at the top left)
#[macro_export] macro_rules! rounding {
    ($all:expr) => {
        ($all, $all, $all, $all)
    };
    ($top:expr, $right:expr, $bottom:expr, $left:expr) => {
        ($top, $right, $bottom, $left)
    };
}