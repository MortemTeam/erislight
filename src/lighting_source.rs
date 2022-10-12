use crate::*;

pub mod pubs {
    use crate::{Value, DMResult};

    pub fn vis_update(src: &Value, usr: &Value) -> DMResult {
        return super::vis_update(src, usr, vec![]);
    }

    pub fn recalc_corner(src: &Value, usr: &Value, C: &Value) -> DMResult {
        return super::recalc_corner(src, usr, vec![C.clone()]);
    }
}

#[hook("/datum/light_source/proc/destroy")]
fn destroy() {
    src.set(byond_string!("destroyed"), Value::from(true));
    force_update(src, usr, vec![]);

    let source_atom = src.get(byond_string!("source_atom"))?;
    if source_atom.is_truthy() {
        let sa_light_sources = source_atom.get(byond_string!("light_sources"))?;
        if sa_light_sources.is_truthy() {
            sa_light_sources.as_list()?.remove(src);
            src.set(byond_string!("source_atom"), Value::null());
        }
    }

    let top_atom = src.get(byond_string!("top_atom"))?;
    if top_atom.is_truthy() {
        let ta_light_sources = source_atom.get(byond_string!("light_sources"))?;
        if ta_light_sources.is_truthy() {
            ta_light_sources.as_list()?.remove(src);
            src.set(byond_string!("top_atom"), Value::null());
        }
    }
    
    Ok(Value::null())
}

fn effect_update(src: &Value) {
    if ! src.get(byond_string!("needs_update")).unwrap().is_truthy() {
        Value::globals().get_list(byond_string!("lighting_update_lights")).unwrap()
            .append(src);
    
        src.set(byond_string!("needs_update"), Value::from(true));
    }
}

/*
#[hook("/datum/light_source/proc/update")]
fn update(new_top_atom: &Value) {
    let mut top_atom = &src.get(byond_string!("top_atom"))?;
    if new_top_atom.is_truthy() && new_top_atom != top_atom {
        let source_atom = &src.get(byond_string!("source_atom"))?;
        if top_atom != source_atom {
            top_atom.get_list(byond_string!("light_sources"))?.remove(src);
        }

        top_atom = new_top_atom;

        if top_atom != source_atom {
            let ta_light_sources = match top_atom.get(byond_string!("light_sources"))?.as_list() {
                Ok(list) => list,
                Err(_) => {
                    let list = List::new();
                    top_atom.set(byond_string!("light_sources"), &list);
                    list
                },
            };

            ta_light_sources.append(src);
        }
    }

    effect_update(src);
    Ok(Value::null())
}
*/

#[hook("/datum/light_source/proc/force_update")]
fn force_update() {
    src.set(byond_string!("force_update"), Value::from(true));

    effect_update(src);
    Ok(Value::null())
}

#[hook("/datum/light_source/proc/vis_update")]
fn vis_update() {
    src.set(byond_string!("vis_update"), Value::from(true));

    effect_update(src);
    Ok(Value::null())
}

/*
#[hook("/datum/light_source/proc/check")]
fn check() {
    let source_atom = src.get(byond_string!("source_atom"))?;
    let mut light_range = &src.get(byond_string!("light_range"))?;
    let mut light_power = &src.get(byond_string!("light_power"))?;

    if ! source_atom.is_truthy() || ! light_range.is_truthy() || ! light_power.is_truthy() {
        destroy(src, usr, vec![]);
        return Ok(Value::from(true));
    }

    let mut ret = false;
    let mut top_atom = &src.get(byond_string!("top_atom"))?;
    let source_atom = &src.get(byond_string!("source_atom"))?;

    if ! top_atom.is_truthy() {
        src.set(byond_string!("top_atom"), source_atom);
        top_atom = source_atom;
        ret = true;
    }

    let mut source_turf = &src.get(byond_string!("source_turf"))?;
    let top_atom_loc = &top_atom.get(byond_string!("loc"))?;
    if istype(top_atom, "/turf") {
        if source_turf != top_atom {
            src.set(byond_string!("source_turf"), top_atom);
            source_turf = top_atom;
            ret = true;
        }
    } else if top_atom_loc != source_turf {
        src.set(byond_string!("source_turf"), top_atom_loc);
        source_turf = top_atom_loc;
        ret = true;
    }

    let sa_light_power = &source_atom.get(byond_string!("light_power"))?;
    if sa_light_power != light_power {
        src.set(byond_string!("light_power"), sa_light_power);
        light_power = sa_light_power;
        ret = true;
    }

    let sa_light_range = &source_atom.get(byond_string!("light_range"))?;
    if sa_light_range != light_range {
        src.set(byond_string!("light_range"), sa_light_range);
        light_range = sa_light_range;
        ret = true;
    }

    if light_range.is_truthy() && light_power.is_truthy() && ! src.get(byond_string!("applied"))?.is_truthy() {
        ret = true;
    }

    let sa_light_color = &source_atom.get(byond_string!("light_color"))?;
    let mut light_color = &src.get(byond_string!("light_color"))?;
    if sa_light_color != light_color {
        src.set(byond_string!("light_color"), sa_light_color);
        light_color = sa_light_color;
        parse_light_color(src, usr, vec![]);
        ret = true;
    }

    Ok(Value::from(ret))
}
*/

