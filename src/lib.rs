use auxtools::*;

#[hook("/proc/auxtest_inc_counter")]
fn inc_counter() {
	static mut COUNTER: u32 = 0;

	Ok(Value::from(unsafe {
		COUNTER += 1;
		COUNTER
	}))
}
