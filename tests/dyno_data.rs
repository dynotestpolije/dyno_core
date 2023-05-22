use std::path::PathBuf;

use dyno_core::*;
use lazy_static::lazy_static;

const SIZE_TESTED: usize = 100_000;
const MANIFEST_DIR: &str = env!("CARGO_MANIFEST_DIR");

const SER_DATA: SerialData = SerialData {
    time: 1000,
    pulse_enc: 4200,
    pulse_rpm: 69,
    temperature: 420f32,
    pulse_enc_max: 360,
    period: 200,
    pulse_enc_raw: 4200 / 4,
    pulse_enc_z: 4200 / 360,
};

lazy_static! {
    static ref DEFAULT_DATA_BUFFER: BufferData = {
        let mut buffer = BufferData::new();
        let data = Data::from_serial(buffer.last(), &CONFIG, SER_DATA);
        for _ in 0..SIZE_TESTED {
            buffer.push_data(data.clone());
        }
        buffer
    };
    static ref CONFIG: dyno_core::DynoConfig = dyno_core::DynoConfig::default();
}

macro_rules! asserts_data {
    ($data: ident, $odo: literal) => {{
        assert_eq!(
            $data.rpm_roda.to_float().round(),
            3500.,
            "data rpm roda asserts"
        );
        assert_eq!(
            $data.rpm_engine.to_float().round(),
            41400.,
            "data rpm engine asserts"
        );
        assert_eq!(
            $data.odo.to_float().round_decimal(4),
            $odo,
            "data odo asserts"
        );
        assert_eq!(
            $data.speed.to_float().round_decimal(2),
            93.81,
            "data speed asserts"
        );
        assert_eq!($data.temp, Celcius::new(420.), "data temp asserts");

        // TODO: torque and horsepower implementation
        assert_eq!($data.torque.round_decimal(1), 48.1, "data torque asserts");
        assert_eq!(
            $data.horsepower.round_decimal(1),
            17.6,
            "data horsepower asserts"
        );
    }};

    ($data: ident) => {
        asserts_data!($data, 521.1902)
    };
}

#[test]
fn test_data_buffer() {
    assert_eq!(DEFAULT_DATA_BUFFER.len(), SIZE_TESTED);
    let data = DEFAULT_DATA_BUFFER.last();
    asserts_data!(data);
}

fn test_save_compressed() {
    let path = PathBuf::from(MANIFEST_DIR).join("tests/files/test_bin.dyno");
    if !path.exists() {
        match DEFAULT_DATA_BUFFER.compress_to_file(&path) {
            Ok(k) => k,
            Err(err) => panic!("ERROR: {err}"),
        }
    }
    assert!(path.is_file());
}
fn test_open_compressed() {
    let path = PathBuf::from(MANIFEST_DIR).join("tests/files/test_bin.dyno");
    std::thread::sleep(std::time::Duration::from_secs(1));
    let buffer_data = match BufferData::decompress_from_file(path) {
        Ok(ok) => ok,
        Err(err) => panic!("ERROR: {err}"),
    };

    assert_eq!(buffer_data.len(), SIZE_TESTED);
    let data = buffer_data.last();
    asserts_data!(data);
}
#[test]
fn test_compressed_data_buffer() {
    test_save_compressed();
    test_open_compressed();
}

fn test_save_csv() {
    let path = PathBuf::from(MANIFEST_DIR).join("tests/files/test_csv.csv");
    if !path.exists() {
        match DEFAULT_DATA_BUFFER.save_as_csv(&path) {
            Ok(k) => k,
            Err(err) => panic!("{err}"),
        }
    }
    assert!(path.exists());
    assert!(path.is_file());
}

fn test_open_csv() {
    let path = PathBuf::from(MANIFEST_DIR).join("tests/files/test_csv.csv");
    std::thread::sleep(std::time::Duration::from_secs(1));
    assert!(path.exists());
    let buffer_data = match BufferData::open_from_csv(&path) {
        Ok(k) => k,
        Err(err) => panic!("ERROR: {err}"),
    };
    assert_eq!(buffer_data.len(), SIZE_TESTED);
    let data = buffer_data.last();
    asserts_data!(data, 0.0);
}

#[test]
fn test_csv_data_buffer() {
    test_save_csv();
    test_open_csv();
}

fn test_save_excel() {
    let path = PathBuf::from(MANIFEST_DIR).join("tests/files/test_excel.xlsx");
    if !path.exists() {
        match DEFAULT_DATA_BUFFER.save_as_excel(&path) {
            Ok(k) => k,
            Err(err) => panic!("ERROR: {err}"),
        }
    }
    assert!(path.exists());
    assert!(path.is_file());
}

fn test_open_excel() {
    let path = PathBuf::from(MANIFEST_DIR).join("tests/files/test_excel.xlsx");
    std::thread::sleep(std::time::Duration::from_secs(1));
    assert!(path.exists());
    let buffer_data = match BufferData::open_from_excel(&path) {
        Ok(k) => k,
        Err(err) => panic!("ERROR: {err}"),
    };
    assert_eq!(buffer_data.len(), SIZE_TESTED);
    let data = buffer_data.last();
    asserts_data!(data, 0.0);
}

#[test]
fn test_excel_data_buffer() {
    test_save_excel();
    test_open_excel();
}
