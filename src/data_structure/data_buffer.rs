use crate::{convertions::prelude::*, Buffer, CsvSaver, Float, MotorType, Numeric, Stroke};
use chrono::{NaiveDateTime, Utc};

use super::filter::DataFilter;

#[derive(Debug, Default, Clone, Copy, serde::Deserialize, serde::Serialize)]
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
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
    pub fn from_self(&mut self, other: Self) {
        let tmp = self.odo;
        *self = other;
        self.odo += tmp;
    }
    pub fn filter(&mut self, filter: &mut DataFilter) {
        self.rpm_roda = filter.rpm_roda.next(self.rpm_roda);
        self.rpm_engine = filter.rpm_engine.next(self.rpm_engine);
        self.torque = filter.torque.next(self.torque);
        self.horsepower = filter.horsepower.next(self.horsepower);
    }

    pub fn from_serial(
        &mut self,
        config: &'_ mut crate::config::DynoConfig,
        serial_data: super::SerialData,
    ) {
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
        self.odo += jarak_tempuh_roller.to_kilometres();

        let percepatan_roller = MetresPerSecond::from_ms(jarak_tempuh_roller, delta_ms);
        let speed = percepatan_roller
            .to_kilometres_per_hour()
            .if_not_normal(self.speed);

        let rpm_roda = RotationPerMinute::from_rot(putaran, delta_ms).if_not_normal(self.rpm_roda);
        let rpm_engine = match &config.motor_type {
            MotorType::Engine => match config.motor_info.stroke {
                Stroke::Four => RotationPerMinute::from_rot(
                    ((pulse_rpm * 2) / (config.motor_info.cylinder as u32)) as Float,
                    delta_ms,
                ),
                _ => RotationPerMinute::from_rot(pulse_rpm as Float, delta_ms),
            },
            MotorType::Electric => RotationPerMinute::from_rot(pulse_rpm as Float, delta_ms),
        }
        .if_not_normal(self.rpm_engine);

        let percepatan_sudut = rpm_roda.to_radians_per_second();
        self.torque = NewtonMeter::new(
            (config.inertia_roller_beban() * (percepatan_sudut - self.percepatan_sudut).to_float())
                * config.perbandingan_gear(),
        )
        .if_negative_normal(self.torque);

        self.horsepower =
            HorsePower::from_nm(self.torque, rpm_roda).if_negative_normal(self.horsepower);
        self.temp = Celcius::new(temperature).if_not_normal(self.temp);
        self.time_stamp = Utc::now().naive_local();

        self.speed = speed;
        self.rpm_roda = rpm_roda;
        self.percepatan_sudut = percepatan_sudut;
        self.percepatan_roller = percepatan_roller;
        self.rpm_engine = rpm_engine;
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

    #[cfg(feature = "use_excel")]
    pub fn from_row_excel(&mut self, row: &'_ [calamine::DataType]) -> Option<()> {
        if row.len() < <BufferData as crate::ExcelSaver>::SIZE_IDX {
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
        self.speed = next_row()?.into();
        self.rpm_roda = next_row()?.into();
        self.rpm_engine = next_row()?.into();
        self.torque = next_row()?.into();
        self.horsepower = next_row()?.into();
        self.temp = next_row()?.into();
        self.time_stamp = row_iter.next()?.as_datetime()?;
        Some(())
    }

    pub fn from_line_delim<S: AsRef<str>>(&mut self, line_str: S) -> Option<()> {
        let mut itw = line_str.as_ref().split(BufferData::CSV_DELIMITER);
        let mut pnext = || itw.next().and_then(|x| x.parse::<Float>().ok());

        self.speed = pnext()?.into();
        self.rpm_roda = pnext()?.into();
        self.rpm_engine = pnext()?.into();
        self.torque = pnext()?.into();
        self.horsepower = pnext()?.into();
        self.temp = pnext()?.into();
        self.time_stamp =
            NaiveDateTime::from_timestamp_millis(itw.next().and_then(|x| x.parse().ok())?)?;

        Some(())
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

    pub data: Data,
    pub len: usize,

    #[serde(skip)]
    #[serde(default)]
    pub total_time: u64,
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
        self.data = Default::default();
        self.len = 0;
    }

    #[inline(always)]
    pub const fn is_empty(&self) -> bool {
        self.len == 0
    }

    #[inline]
    pub fn process_data(&mut self) {
        self.speed.push(self.data.speed);
        self.rpm_roda.push(self.data.rpm_roda);
        self.rpm_engine.push(self.data.rpm_engine);
        self.torque.push(self.data.torque);
        self.horsepower.push(self.data.horsepower);
        self.temp.push(self.data.temp);
        self.time_stamp
            .push(self.data.time_stamp.timestamp_millis() as _);
        self.len += 1;
    }

    #[inline(always)]
    pub fn push_from_serial(
        &mut self,
        config: &'_ mut crate::config::DynoConfig,
        serial_data: crate::SerialData,
    ) {
        self.total_time += serial_data.period as u64;
        self.data.from_serial(config, serial_data);
        self.data.filter(&mut config.filter);
        self.process_data();
    }

    pub fn push_from_data(&mut self, config: &'_ mut crate::config::DynoConfig, data: Data) {
        self.data.from_self(data);
        self.data.filter(&mut config.filter);
        self.process_data();
    }

    pub fn extend_data(&mut self, data: impl AsRef<[Data]>) {
        data.as_ref().iter().copied().for_each(|d| {
            self.data.from_self(d);
            self.process_data();
        })
    }

    #[inline]
    pub fn time_fmt(&self) -> String {
        let seconds = (self.total_time / 1000) % 60;
        let minutes = (self.total_time / (1000 * 60)) % 60;
        let hours = (self.total_time / (1000 * 60 * 60)) % 24;
        format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
    }

    #[inline]
    pub fn last(&self) -> &'_ Data {
        &self.data
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

impl CsvSaver for BufferData {
    const CSV_DELIMITER: &'static str = ",";

    fn open_csv_from_reader<R: std::io::BufRead>(reader: R) -> crate::DynoResult<Self> {
        let mut slf = Self::default();
        reader.lines().map_while(Result::ok).for_each(|line_str| {
            if slf.data.from_line_delim(line_str).is_some() {
                slf.process_data();
            } else {
                log::error!("Parsing Error: Failed to parse line csv");
            }
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
impl crate::ExcelSaver for BufferData {
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
        rows_iterator.for_each(|row_excel| {
            if slf.data.from_row_excel(row_excel).is_some() {
                slf.process_data();
            } else {
                log::error!("Failed to parse row excel into data");
            }
        });
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
