use crate::{
    convertions::prelude::*, Buffer, CsvSaver, ExcelSaver, Float, InfoMotor, MotorType, Numeric,
    Stroke,
};
use chrono::{NaiveDateTime, Utc};

#[derive(Debug, Default, Clone, serde::Deserialize, serde::Serialize)]
pub struct Data {
    pub speed: KilometresPerHour,
    pub torque: NewtonMeter,
    pub horsepower: HorsePower,
    pub temp: Celcius,
    pub time_stamp: NaiveDateTime,
    pub rpm_roda: RotationPerMinute,
    pub rpm_engine: RotationPerMinute,
    pub odo: KiloMetres,

    pub percepatan_sudut: RadiansPerSecond,
    pub percepatan_roller: MetresPerSecond,
}
impl Data {
    pub fn from_serial(
        last: &'_ Self,
        config: &'_ crate::config::DynoConfig,
        serial_data: super::SerialData,
    ) -> Self {
        let super::SerialData {
            period,
            pulse_enc_max,
            pulse_enc,
            pulse_rpm,
            temperature,
            ..
        } = serial_data;

        let delta_ms = period as Float;

        // let pulse_enc = if pulse_enc > 2_000_000 {
        // } else {
        // }
        let putaran = pulse_enc as Float / pulse_enc_max as Float;

        let jarak_tempuh_roller = config.keliling_roller * putaran;
        let odo = jarak_tempuh_roller.to_kilometres().if_not_normal(last.odo);

        let percepatan_roller = MetresPerSecond::from_ms(jarak_tempuh_roller, delta_ms);
        let speed = percepatan_roller
            .to_kilometres_per_hour()
            .if_not_normal(last.speed);

        let rpm_roda = RotationPerMinute::from_rot(putaran, delta_ms).if_not_normal(last.rpm_roda);

        let rpm_engine = match &config.motor_type {
            MotorType::Engine(InfoMotor {
                cylinder, stroke, ..
            }) => match stroke {
                Stroke::Four => RotationPerMinute::from_rot(
                    ((pulse_rpm * 2) / (*cylinder as u32)) as Float,
                    delta_ms,
                ),
                _ => RotationPerMinute::from_rot(pulse_rpm as Float, delta_ms),
            },
            MotorType::Electric(_) => RotationPerMinute::from_rot(pulse_rpm as Float, delta_ms),
        }
        .if_not_normal(last.rpm_engine);

        let percepatan_sudut = rpm_roda.to_radians_per_second();

        let torque = NewtonMeter::new(
            (config.inertia_roller_beban() * (percepatan_sudut - last.percepatan_sudut).to_float())
                * config.perbandingan_gear(),
        )
        .if_negative_normal(last.torque);

        let horsepower = HorsePower::from_nm(torque, rpm_roda).if_negative_normal(last.horsepower);

        let temp = Celcius::new(temperature).if_not_normal(last.temp);

        let time_stamp = Utc::now().naive_local();

        Self {
            speed,
            odo,
            rpm_roda,
            rpm_engine,
            temp,
            time_stamp,
            torque,
            horsepower,
            percepatan_sudut,
            percepatan_roller,
        }
    }

    pub fn time_duration_formatted(&self, start: chrono::NaiveTime) -> String {
        let dur = self.time_stamp.time() - start;
        format!(
            "{:2}:{:2}:{:2}",
            dur.num_hours(),
            dur.num_minutes(),
            dur.num_seconds()
        )
    }
    #[inline]
    pub fn from_self(&mut self, other: Self) {
        self.speed = other.speed;
        self.odo += other.odo;
        self.rpm_roda = other.rpm_roda;
        self.rpm_engine = other.rpm_engine;
        self.temp = other.temp;
        self.time_stamp = other.time_stamp;
        self.torque = other.torque;
        self.horsepower = other.horsepower;
        self.percepatan_sudut = other.percepatan_sudut;
        self.percepatan_roller = other.percepatan_roller;
    }

