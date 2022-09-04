use auxtools::*;

fn make_new(path: String, args: &[&Value]) -> Option<DMResult>  {
    if let Some(creator) = Proc::find("/proc/make_new") {
        Some(creator.call(&[&[&Value::from_string(path).unwrap()], args].concat()))
    } else {
        None
    }
}

fn log_debug(message: String) {
    if let Some(warner) = Proc::find("/proc/log_debug") {
        warner.call(&[&Value::from_string(message).unwrap()]);
    }
}
