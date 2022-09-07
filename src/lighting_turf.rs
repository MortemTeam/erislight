use crate::*;

#[hook("/turf/proc/reconsider_lights")]
fn reconsider_lights() {
    let affecting_lights = src.get(byond_string!("affecting_lights"))?;
    if affecting_lights.is_truthy() {
        for L in ListIterator::from(affecting_lights.as_list()?) {
            L.call("vis_update", &[]);
        }
    }

    Ok(Value::null())
}

#[hook("/turf/proc/lighting_clear_overlay")]
fn lighting_clear_overlay() {
    let lighting_overlay = src.get(byond_string!("lighting_overlay"))?;
    if lighting_overlay.is_truthy() {
        qdel(&lighting_overlay);
    }

    for C in ListIterator::from(src.get_list(byond_string!("corners"))?) {
        C.call("update_active", &[]);
    }

    Ok(Value::null())
}

#[hook("/turf/proc/lighting_build_overlay")]
fn lighting_build_overlay() {
    if src.get(byond_string!("lighting_overlay"))?.is_truthy() {
        return Ok(Value::null());
    }

    if ! src.get(byond_string!("loc"))?.get(byond_string!("dynamic_lighting"))?.is_truthy() {
        return Ok(Value::null());
    }

    if ! src.get(byond_string!("lighting_corners_initialised"))?.is_truthy() {
        generate_missing_corners(src, usr, vec![]);
    }

    make_new("/atom/movable/lighting_overlay", vec![src.clone()]);

    for C in ListIterator::from(src.get_list(byond_string!("corners"))?) {
        if ! C.get(byond_string!("active"))?.is_truthy() {
            for S in ListIterator::from(C.get_list(byond_string!("affecting"))?) {
                S.call("recalc_corner", &[&C]);
            }

            C.set(byond_string!("active"), Value::from(true));
        }
    }

    Ok(Value::null())
}

#[hook("/turf/proc/get_lumcount")]
fn get_lumcount(minlum: Value, maxlum: Value) {
    let minlum = minlum.as_number().unwrap_or_else(|_| 0.0);
    let maxlum = maxlum.as_number().unwrap_or_else(|_| 1.0);

    if ! src.get(byond_string!("lighting_overlay"))?.is_truthy() {
        return Ok(Value::from(0.5));
    }

    let mut totallums = 0.0;
    for v in ListIterator::from(src.get_list(byond_string!("corners"))?) {
        for lum_name in ["lum_r", "lum_b", "lum_g"] {
            totallums += v.get_number(StringRef::new(lum_name)?)?;
        }
    }

    totallums /= 12.0;

    
    totallums = (totallums - minlum) / (maxlum - minlum);

    Ok(Value::from(totallums.clamp(0.0, 1.0)))
}

#[hook("/turf/proc/recalc_atom_opacity")]
fn recalc_atom_opacity() {
    src.set(byond_string!("has_opaque_atom"), Value::from(false));

    for A in ListIterator::from(src.get_list(byond_string!("contents"))?) {
        if A.get(byond_string!("opacity"))?.is_truthy() {
            src.set(byond_string!("has_opaque_atom"), Value::from(true));
            return Ok(Value::null());
        }
    }

    src.set(byond_string!("has_opaque_atom"), src.get(byond_string!("opacity"))?);
    Ok(Value::null())
}

// /turf/Entered(var/atom/movable/Obj, var/atom/OldLoc)
// /turf/Exited(var/atom/movable/Obj, var/atom/newloc)
// Not realized because ..() not implemented in auxtools

#[hook("/turf/change_area")]
fn change_area(old_area: Value, new_area: Value) {
    let new_area_dynamic_lighting = new_area.get(byond_string!("dynamic_lighting"))?;
    if new_area_dynamic_lighting != old_area.get(byond_string!("dynamic_lighting"))? {
        match new_area_dynamic_lighting.is_truthy() {
            true => lighting_build_overlay(src, usr, vec![]),
            false => lighting_clear_overlay(src, usr, vec![]),
        };
    }

    Ok(Value::null())
}

#[hook("/turf/proc/get_corners")]
fn get_corners(dir: Value) {
    Ok(if src.get(byond_string!("has_opaque_atom"))?.is_truthy() {
        Value::null()
    } else {
        src.get(byond_string!("corners"))?
    })
}

#[hook("/turf/proc/generate_missing_corners")]
fn generate_missing_corners() {
    src.set(byond_string!("lighting_corners_initialised"), Value::from(true));

    if ! src.get(byond_string!("corners"))?.is_truthy() {
        src.set(byond_string!("corners"), List::with_size(4));
    }
    
    let LIGHTING_CORNER_DIAGONAL = Value::globals().get_list(byond_string!("LIGHTING_CORNER_DIAGONAL"))?;
    let corners = src.get_list(byond_string!("corners"))?;
    for i in 1..=4 { // for (var/i = 1 to 4) 
        if !corners.get(i)?.is_truthy() {
            corners.set(i, 
                make_new("/datum/lighting_corner", vec![src.clone(), LIGHTING_CORNER_DIAGONAL.get(i)?])
                .unwrap()
            );
        }
    }

    Ok(Value::null())
}
