
use std::sync::{Arc, Mutex};
use std::thread;
use std::io::Write;
use std::io::stdout;


pub fn approx_equal(a: f64, b: f64) -> bool {
	let abs_a = a.abs();
	let abs_b = b.abs();
	let diff = (a - b).abs();

	if a == b {
        // Handle infinities.
		true
	} else if a == 0.0 || b == 0.0 || diff < f64::MIN_POSITIVE {
		// One of a or b is zero (or both are extremely close to it)
        // so use the absolute error
		diff < (f64::EPSILON * f64::MIN_POSITIVE)
	} else {
        // Use relative error.
		(diff / f64::min(abs_a + abs_b, f64::MAX)) < f64::EPSILON
	}
}


/// Times a function with progress reports including an estimation for time remaining
///
/// # Arguments
/// `n` - Number of times to run the function
/// `f` - The function, possibly a closure,
///     taking no arguments, returning a `Result<(), String>`
pub fn timer<F>(n: usize, f: F) -> Result<(), String>
where F: Fn() -> Result<(), String> + Send + Sync + 'static
{
    println!("Running...");
    // spawn two threads, one running the function and one for progress output

    // we need two arcs to the loop index for the threads to both access it
    let idx1 = Arc::new(Mutex::new(0));
    let idx2 = idx1.clone();

    let start_time: std::time::Instant = std::time::Instant::now();

    let function_thread = thread::spawn( move || -> Result<(), String> {
        loop {
            // run f()
            f()?;

            // increment i
            let mut guard = idx1.lock().map_err(|e| e.to_string())?;
            *guard += 1;
            if *guard == n {
                break;
            }
        }
        Ok(())
    });

    let print_thread = thread::spawn(move || -> Result<(), String> {
        loop {
            let i;
            {
                // we must put this in another scope in order to release the lock as soon as possible
                let guard = idx2.lock().map_err(|e| e.to_string())?;
                // get the index
                i = (*guard).clone();
            }
            if i == n {
                break;
            }
            // calculate and print progress
            let progress = 100.0 * (i + 1) as f32 / n as f32;
            let time_per_call = (std::time::Instant::now() - start_time) / (i + 1) as u32;
            let time_remaining = (n - i - 1) as u32 * time_per_call;
            print!("\rProgress: {:.2}%. Estimated time remaining: {:.2} s", progress, time_remaining.as_millis() as f32 / 1000.0);
            // we need to flush here to display the text
            stdout().flush().map_err(|e| e.to_string())?;
            thread::sleep(std::time::Duration::new(1, 0));
        }
        Ok(())
    });

    function_thread.join().expect("Function thread panicked!")?;
    print_thread.join().expect("Print thread panicked!")?;
    println!("\rDone. Execution took {:.2} s", (std::time::Instant::now() - start_time).as_millis() as f32 / 1000.0);

    Ok(())
}