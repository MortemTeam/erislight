use crate::*;

// #define TURF_HAS_VALID_ZONE(T) (istype(T, /turf/simulated) && T:zone && !T:zone:invalid)
pub fn TURF_HAS_VALID_ZONE(T: &Value) -> bool {
    return if T.is_exact_type("/turf/simulated") && 
        let zone = T.get(byond_string!("zone")).unwrap() && zone.is_truthy() &&
            ! zone.get(byond_string!("invalid")).unwrap().is_truthy() {
                 true
            } else {
                false
            }
}

// #define RANGE_TURFS(RADIUS, CENTER) \
pub fn RANGE_TURF(RADIUS: u32, CENTER: &Value) -> Value {
    let (x, y, z) = value_loc(CENTER);

    let world = Value::world();
    let maxx = world.get_number(byond_string!("maxx")).unwrap() as u32;
    let maxy = world.get_number(byond_string!("maxy")).unwrap() as u32;

    return Proc::find("/proc/block").unwrap().call(&[
        &Value::turf(max(x - RADIUS, 1),    max(y - RADIUS, 1),    z).unwrap(),
        &Value::turf(min(x - RADIUS, maxx), min(y - RADIUS, maxy), z).unwrap(),
    ]).unwrap();
}
