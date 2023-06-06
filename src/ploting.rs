// use plotters::coord::Shift;
// use plotters::prelude::*;
// use std::path::{Path as StdPath, PathBuf};

// use crate::{BufferData, DynoResult, Numeric};

use std::path::{Path, PathBuf};

use plotly::{
    common::{Line, LineShape},
    layout::{Axis, RangeSelector, RangeSlider, SelectorButton, SelectorStep, StepMode},
    Layout, Plot,
};

use crate::{BufferData, DynoResult};

#[repr(usize)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum PlotOut {
    #[default]
    Show = 0,
    Html,
    Image,
    Json,
}

// // 1600*1200

#[derive(Default, Clone)]
pub struct DynoPlot {
    pub plot: Plot,
    pub file: PathBuf,
    pub size: (u32, u32),

    pub out: PlotOut,
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

    pub fn set_output_plot<P>(&mut self, out: PlotOut, file: Option<P>) -> &mut Self
    where
        P: AsRef<Path>,
    {
        if let Some(f) = file {
            self.file = f.as_ref().to_path_buf();
        }
        self.out = out;
        self
    }
}
impl DynoPlot {
    pub fn create_all(mut self, data: &BufferData) -> DynoResult<()> {
        self.add_trace(data.get_trace(0, &data.time_stamp, Line::new().shape(LineShape::Spline)));
        (1..=3).for_each(|i| {
            self.plot.add_trace(
                data.get_trace(i, &data.time_stamp, Line::new().shape(LineShape::Spline))
                    .y_axis(format!("y{}", i + 1)),
            )
        });

        let layout = Layout::new().x_axis(
            Axis::new()
                .range_slider(RangeSlider::new().visible(true))
                .range_selector(RangeSelector::new().buttons(vec![
                        SelectorButton::new()
                            .count(1)
                            .label("30s")
                            .step(SelectorStep::Second)
                            .step_mode(StepMode::Backward),
                        SelectorButton::new()
                            .count(6)
                            .label("1m")
                            .step(SelectorStep::Minute)
                            .step_mode(StepMode::Backward),
                        SelectorButton::new()
                            .count(1)
                            .label("5m")
                            .step(SelectorStep::Minute)
                            .step_mode(StepMode::ToDate),
                        SelectorButton::new()
                            .count(1)
                            .label("all")
                            .step(SelectorStep::All)
                            .step_mode(StepMode::Backward),
                        SelectorButton::new().step(SelectorStep::All),
                    ])),
        );
        self.set_layout(layout);
        Ok(())
    }
}

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
