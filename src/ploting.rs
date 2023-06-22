use plotly::{
    color::Rgba,
    common::{AxisSide, Line, LineShape, Mode, Title},
    layout::{Axis, RangeSlider},
    Layout, Plot,
};

use crate::{dynotests::DynoTest, Buffer, BufferData, Numeric};

#[derive(Default, Clone, PartialEq)]
pub struct DynoPlot {
    pub plot: Plot,
    pub size: (u32, u32),
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
        Self::default()
    }

    pub fn set_size(&mut self, size: (u32, u32)) -> &mut Self {
        self.size = size;
        self
    }

    pub fn create_history_dyno(mut self, datas: impl AsRef<[DynoTest]>) -> Self {
        let datas = datas.as_ref();
        let y = datas
            .iter()
            .map(|d| (d.stop - d.start).num_seconds())
            .collect::<Vec<_>>();
        let x = datas.iter().map(|d| d.created_at).collect::<Vec<_>>();
        let trace = plotly::Scatter::new(x, y)
            .show_legend(true)
            .mode(Mode::LinesMarkers)
            .name("Long Usage (second)");

        self.add_trace(trace);

        let layout = Layout::new()
            .x_axis(Axis::new().range_slider(RangeSlider::new().visible(true)))
            .plot_background_color(Rgba::new(0, 0, 0, 0.5))
            .paper_background_color(Rgba::new(0, 0, 0, 0.5))
            .title(Title::new("History Usage Dyno"));
        self.set_layout(layout);
        self
    }

    pub fn create_dyno_plot(mut self, data: &BufferData) -> Self {
        self.add_trace(
            to_scatter("Speed (km/h)", &data.speed, &data.time_stamp)
                .line(Line::new().shape(LineShape::Spline)),
        );
        self.add_trace(
            to_scatter("RPM Roda (rpm)", &data.rpm_roda, &data.time_stamp)
                .line(Line::new().shape(LineShape::Spline))
                .y_axis("y2"),
        );
        self.add_trace(
            to_scatter("RPM Engine (rpm)", &data.rpm_engine, &data.time_stamp)
                .line(Line::new().shape(LineShape::Spline))
                .y_axis("y3"),
        );
        self.add_trace(
            to_scatter("Torque (Nm)", &data.torque, &data.time_stamp)
                .line(Line::new().shape(LineShape::Spline))
                .y_axis("y4"),
        );
        self.add_trace(
            to_scatter("HorsePower (HP)", &data.horsepower, &data.time_stamp)
                .line(Line::new().shape(LineShape::Spline))
                .y_axis("y5"),
        );
        self.add_trace(
            to_scatter("Temperature (C)", &data.temp, &data.time_stamp)
                .line(Line::new().shape(LineShape::Spline))
                .y_axis("y6"),
        );

        let layout = Layout::new()
            .x_axis(Axis::new().range_slider(RangeSlider::new().visible(true)))
            .y_axis(Axis::new().title(Title::new("Y Axis")))
            .y_axis2(
                Axis::new()
                    .title(Title::new("Y Axis2"))
                    .anchor("free")
                    .overlaying("y")
                    .side(AxisSide::Right),
            )
            .y_axis3(
                Axis::new()
                    .title(Title::new("RPM"))
                    .anchor("x")
                    .overlaying("y")
                    .side(AxisSide::Right),
            )
            .plot_background_color(Rgba::new(0, 0, 0, 0.2))
            .paper_background_color(Rgba::new(0, 0, 0, 0.4))
            .title(Title::new("Dynotest Plot"));

        self.plot.set_layout(layout);
        self
    }

    #[cfg(feature = "use_wasm")]
    pub async fn render_to_canvas(&self, canvas: impl AsRef<str>) {
        plotly::bindings::new_plot(canvas.as_ref(), &self.plot).await;
    }
}

fn to_scatter<X: Numeric + serde::Serialize, Y: Numeric + serde::Serialize>(
    name: impl AsRef<str>,
    x: &Buffer<X>,
    y: &Buffer<Y>,
) -> Box<plotly::Scatter<X, Y>> {
    plotly::Scatter::new(x.into_inner(), y.into_inner())
        .mode(Mode::LinesMarkers)
        .name(name.as_ref())
        .show_legend(true)
}

// use plotters::coord::Shift;
// use plotters::prelude::*;
// use std::path::{Path as StdPath, PathBuf};

// use crate::{BufferData, DynoResult, Numeric};

// crate::set_builder!(&mut DynoPlotters {
//     file: PathBuf,
//     size: Option<(u32, u32)>,
//     backend: Backends,
// });

// impl DynoPlotters {
//     const DEFAULT_SIZE: (u32, u32) = (2048, 1600);
//     pub fn new<P: AsRef<StdPath>>(file: P, backend: Backends) -> Self {
//         Self {
//             file: PathBuf::from(file.as_ref()),
//             backend,
//             ..Default::default()
//         }
//     }

//     pub fn build() -> DynoPlottersBuilder {
//         DynoPlottersBuilder::default()
//     }
// }

// #[inline(always)]
// pub fn format_unix_timestamp_chart(millis: &i64) -> String {
//     format!(
//         "{}",
//         chrono::NaiveDateTime::from_timestamp_millis(*millis)
//             .unwrap_or_default()
//             .format("%H:%M:%S")
//     )
// }
// #[inline(always)]
// pub fn format_float<N: Numeric>(value: &N) -> String {
//     format!("{:.2}", value.to_f64())
// }

