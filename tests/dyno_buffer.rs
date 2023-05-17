use dyno_core::*;

type Buffer = buffer::Buffer<Float>;
lazy_static::lazy_static! {
    static ref BUFFER: Buffer = {
        let mut buffer: Buffer = Buffer::default();
        for _ in 0..99 {
            buffer.push_value(69f64);
        }
        buffer.push_value(420f64);
        buffer
    };
}

#[test]
fn test_buffer_initialize() {
    assert_eq!(BUFFER.first_value(), 69f64);
    assert_eq!(BUFFER.last_value(), 420f64);
    assert_eq!(BUFFER.len(), 100);
    assert_eq!(BUFFER.capacity(), buffer::MAX_CAP_BUFFER);
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
