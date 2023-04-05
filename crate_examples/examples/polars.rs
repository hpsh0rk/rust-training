use polars::prelude::*;
use sysinfo::{ProcessExt, System, SystemExt};

fn main() {
    // Create a new system object
    let system = System::new_all();

    // Initialize new Polars DataFrame
    let mut df = DataFrame::new(vec![
        Series::new("pid", &[] as &[String]),
        Series::new("name", &[] as &[String]),
        Series::new("cpu_percent", &[] as &[f32]),
        Series::new("memory", &[] as &[u64]),
    ])
    .unwrap();

    // Iterate over all currently running processes
    for (pid, process) in system.processes() {
        let name = process.name().to_string();
        let cpu_percent = process.cpu_usage();
        let memory = process.memory();

        let df1 = df!(
                    "pid" => &[pid.to_string()],
                    "name" => &[name],
                    "cpu_percent" =>  &[cpu_percent],
                    "memory" => &[memory],
        )
        .unwrap();
        df = df.vstack(&df1).unwrap()
    }

    // Print the Polars DataFrame
    println!("{}", df);
}
