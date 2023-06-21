use std::env;
use std::fs;
use std::process::{Command, exit};
use std::time::{Duration, Instant};
use wait_timeout::ChildExt;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut command_args = Vec::new();
    let mut output_file = None;
    let mut timeout = None;
    let mut repeat = 1;
    let mut verbose = false;

    let mut i = 1;
    while i < args.len() {
        let arg = &args[i];
        if arg.starts_with('-') && arg.len() > 1 {
            if arg.starts_with("--") {
                match arg.as_str() {
                    "--help" => {
                        println!("Usage: {} [OPTIONS] COMMAND", args[0]);
                        println!("\nOptions:");
                        println!("  -h, --help       Display this help message");
                        println!("  -v, --verbose    Run in verbose mode");
                        println!("  -o, --output     Specify an output file");
                        println!("  -t, --timeout    Specify a timeout in seconds");
                        println!("  -r, --repeat     Specify the number of times to run the command");
                        exit(0);
                    }
                    "--verbose" => verbose = true,
                    "--output" => {
                        if i + 1 < args.len() {
                            output_file = Some(args[i + 1].to_string());
                            i += 1;
                        } else {
                            eprintln!("Error: Missing argument for option {}", arg);
                            exit(1);
                        }
                    }
                    "--timeout" => {
                        if i + 1 < args.len() {
                            timeout = Some(args[i + 1].parse::<u64>().unwrap());
                            i += 1;
                        } else {
                            eprintln!("Error: Missing argument for option {}", arg);
                            exit(1);
                        }
                    }
                    "--repeat" => {
                        if i + 1 < args.len() {
                            repeat = args[i + 1].parse::<usize>().unwrap();
                            i += 1;
                        } else {
                            eprintln!("Error: Missing argument for option {}", arg);
                            exit(1);
                        }
                    }
                    _ => command_args.push(arg.to_string()),
                }
            } else {
                for c in arg.chars().skip(1) {
                    match c {
                        'h' => {
                            println!("Usage: {} [OPTIONS] COMMAND", args[0]);
                            println!("\nOptions:");
                            println!("  -h, --help       Display this help message");
                            println!("  -v, --verbose    Run in verbose mode");
                            println!("  -o, --output     Specify an output file");
                            println!("  -t, --timeout    Specify a timeout in seconds");
                            println!("  -r, --repeat     Specify the number of times to run the command");
                            exit(0);
                        }
                        'v' => verbose = true,
                        'o' => {
                            if i + 1 < args.len() {
                                output_file = Some(args[i + 1].to_string());
                                i += 1;
                                break;
                            } else {
                                eprintln!("Error: Missing argument for option {}", arg);
                                exit(1);
                            }
                        }
                        't' => {
                            if i + 1 < args.len() {
                                timeout = Some(args[i + 1].parse::<u64>().unwrap());
                                i += 1;
                                break;
                            } else {
                                eprintln!("Error: Missing argument for option {}", arg);
                                exit(1);
                            }
                        }
                        'r' => {
                            if i + 1 < args.len() {
                                repeat = args[i + 1].parse::<usize>().unwrap();
                                i += 1;
                                break;
                            } else {
                                eprintln!("Error: Missing argument for option {}", arg);
                                exit(1);
                            }
                        }
                        _ => (),
                    }
                }
            }
        } else {
            command_args.extend_from_slice(&args[i..]);
            break;
        }

        i += 1;
    }

    if command_args.is_empty() {
        eprintln!("Please provide a command to run");
        exit(1);
    }

    let mut total_duration = Duration::new(0, 0);
    for _ in 0..repeat {
        let start = Instant::now();
        #[cfg(windows)] {
            let mut child = Command::new("cmd")
                .arg("/C")
                .args(&command_args)
                .spawn()
                .expect("Failed to execute command");

            if let Some(t) = timeout {
                if let None = child.wait_timeout(Duration::from_secs(t)).unwrap() {
                    child.kill().unwrap();
                    eprintln!("Error: Command timed out after {} seconds", t);
                    exit(1);
                }                
            } else {
                child.wait().expect("Failed to wait on child");
            }
        }
        #[cfg(not(windows))] {
            eprintln!("This program is made for Windows");
            exit(1);
        }

        total_duration += start.elapsed();
    }

    if verbose {
        println!(
            "\nTook {}ms (average over {} runs)",
            total_duration.as_millis() / repeat as u128,
            repeat
        );
    }

    if let Some(file) = output_file {
        fs::write(file, format!("{}", total_duration.as_millis())).expect("Failed to write output file");
    }

    exit(0);
}