    #[cfg(feature = "use_excel")]
    pub fn from_row_excel(row: &'_ [calamine::DataType]) -> Option<Self> {
        if row.len() < BufferData::SIZE_IDX {
            return None;
        };
        let mut row_iter = row.iter();
        let mut next_row = || {
            let item = row_iter.next();
            item.map(|x| {
                x.get_float()
                    .unwrap_or(x.get_int().unwrap_or_default().to_f64())
            })
        };
        let speed = next_row()?.into();
        let rpm_roda = next_row()?.into();
        let rpm_engine = next_row()?.into();
        let torque = next_row()?.into();
        let horsepower = next_row()?.into();
        let temp = next_row()?.into();
        let time_stamp = row_iter.next()?.as_datetime()?;

        Some(Self {
            speed,
            torque,
            horsepower,
            temp,
            time_stamp,
            rpm_roda,
            rpm_engine,
            ..Default::default()
        })
    }

    pub fn from_line_delim<S: AsRef<str>>(line_str: S) -> Option<Self> {
        let mut itw = line_str.as_ref().split(BufferData::CSV_DELIMITER);
        let mut pnext = || itw.next().and_then(|x| x.parse::<Float>().ok());

        let speed = pnext()?.into();
        let rpm_roda = pnext()?.into();
        let rpm_engine = pnext()?.into();
        let torque = pnext()?.into();
        let horsepower = pnext()?.into();
        let temp = pnext()?.into();
        let time_stamp =
            NaiveDateTime::from_timestamp_millis(itw.next().and_then(|x| x.parse().ok())?)?;

        Some(Data {
            speed,
            torque,
            horsepower,
            temp,
            time_stamp,
            rpm_roda,
            rpm_engine,
            ..Default::default()
        })
    }
}

#[derive(Debug, Clone, Default, serde::Deserialize, serde::Serialize)]
pub struct BufferData {
    pub speed: Buffer<KilometresPerHour>,
    pub rpm_roda: Buffer<RotationPerMinute>,
    pub rpm_engine: Buffer<RotationPerMinute>,
    pub torque: Buffer<NewtonMeter>,
    pub horsepower: Buffer<HorsePower>,
    pub temp: Buffer<Celcius>,
    pub time_stamp: Buffer<i64>,

    pub last: Data,
    pub len: usize,
}

#[repr(usize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum DBIdx {
    Speed = 0,
    RpmRoda,
    RpmEngine,
    Torque,
    Hp,
    Temp,
    TimeStamp,
    SizeMax,
}

impl BufferData {
    const MAX_COLS_SIZE: usize = DBIdx::SizeMax as usize;
    pub const BUFFER_NAME: [&'static str; Self::MAX_COLS_SIZE] = [
        "SPEED (km/h)",
        "RPM Roda (RPM x 1000)",
        "RPM Engine (RPM x 1000)",
        "TORQUE (Nm)",
        "HORSEPOWER (HP)",
        "TEMPERATURE (°C)",
        "TIMESTAMP",
    ];

    pub fn new() -> Self {
        Self::default()
    }

    #[inline]
    pub fn clean(&mut self) {
        self.speed.clear();
        self.rpm_roda.clear();
        self.rpm_engine.clear();
        self.torque.clear();
        self.horsepower.clear();
        self.temp.clear();
        self.time_stamp.clear();
        self.len = 0;
    }

    #[inline(always)]
    pub const fn is_empty(&self) -> bool {
        self.len == 0
    }

    #[inline]
    pub fn push_data(&mut self, data: Data) {
        self.speed.push(data.speed);
        self.rpm_roda.push(data.rpm_roda);
        self.rpm_engine.push(data.rpm_engine);
        self.torque.push(data.torque);
        self.horsepower.push(data.horsepower);
        self.temp.push(data.temp);
        self.time_stamp
            .push(data.time_stamp.timestamp_millis() as _);

        self.last.from_self(data);
        self.len += 1;
    }

    #[inline(always)]
    pub fn push_from_serial(
        &mut self,
        config: &'_ crate::config::DynoConfig,
        serial_data: crate::SerialData,
    ) {
        self.push_data(Data::from_serial(&self.last, config, serial_data));
    }

