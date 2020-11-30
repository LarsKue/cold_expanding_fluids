use std::io::stdout;
use std::io::Write;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;

pub fn square<T: std::marker::Copy + std::ops::Mul<T, Output = T>>(x: T) -> T {
    x * x
}

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
pub fn timer<F>(n: usize, mut f: F) -> Result<(), String>
where
    F: FnMut() -> Result<(), String> + Send,
{
    use rayon::join;
    println!("Running...");

    // spawn two threads, one running the function and one for progress output

    // atomic index
    let idx1 = Arc::new(AtomicUsize::new(0));
    let idx2 = Arc::clone(&idx1);

    let start_time: std::time::Instant = std::time::Instant::now();

    let (a, b) = join(
        || -> Result<(), String> {
            loop {
                // run f
                f()?;

                // increment
                if idx1.fetch_add(1, Ordering::Relaxed) == n {
                    break;
                }
            }
            Ok(())
        },
        || -> Result<(), String> {
            loop {

                let i = idx2.load(Ordering::Relaxed);
                if i >= n {
                    // this must be greater or equal since
                    // 1: we want to avoid overflow, which happens when i == n
                    // 2: after the final function thread loop, i == n + 1
                    break;
                }
                // calculate and print progress
                let progress = 100.0 * (i + 1) as f32 / n as f32;
                let time_per_call = (std::time::Instant::now() - start_time) / (i + 1) as u32;
                let time_remaining = (n - i - 1) as u32 * time_per_call;
                print!(
                    "\rProgress: {:.2}%. Estimated time remaining: {:.2} s",
                    progress,
                    time_remaining.as_millis() as f32 / 1000.0
                );
                // need to flush here to display the text
                stdout().flush().map_err(|e| e.to_string())?;

                // TODO: This thread will continue sleeping even when the function thread has
                //  already finished executing, possibly resulting in overhead up to 250ms long.
                //  A possible solution is to send an exit signal to this thread from the function
                //  thread but this may require the Sync Trait for the function f

                // sleep for a quarter second
                thread::sleep(std::time::Duration::from_millis(250));
            }

            Ok(())
        },
    );

    // if any of the functions returned an Err, show it here
    a?;
    b?;

    println!(
        "\rDone. Execution took {:.2} s",
        (std::time::Instant::now() - start_time).as_millis() as f32 / 1000.0
    );

    Ok(())
}



pub fn cap(f: f64, min: f64, max: f64) -> f64 {
    if f > max {
        max
    } else if f < min {
        min
    } else {
        f
    }
}