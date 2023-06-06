#[cfg(feature = "use_plot")]
use plotly::layout::{Axis, GridPattern, LayoutGrid, Shape, ShapeLine, ShapeType};
#[cfg(feature = "use_plot")]
use plotly::{Bar, Layout, Plot, Scatter};

#[cfg(feature = "dhat-heap")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

#[cfg(feature = "use_plot")]
fn main() {
    let show = true;
    let mut plot = Plot::new();
    plot.add_trace(
        Scatter::new(vec![2, 6], vec![1, 1])
            .x_axis("x1")
            .y_axis("y1"),
    );
    plot.add_trace(
        Bar::new(vec![1, 2, 3], vec![4, 5, 6])
            .x_axis("x2")
            .y_axis("y2"),
    );
    plot.add_trace(
        Scatter::new(vec![10, 20], vec![40, 50])
            .x_axis("x3")
            .y_axis("y3"),
    );
    plot.add_trace(
        Bar::new(vec![11, 13, 15], vec![8, 11, 20])
            .x_axis("x4")
            .y_axis("y4"),
    );

    let mut layout = Layout::new()
        .grid(
            LayoutGrid::new()
                .rows(2)
                .columns(2)
                .pattern(GridPattern::Independent),
        )
        .x_axis(Axis::new().domain(&[0.0, 0.48]).anchor("x1"))
        .y_axis(Axis::new().domain(&[0.52, 1.]).anchor("y1"))
        .x_axis2(Axis::new().domain(&[0.52, 1.0]).anchor("x2"))
        .y_axis2(Axis::new().domain(&[0.5, 1.]).anchor("y2"))
        .x_axis3(Axis::new().domain(&[0.0, 0.48]).anchor("x3"))
        .y_axis3(Axis::new().domain(&[0.0, 0.48]).anchor("y3"))
        .x_axis4(Axis::new().domain(&[0.52, 1.0]).anchor("x4"))
        .y_axis4(Axis::new().domain(&[0.0, 0.48]).anchor("y4"));

    layout.add_shape(
        Shape::new()
            .x_ref("x1")
            .y_ref("y1")
            .shape_type(ShapeType::Line)
            .x0(3)
            .y0(0.5)
            .x1(5)
            .y1(0.8)
            .line(ShapeLine::new().width(3.)),
    );
    layout.add_shape(
        Shape::new()
            .x_ref("x2")
            .y_ref("y2")
            .shape_type(ShapeType::Rect)
            .x0(4)
            .y0(2)
            .x1(5)
            .y1(6),
    );
    layout.add_shape(
        Shape::new()
            .x_ref("x3")
            .y_ref("y3")
            .shape_type(ShapeType::Rect)
            .x0(10)
            .y0(20)
            .x1(15)
            .y1(30),
    );
    layout.add_shape(
        Shape::new()
            .x_ref("x4")
            .y_ref("y4")
            .shape_type(ShapeType::Circle)
            .x0(5)
            .y0(12)
            .x1(10)
            .y1(18),
    );

    plot.set_layout(layout);
    if show {
        plot.show();
    }
    println!("{}", plot.to_inline_html(Some("plots")));
}

#[cfg(not(feature = "use_plot"))]
fn main() {
    println!("no feature for `use_plot`")
}
