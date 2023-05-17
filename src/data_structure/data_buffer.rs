use std::{
    fs::File,
    io::{BufRead, BufReader, BufWriter, Write},
    path::Path,
};

use super::buffer::Buffer;
use crate::{
    convertions::prelude::*,
    infomotor::{MotorType, Stroke},
    DynoErr, DynoResult, Float, Numeric,
};
use chrono::{NaiveDateTime, Utc};

#[derive(Debug, Default, Clone, serde::Deserialize, serde::Serialize)]
pub struct Data {
    pub speed: KilometresPerHour,
    pub torque: NewtonMeter,
    pub horsepower: HorsePower,
    pub temp: Celcius,
    pub time_stamp: NaiveDateTime,
    pub rpm: RotationPerMinute,
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
            time: _,
            period,
            pulse_enc_max: pulse_encoder_max,
            pulse_enc: pulse_encoder,
            pulse_rpm,
            temperature,
            pulse_enc_raw: _,
            pulse_enc_z: _,
        } = serial_data;

        let delta_ms = period as Float;

        let putaran = (pulse_encoder / pulse_encoder_max) as Float;

        let jarak_tempuh_roller = config.keliling_roller * putaran;
        let odo = jarak_tempuh_roller.to_kilometres().if_not_normal(last.odo);

        let percepatan_roller = MetresPerSecond::from_ms(jarak_tempuh_roller, delta_ms);
        let speed = percepatan_roller
            .to_kilometres_per_hour()
            .if_not_normal(last.speed);

        let rpm = match &config.motor_type {
            MotorType::Electric(_) => RotationPerMinute::from_rot(putaran, delta_ms),
            MotorType::Engine(en) => match en.stroke {
                Stroke::Four if en.cylinder.not_unkown() => RotationPerMinute::from_rot(
                    (pulse_rpm * 2) as Float / (en.cylinder as u32).to_float(),
                    delta_ms,
                ),
                _ => RotationPerMinute::from_rot(pulse_rpm.to_float(), delta_ms),
            },
        }
        .if_not_normal(last.rpm);

        let percepatan_sudut = rpm.to_radians_per_second();

        let torque = NewtonMeter::new(
            (config.inertia_roller_beban() * (percepatan_sudut - last.percepatan_sudut).to_float())
                * config.perbandingan_gear(),
        )
        .if_negative_normal(last.torque);

        let horsepower = HorsePower::from_nm(torque, rpm).if_negative_normal(last.horsepower);

        let temp = Celcius::new(temperature).if_not_normal(last.temp);

        let time_stamp = Utc::now().naive_local();

        Self {
            speed,
            odo,
            rpm,
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
}

#[derive(Debug, Clone, Default, serde::Deserialize, serde::Serialize)]
pub struct BufferData {
    pub speed: Buffer<KilometresPerHour>,
    pub rpm: Buffer<RotationPerMinute>,
    pub torque: Buffer<NewtonMeter>,
    pub horsepower: Buffer<HorsePower>,
    pub temp: Buffer<Celcius>,
    pub time_stamp: Buffer<i64>,
    pub odo: KiloMetres,

    pub last: Data,
    len: usize,
}

impl BufferData {
    const MAX_COLS_SIZE: usize = 6usize;
    pub const BUFFER_NAME: [&'static str; 6] = [
        "SPEED (km/h)",
        "RPM (rpm)",
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
        self.rpm.clear();
        self.torque.clear();
        self.horsepower.clear();
        self.temp.clear();
        self.time_stamp.clear();
        self.odo = KiloMetres::default();
        self.len = 0;
    }
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    #[inline]
    pub fn push_data(&mut self, data: Data) {
        self.speed.push_value(data.speed);
        self.rpm.push_value(data.rpm);
        self.torque.push_value(data.torque);
        self.horsepower.push_value(data.horsepower);
        self.temp.push_value(data.temp);
        self.time_stamp
            .push_value(data.time_stamp.timestamp_millis() as _);
        self.odo += data.odo;

        self.last = data;
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
    pub fn get_points<Out: FromIterator<[f64; 2]>>(&self, idx: usize) -> Out {
        match idx {
            0 => self.speed.into_points(),
            1 => self.rpm.into_points(),
            2 => self.torque.into_points(),
            3 => self.horsepower.into_points(),
            4 => self.temp.into_points(),
            5 => self.time_stamp.into_points(),
            _ => unreachable!("index is overflow {idx}"),
        }
    }
}

impl BufferData {
    const CSV_DELIMITER: &'static str = ",";

    pub fn open_from_csv<P: AsRef<Path>>(path: P) -> DynoResult<Self> {
        let mut slf = Self::default();
        let mut lines = BufReader::new(File::open(path)?).lines();
        let _header = lines.next();
        for line in lines.flatten() {
            let mut itw = line.split(Self::CSV_DELIMITER);
            let pnext =
                |n: Option<&'_ str>| n.and_then(|x| x.parse::<Float>().ok()).unwrap_or_default();

            slf.speed.push_value(pnext(itw.next()).into());
            slf.rpm.push_value(pnext(itw.next()).into());
            slf.torque.push_value(pnext(itw.next()).into());
            slf.horsepower.push_value(pnext(itw.next()).into());
            slf.temp.push_value(pnext(itw.next()).into());
            slf.time_stamp
                .push_value(itw.next().and_then(|x| x.parse().ok()).unwrap_or_default());
            slf.len += 1;
        }
        slf.last = Data {
            speed: slf.speed.last_value(),
            torque: slf.torque.last_value(),
            horsepower: slf.horsepower.last_value(),
            temp: slf.temp.last_value(),
            time_stamp: NaiveDateTime::from_timestamp_millis(slf.time_stamp.last_value())
                .unwrap_or_default(),
            rpm: slf.rpm.last_value(),
            odo: slf.odo,
            percepatan_sudut: Default::default(),
            percepatan_roller: Default::default(),
        };
        Ok(slf)
    }