    #[inline]
    pub fn last(&self) -> &'_ Data {
        &self.last
    }

    #[inline(always)]
    pub fn len(&self) -> usize {
        self.len
    }

    #[inline(always)]
    pub fn max(&self) -> f64 {
        [
            self.speed.max_value().to_f64(),
            self.rpm_roda.max_value().to_f64() * 0.001,
            self.rpm_engine.max_value().to_f64() * 0.001,
            self.torque.max_value().to_f64(),
            self.horsepower.max_value().to_f64(),
        ]
        .iter()
        .max_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Greater))
        .copied()
        .unwrap_or_default()
    }

    #[inline(always)]
    pub fn min(&self) -> f64 {
        [
            self.speed.min_value().to_f64(),
            self.rpm_roda.max_value().to_f64(),
            self.rpm_engine.min_value().to_f64(),
            self.torque.min_value().to_f64(),
            self.horsepower.min_value().to_f64(),
        ]
        .iter()
        .min_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Greater))
        .copied()
        .unwrap_or_default()
    }
}

#[cfg(feature = "use_plot")]
impl BufferData {
    pub fn get_trace<X>(
        &self,
        idx: usize,
        x_axis: &Buffer<X>,
        line: plotly::common::Line,
    ) -> Box<plotly::Scatter<X, Float>>
    where
        X: serde::Serialize + Clone + Copy + Sized,
    {
        let scatter_impl = |b: Vec<Float>| {
            plotly::Scatter::new(x_axis.into_inner(), b)
                .mode(plotly::common::Mode::LinesMarkers)
                .name(Self::BUFFER_NAME[idx])
                .show_legend(true)
                .line(line)
        };
        match idx {
            0 => scatter_impl(self.speed.iter().map(|x| x.to_float()).collect()),
            1 => scatter_impl(self.rpm_roda.iter().map(|x| x.to_float() * 0.001).collect()),
            2 => scatter_impl(
                self.rpm_engine
                    .iter()
                    .map(|x| x.to_float() * 0.001)
                    .collect(),
            ),
            3 => scatter_impl(self.torque.iter().map(|x| x.to_float()).collect()),
            4 => scatter_impl(self.horsepower.iter().map(|x| x.to_float()).collect()),
            5 => scatter_impl(self.temp.iter().map(|x| x.to_float()).collect()),
            6 => scatter_impl(self.time_stamp.iter().map(|x| x.to_float()).collect()),
            _ => unreachable!("index is overflow {idx}"),
        }
    }
}

impl CsvSaver for BufferData {
    const CSV_DELIMITER: &'static str = ",";

    fn open_csv_from_reader<R: std::io::BufRead>(reader: R) -> crate::DynoResult<Self> {
        let mut slf = Self::default();
        reader
            .lines()
            .map_while(Result::ok)
            .filter_map(Data::from_line_delim)
            .for_each(|data| {
                slf.push_data(data);
            });
        Ok(slf)
    }

    fn save_csv_from_writer<W: std::io::Write>(&self, writer: &mut W) -> crate::DynoResult<()> {
        writeln!(
            writer,
            "SPEED,RPM(RODA),RPM(ENGINE),TORQUE,HORSEPOWER,TEMP,TIME"
        )?;
        for idx in 0usize..self.len {
            writeln!(
                writer,
                "{speed},{rpm_roda},{rpm_engine},{torque},{horsepower},{temp},{time}",
                speed = self.speed[idx],
                rpm_roda = self.rpm_roda[idx],
                rpm_engine = self.rpm_engine[idx],
                torque = self.torque[idx],
                horsepower = self.horsepower[idx],
                temp = self.temp[idx],
                time = self.time_stamp[idx],
            )
            .ok();
        }
        Ok(())
    }
}

#[cfg(feature = "use_excel")]
impl ExcelSaver for BufferData {
    const SIZE_IDX: usize = 7;
    const EXCEL_SHEET_NAME: &'static str = "dynotest";
    const EXCEL_HEADER_NAME: &'static str = "Dynotest Data Table";