/*
#[hook("/datum/light_source/proc/parse_light_color")]
fn parse_light_color() {
    match src.get_string(byond_string!("light_color")) {
        Ok(light_color) => {
            src.set(byond_string!("lum_r"), Value::from(GetRedPart(&light_color) as f32 / 255.0));
            src.set(byond_string!("lum_g"), Value::from(GetGreenPart(&light_color) as f32 / 255.0));
            src.set(byond_string!("lum_b"), Value::from(GetBluePart(&light_color) as f32 / 255.0));
        },
        Err(_) => {
            src.set(byond_string!("lum_r"), Value::from(1));
            src.set(byond_string!("lum_g"), Value::from(1));
            src.set(byond_string!("lum_b"), Value::from(1));
        },
    }

    Ok(Value::null())
}
*/

/*
static mut update_gen: f32 = 1.;
#[hook("/datum/light_source/proc/apply_lum")]
fn apply_lum() {
    unsafe { 
        src.set(byond_string!("applied"), Value::from(true));

        src.set(byond_string!("applied_lum_r"), &src.get(byond_string!("lum_r"))?);
        src.set(byond_string!("applied_lum_g"), &src.get(byond_string!("lum_g"))?);
        src.set(byond_string!("applied_lum_b"), &src.get(byond_string!("lum_b"))?);

        for T in FOR_DVIEW("/turf",
            &src.get(byond_string!("light_range"))?,
            &src.get(byond_string!("source_turf"))?,
            &Value::from(INVISIBILITY_LIGHTING),
        ) {
            if ! T.get(byond_string!("lighting_corners_initialised"))?.is_truthy() {
                lighting_turf::pubs::generate_missing_corners(&T, usr);
            }

            let effect_str = src.get_list(byond_string!("effect_str"))?;
            let affecting_turfs = src.get_list(byond_string!("affecting_turfs"))?;
            lighting_turf::pubs::get_corners(&T, usr).unwrap().as_list().and_then(|corners| {
                for C in ListIterator::from(corners) {
                    if C.get_number(byond_string!("update_gen"))? == update_gen {
                        continue;
                    }

                    C.set(byond_string!("update_gen"), Value::from(update_gen));
                    C.get_list(byond_string!("affecting"))?.append(src);

                    if ! C.get(byond_string!("active"))?.is_truthy() {
                        effect_str.set(C, Value::from(0.0));
                        continue;
                    }

                    APPLY_CORNER(src, &C);
                }

                Ok(())
            });

            if ! T.get(byond_string!("affecting_lights"))?.is_truthy() {
                T.set(byond_string!("affecting_lights"), List::new());
            }

            T.get_list(byond_string!("affecting_lights"))?.append(src);
            affecting_turfs.append(T);
        } END_FOR_DVIEW();

        update_gen += 1.;
    }

    Ok(Value::null())
}
*/

