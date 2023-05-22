#![cfg(test)]

use dyno_core::*;

#[test]
fn test_tick_around() {
    let full_revolution = 360.0;
    let m: Metres = (10000f64 / full_revolution).araund(0.18.into());
    assert_eq!(m.to_centimetres().round_decimal(2), 1570.79, "odo in Cm");
    assert_eq!(m.round_decimal(2), 15.7, "odo in Meter");
    assert_eq!(m.to_kilometres().round_decimal(2), 0.01, "odo in KiloMeter");
}
#[test]
fn test_angular_velocity() {
    //
    //      ω = θ / t
    //
    // - ω : Angular Velocity
    // - θ : Angular Displacement ( jarak tempuh )
    // - t : time taken by the object in the circular motion
    let time_ms = 100.0; // in ms
    let full_revolution = 360.0;
    // 1570.79 calculate rpm / rps from tick in one second devided by max tick in one full rotation encoder
    let rpm = RotationPerMinute::new((10000f64 / full_revolution).per_minute(time_ms));

    assert_eq!(
        rpm.round_decimal(1),
        16666.6,
        "angular velocity in Revolution Per Minute"
    );
    assert_eq!(
        rpm.to_radians_per_second().round_decimal(2),
        1745.32,
        "angular velocity in Radian Per Second"
    );
}
#[test]
fn test_velocity() {
    let delta_time = 0.1f64;
    let full_revolution = 360.0;
    let odo: Metres = (10000f64 / full_revolution).araund(0.18.into());
    //      s = d / t
    //
    // - s : speed ( kecepatan )
    // - d : distance ( jarak tempuh )
    // - t : time
    let velocity = odo.safe_div(delta_time).map_or(Default::default(), |val| {
        MetresPerSecond::new(val).to_kilometres_per_hour()
    });

    assert_eq!(velocity.round_decimal(2), 565.48, "velocity in km/h");
}
#[test]
fn test_acceleration() {
    let delta_time = 100f64;
    let v0 = 0.0;
    let full_revolution = 360.0;
    let odo = (10000f64 / full_revolution).araund(0.18.into());
    let velocity = MetresPerSecond::from_ms(odo, delta_time);

    // acceleration =  Δv / Δt;
    // acceleration = (v1 - v0) / Δt;
    // acceleration = (velocity - 0) / delta  => contoh
    //                                   (v01        - v0) / Δt
    let delta_v = velocity - v0;
    let acceleration = delta_v.per_second(delta_time);
    assert_eq!(
        acceleration.round_decimal(2),
        1570.79,
        "acceleration in one second (m/s²)"
    );
}

#[test]
fn test_round_decimal() {
    const VALUE: f64 = 69.69696969;
    assert_eq!(VALUE.round_decimal(2), 69.7);
    assert_eq!(VALUE.round_decimal(4), 69.697);
    assert_eq!(VALUE.round_decimal(6), 69.69697);
}
