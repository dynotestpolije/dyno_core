use std::{
    fs::File,
    io::{BufRead, BufReader, BufWriter, Write},
    ops::{Index, IndexMut},
    path::Path,
};

use super::buffer::Buffer;
use crate::{
    buffer::ValIdx, convertions::prelude::*, infomotor::InfoMotor, Float, Numeric, ResultHandler,
};
use bincode::Options;
use chrono::{Local, NaiveDateTime};

#[derive(Debug, Default, Clone)]
pub struct Data {
    pub speed: KilometresPerHour,
    pub torque: Float,
    pub horsepower: Float,
    pub temp: Celcius,
    pub time_stamp: NaiveDateTime,
    pub rpm: RotationPerMinute,
    pub odo: KiloMetres,
}
impl Data {
    pub fn from_serial(info: &'_ InfoMotor, serial_data: super::SerialData) -> Self {
        let super::SerialData {
            time,
            pulse_encoder,
            pulse_rpm,
            temperature,
        } = serial_data;

        // TODO: hardcoded full pulse revolution set it in different data structure that configurable
        const FULL_ROTATION_PULSE: Float = 360.0;

        let delta_ms = time as Float;
        // TODO: not tire_diameter but, encoder drum dirameter
        let tire_diameter = info.tire_diameter as Float;

        let odo = ((pulse_encoder as Float) / FULL_ROTATION_PULSE)
            .araund(tire_diameter)
            .to_kilometres();

        // TODO: rpm hanya sementara
        let rpm = RotationPerMinute::new(pulse_rpm.to_float().per_minute(delta_ms));
        let speed = KilometresPerHour::new(odo.to_float().per_hour(delta_ms));

        // TODO: calculate torque
        let torque = 0.0;
        // TODO: calculate horsepower
        let horsepower = 0.0;

        let temp = Celcius::new(temperature);
        let time_stamp = Local::now().naive_local();

        Self {
            speed,
            odo,
            rpm,
            temp,
            time_stamp,
            torque,
            horsepower,
        }
    }
}

#[derive(Clone, Default)]
#[cfg_attr(feature = "use_serde", derive(serde::Deserialize, serde::Serialize))]
pub struct BufferData {
    pub speed: Buffer<Float>,
    pub rpm: Buffer<Float>,
    pub torque: Buffer<Float>,
    pub horsepower: Buffer<Float>,
    pub temp: Buffer<Float>,
    pub time_stamp: Buffer<Float>,
    pub odo: KiloMetres,
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
        self.len_items() == 0
    }

    #[inline]
    pub fn push_data(&mut self, data: Data) {
        self.speed.push_value(data.speed.to_float());
        self.rpm.push_value(data.rpm.to_float());
        self.torque.push_value(data.torque.to_float());
        self.horsepower.push_value(data.horsepower.to_float());
        self.temp.push_value(data.temp.to_float());
        self.time_stamp
            .push_value(data.time_stamp.timestamp_millis() as _);
        self.odo += data.odo;
        self.len += 1;
    }
    #[inline(always)]
    pub fn push_from_serial(&mut self, info: &InfoMotor, serial_data: crate::SerialData) {
        let data = Data::from_serial(info, serial_data);
        self.push_data(data);
    }

    #[inline]
    pub fn last(&self) -> Data {
        let speed = self.speed.last_value().into();
        let rpm = self.rpm.last_value().into();
        let temp = self.temp.last_value().into();
        let horsepower = self.horsepower.last_value();
        let torque = self.torque.last_value();
        let time_stamp = NaiveDateTime::from_timestamp_millis(self.time_stamp.last_value() as _)
            .unwrap_or_default();

        Data {
            speed,
            torque,
            temp,
            horsepower,
            rpm,
            odo: self.odo,
            time_stamp,
        }
    }
    pub fn len_items(&self) -> usize {
        [
            self.len,
            self.speed.len(),
            self.rpm.len(),
            self.torque.len(),
            self.horsepower.len(),
            self.temp.len(),
            self.time_stamp.len(),
        ]
        .into_iter()
        .min()
        .unwrap_or_default()
    }

    #[inline(always)]
    pub fn get_points<Out>(&self, idx: usize) -> Out
    where
        Out: FromIterator<[crate::Float; 2]>,
    {
        self[idx].into_points()
    }
}

#[cfg(feature = "use_serde")]
impl BufferData {
    #[inline(always)]
    fn bin_option() -> impl bincode::Options {
        bincode::Options::with_big_endian(bincode::Options::allow_trailing_bytes(
            bincode::Options::with_varint_encoding(bincode::options()),
        ))
    }

    #[inline(always)]
    pub fn serialize_bin(&self) -> crate::DynoResult<Vec<u8>> {
        Self::bin_option().serialize(self).dyn_err()
    }