    pub fn save_as_csv<P: AsRef<Path>>(&self, path: P) -> DynoResult<()> {
        let mut writer = BufWriter::new(File::create(path)?);
        let len = self.len();
        writeln!(&mut writer, "SPEED,RPM,TORQUE,HORSEPOWER,TEMP,TIME")?;
        for idx in 0usize..len {
            writeln!(
                &mut writer,
                "{speed},{rpm},{torque},{horsepower},{temp},{time}",
                speed = self.speed[idx],
                rpm = self.rpm[idx],
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
impl BufferData {
    const EXCEL_SHEET_NAME: &'static str = "dynotest";
    const EXCEL_HEADER_NAME: &'static str = "Dynotest Data Table";

    pub fn open_from_excel<P: AsRef<Path>>(path: P) -> DynoResult<Self> {
        use calamine::Reader;
        let mut slf = Self::default();
        let mut wb = calamine::open_workbook_auto(path)?;
        let wb_range = match wb.worksheet_range(Self::EXCEL_SHEET_NAME) {
                Some(Ok(range)) => range,
                Some(Err(err)) => return Err(From::from(err)),
                None => return Err(DynoErr::excel_error(
                    format!("Worksheet `{}` not exists, please add or change the worksheet name to open succesfully open the file", Self::EXCEL_SHEET_NAME),
                ))
            };
        let mut rows_iterator = wb_range.rows();
        let _header = rows_iterator.next();
        for row in rows_iterator {
            let row_len = row.len().min(Self::MAX_COLS_SIZE) - 1;
            let push_fn = |idx: usize| {
                row[idx]
                    .get_float()
                    .unwrap_or_else(|| row[idx].get_int().unwrap_or_default().to_float())
            };

            slf.speed.push_value(push_fn(0).into());
            slf.rpm.push_value(push_fn(1).into());
            slf.torque.push_value(push_fn(2).into());
            slf.horsepower.push_value(push_fn(3).into());
            slf.temp.push_value(push_fn(4).into());
            slf.time_stamp.push_value(
                row[row_len]
                    .as_datetime()
                    .unwrap_or_default()
                    .timestamp_millis() as _,
            );
            slf.len += 1;
        }

        slf.last = Data {
            speed: slf.speed.last_value(),
            torque: slf.torque.last_value(),
            horsepower: slf.horsepower.last_value(),
            temp: slf.temp.last_value(),
            time_stamp: NaiveDateTime::from_timestamp_millis(slf.time_stamp.last_value())
                .unwrap_or_default(),
            rpm: slf.rpm.last_value(),
            odo: slf.odo,
            percepatan_sudut: Default::default(),
            percepatan_roller: Default::default(),
        };
        Ok(slf)
    }

    pub fn save_as_excel<P: AsRef<Path>>(&self, path: P) -> DynoResult<()> {
        use rust_xlsxwriter::*;
        let format_header = Format::new().set_bold().set_border(FormatBorder::Medium);
        let date_format = Format::new().set_num_format("dd/mm/yyyy hh:mm AM/PM");

        let mut wb = Workbook::new();
        let mut ws = Worksheet::new();
        ws.set_name(Self::EXCEL_SHEET_NAME)?.set_active(true);
        let header = format!(
            r#"&C&"Courier New,Bold"{} - &CCreated at &[Date]"#,
            Self::EXCEL_HEADER_NAME
        );

        ws.set_header(&header);
        for col in 0..Self::MAX_COLS_SIZE {
            ws.write_string_with_format(0, col as _, Self::BUFFER_NAME[col], &format_header)?;
            match col {
                0 => self.speed.iter().enumerate().for_each(|(index, value)| {
                    if let Err(err) = ws.write_number((index + 1) as _, col as _, value.to_f64()) {
                        log::error!("{err}")
                    }
                }),
                1 => self.rpm.iter().enumerate().for_each(|(index, value)| {
                    if let Err(err) = ws.write_number((index + 1) as _, col as _, value.to_f64()) {
                        log::error!("{err}")
                    }
                }),

                2 => self.torque.iter().enumerate().for_each(|(index, value)| {
                    if let Err(err) = ws.write_number((index + 1) as _, col as _, value.to_f64()) {
                        log::error!("{err}")
                    }
                }),

                3 => self
                    .horsepower
                    .iter()
                    .enumerate()
                    .for_each(|(index, value)| {
                        if let Err(err) =
                            ws.write_number((index + 1) as _, col as _, value.to_f64())
                        {
                            log::error!("{err}")
                        }
                    }),

                4 => self.temp.iter().enumerate().for_each(|(index, value)| {
                    if let Err(err) = ws.write_number((index + 1) as _, col as _, value.to_f64()) {
                        log::error!("{err}")
                    }
                }),

                5 => self
                    .time_stamp
                    .iter()
                    .enumerate()
                    .for_each(|(index, value)| {
                        let date_time = match NaiveDateTime::from_timestamp_millis(*value as _) {
                            Some(k) => k,
                            None => Default::default(),
                        };
                        if let Err(err) =
                            ws.write_datetime((index + 1) as _, col as _, &date_time, &date_format)
                        {
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

        wb.push_worksheet(ws);
        wb.save(path).map_err(From::from)
    }
}