// impl DynoPlotters {
//     pub fn create(&self, data: &BufferData) -> DynoResult<()> {
//         use Backends::*;
//         let size = self.size.unwrap_or(Self::DEFAULT_SIZE);
//         match self.backend {
//             Svg => self.crate_impl(SVGBackend::new(&self.file, size), data),
//             Png | Jpg => self.crate_impl(BitMapBackend::new(&self.file, size), data),
//         }
//     }

//     pub fn crate_impl<DB>(&self, backend: DB, data: &BufferData) -> DynoResult<()>
//     where
//         DB: IntoDrawingArea,
//     {
//         let root = backend.into_drawing_area();
//         root.fill(&WHITE)?;
//         let root_area = root.titled("DynoTests Data Charts", ("sans-serif", 40))?;
//         let (upper, lower) = root_area.split_vertically(50.percent());

//         Self::create_top_chart(upper, data)?;
//         Self::create_bottom_chart(lower, data)?;

//         root.present()
//             .map(|_| log::info!("DynoTests Chart has been saved to {}", self.file.display()))
//             .map_err(|err| crate::DynoErr::plotters_error(
//                 format!("Unable to write Chart to file, please make sure '{}' dir exists under current dir - [{err}]", self.file.display()))
//             )
//     }

//     fn create_bottom_chart<DB>(root: DrawingArea<DB, Shift>, data: &BufferData) -> DynoResult<()>
//     where
//         DB: DrawingBackend,
//     {
//         let x_min = data.rpm.min_value().to_u64() / 1000;
//         let x_max = data.rpm.max_value().to_u64() / 1000;
//         let y_min = data
//             .torque
//             .min_value()
//             .to_f64()
//             .min(data.horsepower.min_value().to_f64());
//         let y_max = data
//             .torque
//             .max_value()
//             .to_f64()
//             .max(data.horsepower.max_value().to_f64());

//         let mut cc = ChartBuilder::<DB>::on(&root)
//             .margin(8.percent())
//             .x_label_area_size(10.percent_height())
//             .y_label_area_size(14.percent_height())
//             .caption(
//                 "Torque And Power / RPM",
//                 ("sans-serif", (5).percent_height()),
//             )
//             .build_cartesian_2d(x_min..x_max, (y_min - y_max / 6.)..(y_max + y_max / 6.))?;

//         cc.configure_mesh()
//             .x_labels(15)
//             .y_labels(15)
//             .max_light_lines(2)
//             .x_desc("Rpm Engine (RPM x 1000)")
//             .y_desc("Torsi (Nm) And Power (HP)")
//             .x_label_formatter(&ToString::to_string)
//             .y_label_formatter(&format_float)
//             .draw()?;

//         let iter_torque = data.torque.iter().map(|x| x.to_f64());
//         let iter_hp = data.horsepower.iter().map(|x| x.to_f64());
//         let mut series = data
//             .rpm
//             .iter()
//             .map(|x| x.to_u64() / 1000)
//             .zip(iter_torque.zip(iter_hp))
//             .map(|(a, (b, c))| (a, [b, c]))
//             .collect::<Vec<_>>();
//         series.sort_unstable_by(|(a, _), (b, _)| a.cmp(b));
//         series.dedup_by(|(a, _), (b, _)| *a == *b);

//         for (i, &vidx) in [2usize, 3usize].iter().enumerate() {
//             let color = Palette99::pick(vidx);
//             cc.draw_series(LineSeries::new(
//                 series.iter().map(|(rpm, v)| (*rpm, v[i])),
//                 color.stroke_width(3),
//             ))?
//             .label(BufferData::BUFFER_NAME[vidx])
//             .legend(move |(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], color.filled()));
//         }

//         cc.configure_series_labels()
//             .border_style(BLACK)
//             .draw()
//             .map_err(From::from)
//     }

//     fn create_top_chart<DB>(root: DrawingArea<DB, Shift>, data: &BufferData) -> DynoResult<()>
//     where
//         DB: DrawingBackend,
//     {
//         let y_min = data.min();
//         let y_max = data.max();
//         let x_max = data.len() as i64;
//         let mut cc = ChartBuilder::<DB>::on(&root)
//             .margin(8.percent())
//             .x_label_area_size(10.percent_height())
//             .y_label_area_size(14.percent_height())
//             .caption("DynoTests Data", ("sans-serif", (5).percent_height()))
//             .build_cartesian_2d(0..x_max, (y_min - y_max / 4.)..(y_max + y_max / 4.))?;

//         cc.configure_mesh()
//             .x_labels(15)
//             .y_labels(15)
//             // .disable_mesh()
//             .max_light_lines(2)
//             .x_desc("TimeStamp")
//             .y_desc("Value")
//             .x_label_formatter(&|i| {
//                 format_unix_timestamp_chart(&data.time_stamp[(*i % x_max) as usize])
//             })
//             .y_label_formatter(&format_float)
//             .draw()?;

//         for idx in 0..=3 {
//             let color = Palette99::pick(idx).mix(0.9);
//             cc.draw_series(LineSeries::new(
//                 (0..x_max).zip(data.get_iter(idx)),
//                 color.stroke_width(3),
//             ))?
//             .label(BufferData::BUFFER_NAME[idx])
//             .legend(move |(x, y)| {
//                 PathElement::new(vec![(x, y), (x + 20, y)], color.filled().stroke_width(3))
//             });
//         }
//         cc.configure_series_labels()
//             .border_style(BLACK)
//             .draw()
//             .map_err(From::from)
//     }
// }