    #[inline(always)]
    pub fn deserialize_bin(bin: &[u8]) -> crate::DynoResult<Self> {
        Self::bin_option().deserialize(bin).dyn_err()
    }

    #[inline(always)]
    pub fn serialize_to_file<P: AsRef<Path>>(&self, path: P) -> crate::DynoResult<()> {
        // let path = path.as_ref().to_path_buf();
        let bin = Self::bin_option().serialize(self)?;
        std::fs::write(path, bin.as_slice()).dyn_err()
    }

    #[inline(always)]
    pub fn deserialize_from_file<'err, P: AsRef<Path>>(path: P) -> crate::DynoResult<'err, Self> {
        let data = std::fs::read(path)?;
        Self::bin_option().deserialize(&data).dyn_err()
    }
}
impl BufferData {
    const CSV_DELIMITER: &'static str = ",";

    pub fn open_from_csv<'err, P: AsRef<Path>>(path: P) -> crate::DynoResult<'err, Self> {
        let mut slf = Self::default();
        let mut lines = BufReader::new(File::open(path)?).lines();

        {
            // ignore header
            let header = lines.next().map(|x| x.ok());
            dbg!(header);
        }

        for line in lines.flatten() {
            for (idx, word) in line.split(Self::CSV_DELIMITER).enumerate() {
                slf.index_mut(idx)
                    .push_value(word.parse().unwrap_or_default());
            }
            slf.len += 1;
        }
        Ok(slf)
    }

    pub fn save_as_csv<P: AsRef<Path>>(&self, path: P) -> crate::DynoResult<()> {
        let mut writer = BufWriter::new(File::create(path)?);
        let len = self.len_items();
        writeln!(&mut writer, "SPEED,RPM,TORQUE,HORSEPOWER,TEMP,TIME")?;
        for idx in 0usize..len {
            writeln!(
                &mut writer,
                "{speed},{rpm},{torque},{horsepower},{temp},{time}",
                speed = self.speed.index(idx).value,
                rpm = self.rpm.index(idx).value,
                torque = self.torque.index(idx).value,
                horsepower = self.horsepower.index(idx).value,
                temp = self.temp.index(idx).value,
                time = self.time_stamp.index(idx).value,
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

    pub fn open_from_excel<'err, P: AsRef<Path>>(path: P) -> crate::DynoResult<'err, Self> {
        use calamine::Reader;
        let mut slf = Self::default();
        let mut wb = calamine::open_workbook_auto(path)?;
        let wb_range = match wb.worksheet_range(Self::EXCEL_SHEET_NAME) {
                Some(Ok(range)) => range,
                Some(Err(err)) => return Err(From::from(err)),
                None => return Err(crate::DynoErr::new(
                    format!("Worksheet `{}` not exists, please add or change the worksheet name to open succesfully open the file", Self::EXCEL_SHEET_NAME),
                    crate::ErrKind::ExcelError
                ))
            };
        let mut rows_iterator = wb_range.rows();
        let _header = rows_iterator.next();
        dbg!(_header);
        for row in rows_iterator {
            let row_len = row.len().min(Self::MAX_COLS_SIZE) - 1;
            for c in 0..row_len {
                slf.index_mut(c).push_value(
                    row[c]
                        .get_float()
                        .unwrap_or_else(|| row[c].get_int().unwrap_or_default() as _),
                );
            }
            slf.time_stamp.push_value(
                row[row_len]
                    .as_datetime()
                    .unwrap_or_default()
                    .timestamp_millis() as _,
            );
            slf.len += 1;
        }
        Ok(slf)
    }

    pub fn save_as_excel<P: AsRef<Path>>(&self, path: P) -> crate::DynoResult<()> {
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
                0..=4 => self[col].iter().for_each(|ValIdx { index, value }| {
                    if let Err(err) = ws.write_number((index + 1) as _, col as _, *value) {
                        log::error!("{err}")
                    }
                }),

                _ => self.time_stamp.iter().for_each(|ValIdx { index, value }| {
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
            }
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

impl Index<usize> for BufferData {
    type Output = Buffer<Float>;
    #[inline(always)]
    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.speed,
            1 => &self.rpm,
            2 => &self.torque,
            3 => &self.horsepower,
            4 => &self.temp,
            5 => &self.time_stamp,
            _ => unreachable!(),
        }
    }
}

impl IndexMut<usize> for BufferData {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.speed,
            1 => &mut self.rpm,
            2 => &mut self.torque,
            3 => &mut self.horsepower,
            4 => &mut self.temp,
            5 => &mut self.time_stamp,
            _ => unreachable!(),
        }
    }
}
