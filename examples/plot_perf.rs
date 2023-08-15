use chrono::NaiveDate;
#[cfg(feature = "use_plot")]
use dyno_core::{dynotests::DynoTest, BufferData, CompresedSaver, DynoPlot, PlotColor};
use uuid::Uuid;

fn main() {
    #[cfg(feature = "use_plot")]
    {
        let mut args = std::env::args();
        let program_name = args.next().expect("this should not be happen");
        let data_path = args.next().unwrap_or_else(|| {
            eprintln!("ERROR: no arguments for file");
            eprintln!("USAGE: {program_name} [file.dyno]");
            std::process::exit(1);
        });
        let data = BufferData::decompress_from_path(&data_path).unwrap_or_else(|err| {
            eprintln!("ERROR: Failed to decompress and deserialize `{data_path}` file");
            eprintln!("RETURNED ERROR: {err}");
            std::process::exit(1);
        });

        let plot = DynoPlot::new()
            .create_dyno_plot(&data)
            .set_color(PlotColor::dark());

        plot.show();

        let historys = vec![
            DynoTest {
                id: 0,
                user_id: 0,
                info_id: None,
                uuid: Uuid::new_v4(),
                data_url: "data/data".to_owned(),
                data_checksum: "checksum".to_owned(),
                verified: false,
                start: NaiveDate::from_ymd_opt(2010, 10, 10)
                    .unwrap()
                    .and_hms_milli_opt(10, 10, 10, 10)
                    .unwrap(),
                stop: NaiveDate::from_ymd_opt(2010, 10, 10)
                    .unwrap()
                    .and_hms_milli_opt(10, 40, 3, 33)
                    .unwrap(),
                updated_at: NaiveDate::from_ymd_opt(2010, 10, 10)
                    .unwrap()
                    .and_hms_milli_opt(10, 10, 10, 10)
                    .unwrap(),
                created_at: NaiveDate::from_ymd_opt(2010, 10, 10)
                    .unwrap()
                    .and_hms_milli_opt(10, 10, 10, 10)
                    .unwrap(),
            },
            DynoTest {
                id: 0,
                user_id: 0,
                info_id: None,
                uuid: Uuid::new_v4(),
                data_url: "data/data".to_owned(),
                data_checksum: "checksum".to_owned(),
                verified: false,
                start: NaiveDate::from_ymd_opt(2010, 10, 14)
                    .unwrap()
                    .and_hms_milli_opt(8, 10, 10, 10)
                    .unwrap(),
                stop: NaiveDate::from_ymd_opt(2010, 10, 14)
                    .unwrap()
                    .and_hms_milli_opt(8, 40, 1, 70)
                    .unwrap(),
                updated_at: NaiveDate::from_ymd_opt(2010, 10, 14)
                    .unwrap()
                    .and_hms_milli_opt(10, 10, 10, 10)
                    .unwrap(),
                created_at: NaiveDate::from_ymd_opt(2010, 10, 14)
                    .unwrap()
                    .and_hms_milli_opt(10, 10, 10, 10)
                    .unwrap(),
            },
        ];
        let plot = DynoPlot::new()
            .create_history_dyno(historys)
            .set_color(PlotColor::dark());

        plot.show();
    }
}

// use chrono::Local;
// use plotters::{
//     backend::{PixelFormat, RGBPixel},
//     coord::Shift,
//     prelude::*,
//     style::full_palette::*,
// };
// use std::path::Path as StdPath;

// use crate::{BufferData, DynoErr, DynoResult, Numeric};

// #[repr(u8)]
// #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
// pub enum PlotTheme {
//     #[default]
//     Light = 0,
//     Dark = 1,
// }

// pub struct DynoPlot<DB: DrawingBackend, CT: CoordTranslate> {
//     root: DrawingArea<DB, CT>,
//     theme: PlotTheme,
// }

// impl<DB, CT> DynoPlot<DB, CT>
// where
//     DB: DrawingBackend,
//     CT: CoordTranslate,
// {
//     const DEFAULT_SIZE: (u32, u32) = (1280, 720);
//     fn theme_change(
//         root: &'_ DrawingArea<DB, Shift>,
//         title: impl AsRef<str>,
//         theme: PlotTheme,
//     ) -> DynoResult<()> {
//         match theme {
//             PlotTheme::Light => {
//                 root.fill(&WHITE).map_err(DynoErr::plotters_error)?;
//                 root.titled(
//                     title.as_ref(),
//                     TextStyle::from(("sans-serif", 10.percent_height()).into_font()).color(&BLACK),
//                 )
//                 .map_err(DynoErr::plotters_error)
//             }
//             PlotTheme::Dark => {
//                 root.fill(&GREY_900).map_err(DynoErr::plotters_error)?;
//                 root.titled(
//                     title.as_ref(),
//                     TextStyle::from(("sans-serif", 10.percent_height()).into_font()).color(&WHITE),
//                 )
//                 .map_err(DynoErr::plotters_error)
//             }
//         }
//     }
// }

