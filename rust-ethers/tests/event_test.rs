use std::collections::HashSet;
use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};
use std::thread;
use std::time::Duration;

#[test]
fn test_event_polling() {
    println!("Starting rust event listener");
    // Start the Rust binary in the background
    let mut rust_process = Command::new("cargo")
        .args(["run"])
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start rust process");

    let rust_stdout = rust_process
        .stdout
        .take()
        .expect("Failed to capture stdout");
    let rust_reader = BufReader::new(rust_stdout);
    let mut collected_events = HashSet::new();

    // Give the Rust process a moment to start up
    thread::sleep(Duration::from_secs(2));
    println!("Rust event listener started, now emitting 100 events....");

    // Run the TypeScript event emitter and stream its output
    let mut typescript_process = Command::new("npx")
        .args([
            "hardhat",
            "run",
            "--network",
            "yellowstone",
            "scripts/emitManyEvents.ts",
        ])
        .current_dir("..")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to start TypeScript emitter");

    // Stream TypeScript stdout
    let ts_stdout = typescript_process
        .stdout
        .take()
        .expect("Failed to capture TypeScript stdout");
    let ts_stderr = typescript_process
        .stderr
        .take()
        .expect("Failed to capture TypeScript stderr");

    // Spawn threads to handle TypeScript output streaming
    let stdout_thread = thread::spawn(move || {
        let reader = BufReader::new(ts_stdout);
        for line in reader.lines() {
            if let Ok(line) = line {
                println!("TypeScript: {}", line);
            }
        }
    });

    let stderr_thread = thread::spawn(move || {
        let reader = BufReader::new(ts_stderr);
        for line in reader.lines() {
            if let Ok(line) = line {
                eprintln!("TypeScript error: {}", line);
            }
        }
    });

    // Wait for TypeScript process to complete
    let typescript_status = typescript_process
        .wait()
        .expect("Failed to wait for TypeScript process");

    // Wait for output threads to finish
    stdout_thread.join().expect("Failed to join stdout thread");
    stderr_thread.join().expect("Failed to join stderr thread");

    assert!(typescript_status.success(), "TypeScript emitter failed");
    println!("TypeScript emitter completed, all events have been emitted");

    // Give some time for the events to be processed after script completion
    thread::sleep(Duration::from_secs(5));

    println!("Waiting for events to be processed by Rust listener");
    // Read all available lines from stdout
    let lines = rust_reader.lines();
    for line in lines {
        if let Ok(line) = line {
            if line.contains("New event: SomethingHappenedFilter { id:") {
                // Extract the event ID using string manipulation
                if let Some(id_str) = line.split("id: ").nth(1).and_then(|s| s.split(" }").next()) {
                    if let Ok(id) = id_str.parse::<u64>() {
                        collected_events.insert(id);
                        if collected_events.len() == 100 {
                            break;
                        }
                    }
                }
            }
        }
    }

    // Kill the Rust process
    rust_process.kill().expect("Failed to kill rust process");

    // Print final stats
    println!("\nFinal event collection stats:");
    println!("Total events collected: {}", collected_events.len());
    if collected_events.len() < 100 {
        println!(
            "Missing events: {:?}",
            (0..100)
                .filter(|i| !collected_events.contains(i))
                .collect::<Vec<_>>()
        );
    }

    // Verify we got all events
    assert_eq!(
        collected_events.len(),
        100,
        "Did not receive all 100 events"
    );
    for i in 0..100 {
        assert!(collected_events.contains(&i), "Missing event with id {}", i);
    }
    println!("All events received by Rust listener");
}
