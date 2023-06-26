use chrono::{Local, NaiveDateTime, TimeZone};
use plotly::{
    common::{AxisSide, Font, Line, LineShape, Marker, Mode, TickFormatStop, Title},
    layout::{Axis, Margin, RangeSelector, RangeSlider, SelectorButton, SelectorStep, StepMode},
    Configuration, Layout, Plot,
};

use crate::{dynotests::DynoTest, Buffer, BufferData, Numeric};

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PlotColor {
    pub(crate) fg: &'static str,
    pub(crate) base: &'static str,
    pub(crate) base100: &'static str,
}
impl PlotColor {
    pub fn dark() -> Self {
        Self {
            fg: "#F1F1F1",
            base: "#121212",
            base100: "#202020",
        }
    }
    pub fn light() -> Self {
        Self {
            fg: "#121212",
            base: "#ffffff",
            base100: "#f1f1f1",
        }
    }
}

#[derive(Default, Clone, PartialEq)]
pub struct DynoPlot {
    plot: Plot,
    color: PlotColor,
}

impl std::ops::Deref for DynoPlot {
    type Target = Plot;
    fn deref(&self) -> &Self::Target {
        &self.plot
    }
}

impl std::ops::DerefMut for DynoPlot {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.plot
    }
}

impl DynoPlot {
    pub fn new() -> Self {
        let mut plot = Plot::new();
        plot.set_configuration(
            Configuration::new()
                .editable(false)
                .show_link(false)
                .watermark(false)
                .autosizable(true)
                .display_logo(false)
                .show_send_to_cloud(false)
                .show_edit_in_chart_studio(false),
        );
        Self {
            plot,
            color: PlotColor::light(),
        }
    }

    pub fn set_color(mut self, color: PlotColor) -> Self {
        self.color = color;
        self
    }

    pub fn create_history_dyno(mut self, datas: impl AsRef<[DynoTest]>) -> Self {
        let datas = datas.as_ref();
        let y = datas
            .iter()
            .map(|d| Local.from_utc_datetime(&d.stop) - Local.from_utc_datetime(&d.start))
            .collect::<Vec<_>>();
        let y_s = y.iter().map(|d| d.num_seconds()).collect::<Vec<_>>();
        let x = datas
            .iter()
            .map(|d| Local.from_utc_datetime(&d.start).format("%+").to_string())
            .collect::<Vec<_>>();
        let trace_s = plotly::Scatter::new(x, y_s)
            .show_legend(true)
            .mode(Mode::Markers)
            .marker(Marker::new().size(20))
            .name("Long Usage");

        self.add_trace(trace_s);

        let layout = Layout::new()
            .margin(Margin::new().top(40).bottom(20))
            .plot_background_color(self.color.base)
            .paper_background_color(self.color.base100)
            .x_axis(
                Axis::new()
                    .range_slider(RangeSlider::new().visible(true))
                    .range_selector(RangeSelector::new().buttons(vec![
                        SelectorButton::new()
                            .count(7)
                            .label("1w")
                            .step(SelectorStep::Day)
                            .step_mode(StepMode::Backward),
                        SelectorButton::new()
                            .count(1)
                            .label("1m")
                            .step(SelectorStep::Month)
                            .step_mode(StepMode::Backward),
                        SelectorButton::new()
                            .count(6)
                            .label("6m")
                            .step(SelectorStep::Month)
                            .step_mode(StepMode::Backward),
                        SelectorButton::new()
                            .count(1)
                            .label("1Y")
                            .step(SelectorStep::Year)
                            .step_mode(StepMode::Backward),
                        SelectorButton::new().step(SelectorStep::All),
                    ]))
                    .tick_format_stops(vec![
                        TickFormatStop::new()
                            .dtick_range(vec![0, 1000])
                            .value("%H:%M:%S.%L ms"),
                        TickFormatStop::new()
                            .dtick_range(vec![1000, 60000])
                            .value("%H:%M:%S s"),
                        TickFormatStop::new()
                            .dtick_range(vec![60000, 3600000])
                            .value("%H:%M m"),
                        TickFormatStop::new()
                            .dtick_range(vec![3600000, 86400000])
                            .value("%H:%M h"),
                        TickFormatStop::new()
                            .dtick_range(vec![86400000, 604800000])
                            .value("%e. %b d"),
                        TickFormatStop::new()
                            .dtick_range(vec!["M1", "M12"])
                            .value("%b '%y M"),
                    ]),
            )
            .y_axis(Axis::new().title(Title::new("Second")))
            .auto_size(true);

        self.set_layout(layout);
        self
    }

