use crate::*;

#[hook("/proc/create_all_lighting_overlays")]
fn create_all_lighting_overlays() {
    for T in world_turfs() {
        if ! T.get(byond_string!("dynamic_lighting"))?.is_truthy() {
            continue
        }

        let A = T.get(byond_string!("loc"))?;
        if ! A.get(byond_string!("dynamic_lighting"))?.is_truthy() {
            continue
        }

        make_new("/atom/movable/lighting_overlay", vec![&T, &Value::from(true)]);

        if ! T.get(byond_string!("lighting_corners_initialised"))?.is_truthy() {
            lighting_turf::pubs::generate_missing_corners(&T, usr);
        }
    }

    Ok(Value::null())
}

// /proc/create_lighting_overlays_zlevel
// Is shit