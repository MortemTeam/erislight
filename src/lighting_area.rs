use crate::*;

#[hook("/area/proc/set_dynamic_lighting")]
fn set_dynamic_lighting(new_dynamic_lighting: Value) {
    if new_dynamic_lighting == &src.get(byond_string!("new_dynamic_lighting"))? {
        return Ok(Value::from(false));
    }

    src.set(byond_string!("dynamic_lighting"), new_dynamic_lighting);

    if new_dynamic_lighting.is_truthy() {
        for T in ListIterator::from(Value::world().as_list()?) {
            if local_ispath(&T, "/turf") && T.get(byond_string!("dynamic_lighting"))?.is_truthy() {
                lighting_turf::pubs::lighting_build_overlay(src, usr);
            }
        }
    } else {
        for T in ListIterator::from(Value::world().as_list()?) {
            if local_ispath(&T, "/turf") && T.get(byond_string!("lighting_overlay"))?.is_truthy() {
                lighting_turf::pubs::lighting_clear_overlay(src, usr);
            }
        }
    }

    Ok(Value::from(true))
}