// impl DynoPlot<BitMapBackend<'_, RGBPixel>> {
//     pub fn new_from_path<P: AsRef<StdPath>>(
//         path: P,
//         title: impl AsRef<str>,
//         theme: PlotTheme,
//         size: Option<(u32, u32)>,
//     ) -> DynoResult<Self> {
//         let root =
//             BitMapBackend::new(&path, size.unwrap_or(Self::DEFAULT_SIZE)).into_drawing_area();
//         Self::theme_change(&root, title, theme)?;
//         Ok(Self {
//             root,
//             theme: Default::default(),
//         })
//     }

//     pub fn new_from_buffer(
//         buffer: &mut Vec<u8>,
//         title: impl AsRef<str>,
//         theme: PlotTheme,
//         size: Option<(u32, u32)>,
//     ) -> DynoResult<Self> {
//         let size = size.unwrap_or(Self::DEFAULT_SIZE);
//         let buf_size = (size.0 * size.1) as usize * <RGBPixel as PixelFormat>::PIXEL_SIZE;
//         if buffer.len() < buf_size {
//             buffer.resize(buf_size, u8::default());
//         }
//         let root = BitMapBackend::with_buffer_and_format(buffer.as_mut_slice(), size)
//             .map_err(DynoErr::plotters_error)?
//             .into_drawing_area();
//         Self::theme_change(&root, title, theme)?;
//         Ok(Self { root, theme })
//     }
// }

// #[cfg(feature = "use_wasm")]
// impl DynoPlot<plotters_canvas::CanvasBackend> {
//     pub fn new_from_canvas_element(
//         canvas: web_sys::HtmlCanvasElement,
//         title: AsRef<str>,
//         theme: PlotTheme,
//     ) -> DynoResult<Self> {
//         let root = plotters_canvas::CanvasBackend::with_canvas_object(canvas)
//             .ok_or(DynoErr::plotters_error(
//                 "Failed to create Canvas Backend from HtmlCanvasElement in DynoPlot",
//             ))?
//             .into_drawing_area();
//         Self::theme_change(&root, title, theme)?;
//         Ok(Self { root, theme })
//     }

//     pub fn new_from_canvas_id(
//         id: impl AsRef<str>,
//         title: AsRef<str>,
//         theme: PlotTheme,
//     ) -> DynoResult<Self> {
//         let root = plotters_canvas::CanvasBackend::new(id)
//             .ok_or(DynoErr::plotters_error(
//                 "Failed to create Canvas Backend from canvas id in DynoPlot",
//             ))?
//             .into_drawing_area();
//         Self::theme_change(&root, title, theme)?;
//         Ok(Self {
//             root,
//             theme: Default::default(),
//         })
//     }
// }

// #[inline(always)]
// pub fn format_unix_timestamp_chart(millis: i64) -> String {
//     let dt = chrono::TimeZone::timestamp_millis(&Local, timestamp);
//     dt.format("%H:%M:%S").to_string()
// }

// #[inline(always)]
// pub fn format_float<N: Numeric>(value: &N) -> String {
//     format!("{:.2}", value.to_f64())
// }

// impl<DB: DrawingBackend> DynoPlot<DB> {
//     pub fn crate_data_chart(&self, data: &BufferData) -> DynoResult<()>
//     where
//         DB: IntoDrawingArea,
//     {
//         let (upper, lower) = self.root.split_vertically(50.percent());
//         Self::create_top_chart(upper, data)?;
//         Self::create_bottom_chart(lower, data)?;
//         self.root.present()
//     }

//     fn create_bottom_chart<DB>(root: DrawingArea<DB, Shift>, data: &BufferData) -> DynoResult<()>
//     where
//         DB: DrawingBackend,
//     {
//         let x_min = data.rpm_engine.min_value().to_u64() / 1000;
//         let x_max = data.rpm_engine.max_value().to_u64() / 1000;

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

// impl<DB> std::ops::Deref for DynoPlot<DB>
// where
//     DB: DrawingBackend,
// {
//     type Target = DrawingArea<DB, Shift>;
//     fn deref(&self) -> &Self::Target {
//         &self.root
//     }
// }
// impl<DB> std::ops::DerefMut for DynoPlot<DB>
// where
//     DB: DrawingBackend,
// {
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         &mut self.root
//     }
// }
