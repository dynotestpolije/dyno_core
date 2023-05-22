use dyno_core::*;

type Buf = Buffer<Float>;
lazy_static::lazy_static! {
    static ref BUFFER: Buf = {
        let mut buffer: Buf = Buf::default();
        for _ in 0..99 {
            buffer.push(69f64);
        }
        buffer.push(420f64);
        buffer
    };
}

#[test]
fn test_buffer_initialize() {
    assert_eq!(BUFFER.first_value(), 69f64);
    assert_eq!(BUFFER.last_value(), 420f64);
    assert_eq!(BUFFER.len(), 100);
    assert_eq!(BUFFER.capacity(), Buffer::<Float>::MAX_CAP_BUFFER);
}

#[test]
fn test_buffer_sum() {
    assert_eq!(BUFFER.sum_value(), 7251f64);
}
#[test]
fn test_buffer_avg() {
    assert_eq!(BUFFER.avg_value(), 72.51f64);
}

#[test]
fn test_buffer_minmax() {
    assert_eq!(BUFFER.len(), 100);
    assert_eq!(BUFFER.min_value(), 69f64);
    assert_eq!(BUFFER.max_value(), 420f64);
}
