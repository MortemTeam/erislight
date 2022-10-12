use crate::*;

const LIGHTING_HEIGHT: f32 = 1.0;
pub const INVISIBILITY_LIGHTING: f32 = 20.0;


// #define TURF_HAS_VALID_ZONE(T) (istype(T, /turf/simulated) && T:zone && !T:zone:invalid)
pub fn TURF_HAS_VALID_ZONE(T: &Value) -> bool {
    if T.is_exact_type("/turf/simulated") {
        let zone = T.get(byond_string!("zone")).unwrap();
        if zone.is_truthy() && ! zone.get(byond_string!("invalid")).unwrap().is_truthy() {
            return true
        }
    }

    false
}

// #define RANGE_TURFS(RADIUS, CENTER) \
pub fn RANGE_TURF(RADIUS: u32, CENTER: &Value) -> Value {
    let (x, y, z) = value_loc(CENTER);

    let world = Value::world();
    let maxx = world.get_number(byond_string!("maxx")).unwrap() as u32;
    let maxy = world.get_number(byond_string!("maxy")).unwrap() as u32;

    block(
        &Value::turf(max(x - RADIUS, 1),    max(y - RADIUS, 1),    z).unwrap(),
        &Value::turf(min(x - RADIUS, maxx), min(y - RADIUS, maxy), z).unwrap(),
    ).unwrap()
}

// lighting_source.dm

pub fn APPLY_CORNER(src: &Value, C: &Value) {
    let mut lfalloff = LUM_FALLOFF(src, C, &src.get(byond_string!("source_turf")).unwrap());
    
    lfalloff *= src.get_number(byond_string!("light_power")).unwrap();

    src.get_list(byond_string!("effect_str")).unwrap().set(C, lfalloff);
    UPDATE_LUMCOUNT(src, C, lfalloff);
}

pub fn REMOVE_CORNER(src: &Value, C: &Value) {
    let lfalloff = -src.get_list(byond_string!("effect_str")).unwrap()
        .get(C).unwrap().as_number().unwrap();

    UPDATE_LUMCOUNT(src, C, lfalloff);
}

fn UPDATE_LUMCOUNT(src: &Value, C: &Value, lfalloff: f32) {
    let applied_lum_r = src.get_number(byond_string!("lum_r")).unwrap();
    let applied_lum_g = src.get_number(byond_string!("lum_g")).unwrap();
    let applied_lum_b = src.get_number(byond_string!("lum_b")).unwrap();

    C.call("update_lumcount", &[
        &Value::from(lfalloff * applied_lum_r),
        &Value::from(lfalloff * applied_lum_g),
        &Value::from(lfalloff * applied_lum_b),
    ]);
}

// C - Corner, T - source_turf
pub fn LUM_FALLOFF(src: &Value, C: &Value, T: &Value) -> f32 {
    let light_range = src.get_number(byond_string!("light_range")).unwrap();
    let Cx = C.get_number(byond_string!("x")).unwrap();
    let Tx = T.get_number(byond_string!("x")).unwrap();
    let Cy = C.get_number(byond_string!("y")).unwrap();
    let Ty = T.get_number(byond_string!("y")).unwrap();

    1.0 - (
        (Cx - Tx).powi(2) + (Cy - Ty).powi(2) + LIGHTING_HEIGHT / light_range.max(1.0)
    ).sqrt()
}

pub fn FOR_DVIEW(filter_type: &str, range: &Value, center: &Value, invis_flags: &Value) -> TypedListIterator {
    let dview_mob = Value::world().get(byond_string!("dview_mob")).unwrap();
    dview_mob.set(byond_string!("loc"), center);
    dview_mob.set(byond_string!("see_invisible"), invis_flags);
    TypedListIterator::from(filter_type, view(range, &dview_mob).unwrap().as_list().unwrap())
}

pub fn END_FOR_DVIEW() {
    Value::world().get(byond_string!("dview_mob")).unwrap()
        .set(byond_string!("loc"), Value::null());
}
