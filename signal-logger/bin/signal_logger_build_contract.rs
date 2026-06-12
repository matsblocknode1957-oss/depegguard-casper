#![no_main]

use signal_logger::SignalLogger;

#[no_mangle]
fn call() {
    SignalLogger::install();
}