/*
#[hook("/datum/light_source/proc/remove_lum")]
fn remove_lum() {
    src.set(byond_string!("applied"), Value::from(false));

    let affecting_turfs = src.get(byond_string!("affecting_turfs"))?;

    for T in ListIterator::from(affecting_turfs.clone().as_list()?) {
        let T_affecting_lights = T.get(byond_string!("affecting_lights"))?;
        if T_affecting_lights.is_truthy() {
            T_affecting_lights.as_list()?.remove(src);
        }
    }

    Value::from(affecting_turfs).call("Cut", &[]);

    let effect_str = src.get(byond_string!("effect_str"))?;
    for C in ListIterator::from(effect_str.clone().as_list()?) {
        REMOVE_CORNER(src, &C);
        C.get_list(byond_string!("affecting"))?.remove(src);
    }

    Value::from(effect_str).call("Cut", &[]);

    Ok(Value::null())
}
*/

#[hook("/datum/light_source/proc/recalc_corner")]
fn recalc_corner(C: &Value) {
    let effect_str = src.get_list(byond_string!("effect_str"))?;

    ListIterator::from(effect_str).find(|x| x == C)
        .and_then(|_| Some(REMOVE_CORNER(src, C)));

    APPLY_CORNER(src, C);

    Ok(Value::null())
}

/*
#[hook("/datum/light_source/proc/smart_vis_update")]
fn smart_vis_update() {
    let mut corners = Vec::new();
    let mut turfs   = Vec::new();

    for T in FOR_DVIEW("/turf",
        &src.get(byond_string!("light_range"))?,
        &src.get(byond_string!("source_turf"))?,
        &Value::from(0),
    ) {
        if ! T.get(byond_string!("lighting_corners_initialised"))?.is_truthy() {
            lighting_turf::pubs::generate_missing_corners(&T, usr);
        }

        corners.extend(ListIterator::from(
            lighting_turf::pubs::get_corners(&T, usr)?.as_list()?
        ));

        turfs.push(T);
    } END_FOR_DVIEW();

    // New turfs, add us to the affecting lights of them.
    let mut L = Vec::new();
    let affecting_turfs = src.get_list(byond_string!("affecting_turfs"))?;
    for T in ListIterator::from(affecting_turfs) {
        if ! turfs.contains(&T) {
            L.push(T);
        }
    }

    let affecting_turfs = src.get_list(byond_string!("affecting_turfs"))?;
    for T in L.iter() {
        affecting_turfs.append(T);
        let T_affecting_lights = T.get(byond_string!("affecting_lights"))?;
        if ! T_affecting_lights.is_truthy() {
            let list = List::new();
            list.append(src);
            T.set(byond_string!("affecting_lights"), list);
        } else {
            T_affecting_lights.as_list()?.append(src);
        }
    }

    // Now-gone turfs, remove us from the affecting lights.
    L.clear();

    let affecting_turfs = src.get_list(byond_string!("affecting_turfs"))?;
    let vec_affecting_turfs = Vec::from_iter(ListIterator::from(affecting_turfs));
    for T in turfs {
        if ! vec_affecting_turfs.contains(&T) {
            L.push(T)
        }
    }

    let affecting_turfs = src.get_list(byond_string!("affecting_turfs"))?;
    for T in L {
        affecting_turfs.remove(&T);
        let T_affecting_lights = T.get(byond_string!("affecting_lights"))?;
        if T_affecting_lights.is_truthy() {
            T_affecting_lights.as_list()?.remove(src);
        }
    }

    let vec_effect_str = Vec::from_iter(
        ListIterator::from(src.get_list(byond_string!("effect_str"))?)
    );

    let effect_str = src.get_list(byond_string!("effect_str"))?;
    for C in corners.uniq(vec_effect_str.clone()) {
        C.get_list(byond_string!("affecting"))?.append(src);
        if ! C.get(byond_string!("active"))?.is_truthy() {
            effect_str.set(C, Value::from(0));
            continue;
        }

        APPLY_CORNER(src, &C);
    }

    for C in vec_effect_str.uniq(corners.clone()) {
        REMOVE_CORNER(src, &C);
        C.get_list(byond_string!("affecting"))?.remove(src);
        effect_str.remove(C);
    }

    Ok(Value::null())
}
*/