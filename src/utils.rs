use crate::*;

pub fn world_turfs() -> Vec<Value> {
    let world = Value::world();
    let maxx = world.get_number(byond_string!("maxx")).unwrap() as u32;
    let maxy = world.get_number(byond_string!("maxy")).unwrap() as u32;
    let maxz = world.get_number(byond_string!("maxz")).unwrap() as u32;

    local_block(&Value::turf(1, 1, 1).unwrap(), &Value::turf(maxx, maxy, maxz).unwrap())
}

pub fn make_new(path: &str, args: Vec<&Value>) -> Option<Value> {
    if let Some(func) = Proc::find("/proc/make_new") {
        Some(func.call(&[
            &Value::from_string(path).unwrap(), &Value::from(&args)
        ]).unwrap())
    } else {
        None
    }
}

pub fn qdel(v: &Value) -> Option<Value> {
    if let Some(func) = Proc::find("/proc/qdel") {
        Some(func.call(&[v]).unwrap())
    } else {
        None
    }
}

pub fn istype(v: &Value, path: &str) -> bool {
    v.is_exact_type(path)
}

pub fn ispath(v: &Value, path: &str) -> Option<bool> {
    if let Some(func) = Proc::find("/proc/make_ispath") {
        Some(func.call(&[v, &Value::from_string(path).unwrap()]).unwrap().is_truthy())
    } else {
        None
    }
}

pub fn local_ispath(v: &Value, path: &str) -> bool {
    match v.get_type() {
        Err(_) => false,
        Ok(t) => t.starts_with(path),
    }
}

pub fn value_loc(v: &Value) -> (u32, u32, u32) {
    (
        v.get_number(byond_string!("x")).unwrap() as u32,
        v.get_number(byond_string!("y")).unwrap() as u32,
        v.get_number(byond_string!("z")).unwrap() as u32,
    )
}

pub fn local_block(start: &Value, end: &Value) -> Vec<Value> {
    let (sx, sy, sz) = value_loc(start);
    let (ex, ey, ez) = value_loc(end);

    let mut v = Vec::new();
    for z in sz..=ez { for y in sy..=ey { for x in sx..=ex {
        v.push(Value::turf(x, y, z).unwrap());
    }}}

    v
}

pub fn view(dist: &Value, center: &Value) -> Option<Value> {
    if let Some(func) = Proc::find("/proc/make_view") {
        Some(func.call(&[
            dist, center,
        ]).unwrap())
    } else {
        None
    }
}

pub fn block(start: &Value, end: &Value) -> Option<Value> {
    if let Some(func) = Proc::find("/proc/make_block") {
        Some(func.call(&[
            start, end,
        ]).unwrap())
    } else {
        None
    }
}

pub fn locate(x: u32, y: u32, z: u32) -> Option<Value> {
    if let Some(func) = Proc::find("/proc/make_locate") {
        Some(func.call(&[
            &Value::from(x), &Value::from(y), &Value::from(z)
        ]).unwrap())
    } else {
        None
    }
}

pub fn locate_value(v: &Value) -> Option<Value> {
    let (x, y, z) = value_loc(v);
    return locate(x, y, z);
}

pub fn turf_value(v: &Value) -> DMResult {
    let (x, y, z) = value_loc(v);
    Value::turf(x, y, z)
}

pub fn log_debug(message: &str) {
    if let Some(func) = Proc::find("/proc/log_debug") {
        func.call(&[&Value::from_string(message).unwrap()]);
    }
}

pub struct ListIterator {
    list: List,
    current: i32,
}

impl ListIterator {
    pub fn from(list: List) -> Self {
        Self { list: list, current: 0}
    }

    pub fn reset(&mut self) { self.current = 0 }
}

impl Iterator for ListIterator {
    type Item = Value;

    fn next(&mut self) -> Option<Self::Item> {
        self.current += 1;
        match self.list.get(self.current) {
            Ok(value) => Some(value),
            Err(_) => None,
        }
    }
}


pub struct TypedListIterator {
    list: List,
    current: i32,
    var_type: String,
}

impl TypedListIterator {
    pub fn from(var_type: &str, list: List) -> Self {
        Self { list, current: 0,  var_type: var_type.to_string() }
    }
}

impl Iterator for TypedListIterator {
    type Item = Value;

    fn next(&mut self) -> Option<Self::Item> {
        self.current += 1;
        match self.list.get(self.current) {
            Err(_) => None,
            Ok(value) => match local_ispath(&value, &self.var_type) {
                true => Some(value),
                false => self.next(),
            }
        }
    }
}
