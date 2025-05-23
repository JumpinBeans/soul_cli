use std::io::{self, Write};
use chrono::Local;
use clap::{Parser, Subcommand, CommandFactory};
use human_panic::setup_panic;

mod hal;
use hal::{HalTrait, MockHal, TensorData};

#[derive(Parser)]
#[command(name = "SoulDOS", version = "0.0.1-alpha", about = "CLI for SoulWare OS", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    // Help variant removed
    /// Show version information
    Ver,
    /// Display the current date
    Date,
    /// Display the current time
    Time,
    /// Clear the terminal screen
    Cls,
    /// Clear the terminal screen
    Clear,
    /// List directory contents or module status (placeholder)
    Ls,
    /// List directory contents or module status (placeholder)
    Dir,
    /// Display system status or memory resonance
    Status,
    /// Display system status or memory resonance
    Mem,
    /// Check integrity of a specific module
    CheckModuleIntegrity { module_name: Option<String> },
    /// Perform system integrity check
    SystemIntegrityCheck,
    /// Send a ping to the system
    Ping,
    /// Initialize the NPU (Neural Processing Unit)
    InitNpu,
    /// Display the current emotional map from the tensor field
    MapEmotion,
    /// Collapse a truth waveform in the tensor memory
    CollapseTruth {
        emotion: String,
        mode: String,
        time: String,
    },
    /// Test an ONNX model run via the HAL
    RunOnnxTest {
        model_path: String,
        input_info: String,
    },
}

fn print_module_integrity_status(hal: &impl HalTrait, module_name: &str, manifest: &str) {
    match hal.verify_module_signature(module_name, manifest) {
        Ok(true) => println!("  Core module '{}' integrity: VERIFIED internally.", module_name),
        Ok(false) => println!("  Core module '{}' integrity: VERIFICATION FAILED internally.", module_name),
        Err(e) => println!("  Error checking core module '{}' integrity: {}", module_name, e),
    }
}

fn print_module_integrity_status_for_command(hal: &impl HalTrait, module_name: &str, manifest: &str) {
    print!("  Checking '{}' (source: {})... ", module_name, manifest);
    io::stdout().flush().unwrap();
    match hal.verify_module_signature(module_name, manifest) {
        Ok(true) => println!("VERIFIED"),
        Ok(false) => println!("FAILED"),
        Err(e) => println!("ERROR ({})", e),
    }
}


fn handle_command(command_enum: Commands, hal: &impl HalTrait) {
    match command_enum {
        // Case for Commands::Help removed
        Commands::Ver => {
            println!("SoulWare CLI Version 0.0.1-alpha (Handled by 'ver' command)");
        }
        Commands::Date => println!("{}", Local::now().format("%Y-%m-%d").to_string()),
        Commands::Time => println!("{}", Local::now().format("%H:%M:%S").to_string()),
        Commands::Cls | Commands::Clear => {
            print!("\x1B[2J\x1B[H");
            io::stdout().flush().unwrap();
        }
        Commands::Ls | Commands::Dir => {
            println!("Placeholder: Listing directory contents or module status...");
        }
        Commands::Status | Commands::Mem => {
            println!("\nFetching system status...");
            match hal.get_system_status() {
                Ok(status) => println!("System Status: {}", status),
                Err(e) => println!("Error fetching system status: {}", e),
            }
        }
        Commands::CheckModuleIntegrity { module_name } => {
            if let Some(name) = module_name {
                println!("\nChecking integrity of module: '{}'...", name);
                match hal.verify_module_signature(&name, "GitHubBlockchainLedger (Simulated)") {
                    Ok(true) => println!("Module '{}' integrity: VERIFIED", name),
                    Ok(false) => println!("Module '{}' integrity: VERIFICATION FAILED", name),
                    Err(e) => println!("Error checking module '{}' integrity: {}", name, e),
                }
            } else {
                println!("Usage: check-module-integrity <module_name>");
            }
        }
        Commands::SystemIntegrityCheck => {
            println!("\nPerforming System Integrity Check...");

            println!("\nInternal Manifest Checks:");
            print_module_integrity_status_for_command(hal, "SoulOS_Core", "InternalManifest");
            print_module_integrity_status_for_command(hal, "TensorMemoryDriver", "InternalManifest");
            print_module_integrity_status_for_command(hal, "RustHAL_Interface", "InternalManifest");
            
            println!("\nGitHub Blockchain Ledger (Simulated) Checks:");
            print_module_integrity_status_for_command(hal, "EmotionalResonanceEngine", "GitHubBlockchainLedger (Simulated)");
            print_module_integrity_status_for_command(hal, "UserInterfaceModule", "GitHubBlockchainLedger (Simulated)");
            print_module_integrity_status_for_command(hal, "NonExistentModule", "GitHubBlockchainLedger (Simulated)");
        }
        Commands::Ping => println!("pong!"),
        Commands::InitNpu => {
            println!("\nInitializing NPU...");
            match hal.initialize_npu() {
                Ok(status) => println!("NPU Status: {}", status),
                Err(e) => println!("Error initializing NPU: {}", e),
            }
        }
        Commands::MapEmotion => {
            println!("\nFetching emotional map from Tensor Field...");
            match hal.get_emotional_map() {
                Ok(map_data) => {
                    println!("Current Emotional Map in Tensor Field:");
                    if map_data.is_empty() {
                        println!("  Emotional map is currently clear.");
                    } else {
                        for entry in map_data {
                            println!("  - {}", entry);
                        }
                    }
                }
                Err(e) => println!("Error fetching emotional map: {}", e),
            }
        }
        Commands::CollapseTruth { emotion, mode, time } => {
            println!("\nAttempting to collapse truth waveform for emotion '{}', mode '{}', time '{}'...", emotion, mode, time);
            match hal.collapse_truth_waveform(&emotion, &mode, &time) {
                Ok(result_node) => println!("Tensor Waveform Collapse Result: '{}'", result_node),
                Err(e) => println!("Error during truth collapse: {}", e),
            }
        }
        Commands::RunOnnxTest { model_path, input_info } => {
            println!("\nAttempting to run ONNX model test for model: '{}' with input: '{}'...", model_path, input_info);
            let tensor_input = TensorData { info: input_info };
            match hal.run_onnx_model(&model_path, &tensor_input) {
                Ok(output_data) => println!("ONNX Model Test Result: '{}'", output_data.info),
                Err(e) => println!("Error running ONNX model test: {}", e),
            }
        }
    }
}

