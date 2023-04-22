use std::path::PathBuf;

use dyno_types::{
    convertions::temperature::Celcius,
    data_buffer::{BufferData, Data},
    infomotor::{Cylinder, InfoMotor, Stroke, Transmition},
    FloatMath, Numeric, SerialData,
};
use lazy_static::lazy_static;

const MANIFEST_DIR: &str = env!("CARGO_MANIFEST_DIR");

const SER_DATA: SerialData = SerialData {
    time: 200,
    pulse_encoder: 4200,
    pulse_rpm: 69,
    temperature: 420f32,
};
const INFO_MOTOR: InfoMotor = InfoMotor {
    name: String::new(),
    cc: 4,
    cylinder: Cylinder::Single,
    stroke: Stroke::Four,
    transmition: Transmition::Manual(4),
    tire_diameter: 16.0,
};

lazy_static! {
    static ref DEFAULT_DATA_BUFFER: BufferData = {
        let mut buffer = BufferData::new();
        let data = Data::from_serial(&INFO_MOTOR, SER_DATA);
        for _ in 0..100 {
            buffer.push_data(data.clone());
        }
        buffer
    };
}

macro_rules! asserts_data {
    ($data: ident, $odo: literal) => {{
        assert_eq!($data.rpm.to_float().round(), 20700., "data rpm asserts");
        assert_eq!(
            $data.odo.to_float().round_decimal(2),
            $odo,
            "data odo asserts"
        );
        assert_eq!(
            $data.speed.to_float().round_decimal(2),
            105.55,
            "data speed asserts"
        );
        assert_eq!($data.temp, Celcius::new(420.), "data temp asserts");

        // TODO: torque and horsepower implementation
        assert_eq!($data.torque, 0.0);
        assert_eq!($data.horsepower, 0.0);
    }};

    ($data: ident) => {
        asserts_data!($data, 0.58)
    };
}

#[test]
fn test_calculate_data() {
    let data = Data::from_serial(&INFO_MOTOR, SER_DATA);
    assert_eq!(data.rpm.to_float().round(), 20700., "data rpm asserts");
    assert_eq!(
        data.odo.to_float().round_decimal(4),
        0.0058,
        "data odo asserts"
    );
    assert_eq!(
        data.speed.to_float().round_decimal(2),
        105.55,
        "data speed asserts"
    );
    assert_eq!(data.temp, Celcius::new(420.), "data temp asserts");

    // TODO: torque and horsepower implementation
    assert_eq!(data.torque, 0.0);
    assert_eq!(data.horsepower, 0.0);
}
#[test]
fn test_data_buffer() {
    assert_eq!(DEFAULT_DATA_BUFFER.len(), 100);
    let data = DEFAULT_DATA_BUFFER.last();
    asserts_data!(data);
}

#[test]
fn test_save_binaries() {
    let path = PathBuf::from(MANIFEST_DIR).join("tests/files/test_bin.bin");
    if !path.exists() {
        match DEFAULT_DATA_BUFFER.serialize_to_file(&path) {
            Ok(k) => k,
            Err(err) => panic!("ERROR: {err}"),
        }
    }
    assert!(path.exists());
    assert!(path.is_file());
}
#[test]
fn test_open_binaries() {
    let path = PathBuf::from(MANIFEST_DIR).join("tests/files/test_bin.bin");
    std::thread::sleep(std::time::Duration::from_secs(1));
    assert!(path.exists());
    let buffer_data = match BufferData::deserialize_from_file(path) {
        Ok(ok) => ok,
        Err(err) => panic!("ERROR: {err}"),
    };

    assert_eq!(buffer_data.len(), 100);
    let data = buffer_data.last();
    asserts_data!(data);
}

#[test]
fn test_save_csv() {
    let path = PathBuf::from(MANIFEST_DIR).join("tests/files/test_csv.csv");
    if !path.exists() {
        match DEFAULT_DATA_BUFFER.save_as_csv(&path) {
            Ok(k) => k,
            Err(err) => panic!("ERROR: {err}"),
        }
    }
    assert!(path.exists());
    assert!(path.is_file());
}

#[test]
fn test_open_csv() {
    let path = PathBuf::from(MANIFEST_DIR).join("tests/files/test_csv.csv");
    std::thread::sleep(std::time::Duration::from_secs(1));
    assert!(path.exists());
    let buffer_data = match BufferData::open_from_csv(&path) {
        Ok(k) => k,
        Err(err) => panic!("ERROR: {err}"),
    };
    assert_eq!(buffer_data.len(), 100);
    let data = buffer_data.last();
    asserts_data!(data, 0.0);
}
#[test]
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

#[test]
fn test_open_excel() {
    let path = PathBuf::from(MANIFEST_DIR).join("tests/files/test_excel.xlsx");
    std::thread::sleep(std::time::Duration::from_secs(1));
    assert!(path.exists());
    let buffer_data = match BufferData::open_from_excel(&path) {
        Ok(k) => k,
        Err(err) => panic!("ERROR: {err}"),
    };
    assert_eq!(buffer_data.len(), 100);
    let data = buffer_data.last();
    asserts_data!(data, 0.0);
}