    fn open_excel_from_worksheet<R>(mut workbook: calamine::Xlsx<R>) -> crate::DynoResult<Self>
    where
        R: std::io::Read + std::io::Seek,
    {
        let mut slf = Self::default();
        use calamine::Reader;

        let wb_range = match workbook.worksheet_range(Self::EXCEL_SHEET_NAME) {
                Some(Ok(range)) => range,
                Some(Err(err)) => return Err(crate::DynoErr::excel_error(err)),
                None => return Err(crate::DynoErr::excel_error(
                    format!("Worksheet `{}` not exists, please add or change the worksheet name to open succesfully open the file", Self::EXCEL_SHEET_NAME),
                ))
            };
        let mut rows_iterator = wb_range.rows();
        // ignore header when reading
        let _header = rows_iterator.next();
        rows_iterator
            .filter_map(Data::from_row_excel)
            .for_each(|d| slf.push_data(d));
        Ok(slf)
    }

    fn save_excel_from_worksheet(
        &self,
        worksheet: &mut rust_xlsxwriter::Worksheet,
    ) -> crate::DynoResult<()> {
        use rust_xlsxwriter::*;

        let format_header = Format::new().set_bold().set_border(FormatBorder::Medium);
        let date_format = Format::new().set_num_format("dd/mm/yyyy hh:mm AM/PM");
        for col in 0..Self::MAX_COLS_SIZE {
            worksheet.write_string_with_format(
                0,
                col as _,
                Self::BUFFER_NAME[col],
                &format_header,
            )?;
            match col {
                0 => self.speed.iter().enumerate().for_each(|(index, value)| {
                    if let Err(err) =
                        worksheet.write_number((index + 1) as _, col as _, value.to_f64())
                    {
                        log::error!("{err}")
                    }
                }),
                1 => self.rpm_roda.iter().enumerate().for_each(|(index, value)| {
                    if let Err(err) =
                        worksheet.write_number((index + 1) as _, col as _, value.to_f64())
                    {
                        log::error!("{err}")
                    }
                }),
                2 => self
                    .rpm_engine
                    .iter()
                    .enumerate()
                    .for_each(|(index, value)| {
                        if let Err(err) =
                            worksheet.write_number((index + 1) as _, col as _, value.to_f64())
                        {
                            log::error!("{err}")
                        }
                    }),

                3 => self.torque.iter().enumerate().for_each(|(index, value)| {
                    if let Err(err) =
                        worksheet.write_number((index + 1) as _, col as _, value.to_f64())
                    {
                        log::error!("{err}")
                    }
                }),

                4 => self
                    .horsepower
                    .iter()
                    .enumerate()
                    .for_each(|(index, value)| {
                        if let Err(err) =
                            worksheet.write_number((index + 1) as _, col as _, value.to_f64())
                        {
                            log::error!("{err}")
                        }
                    }),

                5 => self.temp.iter().enumerate().for_each(|(index, value)| {
                    if let Err(err) =
                        worksheet.write_number((index + 1) as _, col as _, value.to_f64())
                    {
                        log::error!("{err}")
                    }
                }),

                6 => self
                    .time_stamp
                    .iter()
                    .enumerate()
                    .for_each(|(index, value)| {
                        let date_time = match NaiveDateTime::from_timestamp_millis(*value as _) {
                            Some(k) => k,
                            None => Default::default(),
                        };
                        if let Err(err) = worksheet.write_datetime(
                            (index + 1) as _,
                            col as _,
                            &date_time,
                            &date_format,
                        ) {
                            log::error!("{err}")
                        }
                    }),
                _ => unreachable!(),
            };
        }

        // TODO: Add charts in excel
        // let mut chart = Chart::new(ChartType::Line);
        // let max_row = self.len_items() as _;
        // for col in 0..Self::MAX_COLS_SIZE as _ {
        //     let series = chart.add_series();
        //     series.set_values((Self::EXCEL_SHEET_NAME, 1, col + 1, max_row, col + 1));
        // }
        // ws.insert_chart(1, (Self::MAX_COLS_SIZE + 2) as _, &chart)?;
        Ok(())
    }
}