fn main() {
    setup_panic!(); // Initialize human-panic

    let hal = MockHal::new(); // Initialize HAL

    // --- First Boot Sequence ---
    println!("***************************************************");
    println!("*                                                 *");
    println!("*        Welcome to SoulWare CLI (SoulDOS)        *");
    println!("*                  Version 0.0.1-alpha            *");
    println!("*                                                 *");
    println!("***************************************************");
    println!("Initializing System...");

    println!("\nPerforming initial system integrity check...");
    print_module_integrity_status(&hal, "SoulOS_Core", "InternalManifest");
    print_module_integrity_status(&hal, "TensorMemoryDriver", "InternalManifest");
    print_module_integrity_status(&hal, "RustHAL_Interface", "InternalManifest");


    println!("\nFetching initial system status...");
    match hal.get_system_status() {
        Ok(status) => println!("System Status: {}", status),
        Err(e) => println!("Failed to get status: {}", e),
    }

    println!("\nAttempting to initialize NPU...");
    match hal.initialize_npu() {
        Ok(status) => println!("NPU Status: {}", status),
        Err(e) => println!("Failed to initialize NPU: {}", e),
    }

    println!("\nSystem Initialized. Type 'help' for available commands.");
    // --- End of First Boot Sequence ---

    loop {
        print!("\nSoulDOS> ");
        io::stdout().flush().unwrap();

        let mut line = String::new();
        if io::stdin().read_line(&mut line).is_err() {
            eprintln!("Error reading input.");
            break;
        }

        let trimmed_line = line.trim();

        if trimmed_line.is_empty() {
            continue;
        }

        if trimmed_line.eq_ignore_ascii_case("exit") || trimmed_line.eq_ignore_ascii_case("quit") {
            println!("Exiting SoulWare CLI...");
            break;
        }

        let args = std::iter::once("souldos".to_string())
                              .chain(trimmed_line.split_whitespace().map(String::from));

        match Cli::try_parse_from(args) {
            Ok(cli_matches) => {
                if let Some(command_enum) = cli_matches.command {
                    handle_command(command_enum, &hal);
                } else {
                    // No subcommand was given, so clap will print help by default if
                    // neither `subcommand_required(true)` nor `arg_required_else_help(true)` is set.
                    // Or, we can explicitly print help.
                    if let Err(e) = Cli::command().print_help() {
                         eprintln!("Error displaying help: {}", e);
                    }
                }
            }
            Err(e) => {
                if let Err(print_err) = e.print() {
                    eprintln!("Error displaying command parse error: {}", print_err);
                    eprintln!("Original error was: {}", e.to_string()); 
                }
            }
        }
    }
}