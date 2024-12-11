//! This example is part unit test and part demonstration.
//!
//! We show all of the registration macros, as well as all of the "emission" macros, the ones you
//! would actually call to update a metric.
//!
//! We demonstrate the various permutations of values that can be passed in the macro calls, all of
//! which are documented in detail for the respective macro.
use std::sync::Arc;

use metrics::{
    counter, describe_counter, describe_gauge, describe_histogram, gauge, histogram, KeyName,
    Metadata, SharedString,
};
use metrics::{Counter, CounterFn, Gauge, GaugeFn, Histogram, HistogramFn, Key, Recorder, Unit};

#[derive(Clone, Debug)]
struct PrintHandle(Key);

impl CounterFn for PrintHandle {
    fn increment(&self, value: u64) {
        println!("counter increment for '{}': {}", self.0, value);
    }

    fn absolute(&self, value: u64) {
        println!("counter absolute for '{}': {}", self.0, value);
    }
}

impl GaugeFn for PrintHandle {
    fn increment(&self, value: f64) {
        println!("gauge increment for '{}': {}", self.0, value);
    }

    fn decrement(&self, value: f64) {
        println!("gauge decrement for '{}': {}", self.0, value);
    }

    fn set(&self, value: f64) {
        println!("gauge set for '{}': {}", self.0, value);
    }
}

impl HistogramFn for PrintHandle {
    fn record(&self, value: f64) {
        println!("histogram record for '{}': {}", self.0, value);
    }
}

#[derive(Debug)]
struct PrintRecorder;

impl Recorder for PrintRecorder {
    fn describe_counter(&self, key_name: KeyName, unit: Option<Unit>, description: SharedString) {
        println!(
            "(counter) registered key {} with unit {:?} and description {:?}",
            key_name.as_str(),
            unit,
            description
        );
    }

    fn describe_gauge(&self, key_name: KeyName, unit: Option<Unit>, description: SharedString) {
        println!(
            "(gauge) registered key {} with unit {:?} and description {:?}",
            key_name.as_str(),
            unit,
            description
        );
    }

    fn describe_histogram(&self, key_name: KeyName, unit: Option<Unit>, description: SharedString) {
        println!(
            "(histogram) registered key {} with unit {:?} and description {:?}",
            key_name.as_str(),
            unit,
            description
        );
    }

    fn register_counter(&self, key: &Key, _metadata: &Metadata<'_>) -> Counter {
        Counter::from_arc(Arc::new(PrintHandle(key.clone())))
    }

    fn register_gauge(&self, key: &Key, _metadata: &Metadata<'_>) -> Gauge {
        Gauge::from_arc(Arc::new(PrintHandle(key.clone())))
    }

    fn register_histogram(&self, key: &Key, _metadata: &Metadata<'_>) -> Histogram {
        Histogram::from_arc(Arc::new(PrintHandle(key.clone())))
    }
}

fn init_print_logger() {
    metrics::set_global_recorder(PrintRecorder).unwrap()
}

fn main() {
    let server_name = "web03".to_string();

    init_print_logger();

    let common_labels = &[("listener", "frontend")];

    // Go through describing the metrics:
    describe_gauge!("connection_count", "current number of client connections");
    describe_gauge!("unused_gauge", "some gauge we'll never use in this program");

    let gauge1 = gauge!("test_gauge");
    gauge1.increment(1.0);
    let gauge2 = gauge!("test_gauge", "type" => "decrement");
    gauge2.decrement(1.0);
    let gauge3 = gauge!("test_gauge", "type" => "set");
    gauge3.set(3.1459);

    // All the supported permutations of `gauge!` and its increment/decrement versions:
    gauge!("connection_count").set(300.0);
    gauge!("connection_count", "listener" => "frontend").set(300.0);
    gauge!("connection_count", "listener" => "frontend", "server" => server_name.clone()).set(300.0);
    gauge!("connection_count", common_labels).set(300.0);
    gauge!("connection_count").increment(300.0);
    gauge!("connection_count", "listener" => "frontend").increment(300.0);
    gauge!("connection_count", "listener" => "frontend", "server" => server_name.clone()).increment(300.0);
    gauge!("connection_count", common_labels).increment(300.0);
    gauge!("connection_count").decrement(300.0);
    gauge!("connection_count", "listener" => "frontend").decrement(300.0);
    gauge!("connection_count", "listener" => "frontend", "server" => server_name.clone()).decrement(300.0);
    gauge!("connection_count", common_labels).decrement(300.0);
}