    pub fn create_dyno_plot(mut self, data: &BufferData) -> Self {
        let time_stamp: Vec<_> = data
            .time_stamp
            .iter()
            .map(|x| {
                Local
                    .from_utc_datetime(
                        &NaiveDateTime::from_timestamp_millis(*x).unwrap_or_default(),
                    )
                    .format("%+")
                    .to_string()
            })
            .collect();
        self.add_trace(
            to_scatter("Speed (km/h)", &time_stamp, &data.speed)
                .line(Line::new().shape(LineShape::Spline)),
        );
        self.add_trace(
            to_scatter("RPM Roda", &time_stamp, &data.rpm_roda)
                .line(Line::new().shape(LineShape::Spline))
                .y_axis("y2"),
        );
        self.add_trace(
            to_scatter("RPM Engine", &time_stamp, &data.rpm_engine)
                .line(Line::new().shape(LineShape::Spline))
                .y_axis("y2"),
        );
        self.add_trace(
            to_scatter("Torque (Nm)", &time_stamp, &data.torque)
                .line(Line::new().shape(LineShape::Spline))
                .y_axis("y3"),
        );
        self.add_trace(
            to_scatter("HorsePower (HP)", &time_stamp, &data.horsepower)
                .line(Line::new().shape(LineShape::Spline))
                .y_axis("y3"),
        );
        self.add_trace(
            to_scatter("Temperature (C)", &time_stamp, &data.temp)
                .line(Line::new().shape(LineShape::Spline)),
        );

        let layout = Layout::new()
            .margin(Margin::new().top(40).bottom(20))
            .font(Font::new().color(self.color.fg))
            .plot_background_color(self.color.base)
            .paper_background_color(self.color.base100)
            .x_axis(
                Axis::new()
                    .domain(&[0.05, 0.98])
                    .range_selector(RangeSelector::new().buttons(vec![
                        SelectorButton::new()
                            .count(1)
                            .label("1m")
                            .step(SelectorStep::Minute)
                            .step_mode(StepMode::Backward),
                        SelectorButton::new()
                            .count(6)
                            .label("6m")
                            .step(SelectorStep::Minute)
                            .step_mode(StepMode::Backward),
                        SelectorButton::new()
                            .count(30)
                            .label("30m")
                            .step(SelectorStep::Minute)
                            .step_mode(StepMode::Backward),
                        SelectorButton::new()
                            .count(1)
                            .label("1h")
                            .step(SelectorStep::Hour)
                            .step_mode(StepMode::Backward),
                        SelectorButton::new().step(SelectorStep::All),
                    ]))
                    .tick_format_stops(vec![
                        TickFormatStop::new()
                            .dtick_range(vec![0, 1000])
                            .value("%H:%M:%S.%L ms"),
                        TickFormatStop::new()
                            .dtick_range(vec![1000, 60000])
                            .value("%H:%M:%S s"),
                        TickFormatStop::new()
                            .dtick_range(vec![60000, 3600000])
                            .value("%H:%M m"),
                        TickFormatStop::new()
                            .dtick_range(vec![3600000, 86400000])
                            .value("%H:%M h"),
                        TickFormatStop::new()
                            .dtick_range(vec![86400000, 604800000])
                            .value("%e. %b d"),
                        TickFormatStop::new()
                            .dtick_range(vec!["M1", "M12"])
                            .value("%b '%y M"),
                    ]),
            )
            .y_axis(Axis::new().title(Title::new("Speed")))
            .y_axis2(
                Axis::new()
                    .title(Title::new("RPM"))
                    .anchor("x")
                    .overlaying("y")
                    .side(AxisSide::Right),
            )
            .y_axis3(
                Axis::new()
                    .title(Title::new("Torque and HP"))
                    .anchor("free")
                    .overlaying("y")
                    .position(0.000)
                    .side(AxisSide::Left),
            )
            .auto_size(true);

        self.plot.set_layout(layout);
        self
    }

    #[cfg(feature = "use_wasm")]
    pub async fn render_to_canvas(&self, canvas: impl AsRef<str>) {
        plotly::bindings::new_plot(canvas.as_ref(), &self.plot).await;
    }
}

fn to_scatter<Y: serde::Serialize + Numeric>(
    name: impl AsRef<str>,
    x: &[String],
    y: &Buffer<Y>,
) -> Box<plotly::Scatter<String, Y>> {
    plotly::Scatter::new(x.to_owned(), y.into_inner())
        .mode(Mode::LinesMarkers)
        .line(Line::new().shape(LineShape::Spline))
        .name(name.as_ref())
        .web_gl_mode(true)
        .show_legend(true)
}
