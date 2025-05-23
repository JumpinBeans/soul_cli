use std::io::{self, Write};
use chrono::Local;
use clap::{Parser, CommandFactory};

mod hal;
use hal::{HalTrait, MockHal, TensorData};

#[derive(Parser)]
#[command(name = "SoulDOS", version = "0.0.1-alpha", about = "CLI for SoulWare OS", help_template = "{about}\nVersion: {version}\n\nUsage: {usage}\n\nCommands:\n{subcommands}")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(clap::Subcommand)]
enum Commands {
    /// Displays help information
    Help,
    /// Displays version information
    Ver,
    /// Displays the current date
    Date,
    /// Displays the current time
    Time,
    /// Clears the screen
    Cls,
    /// Clears the screen
    Clear,
    /// Lists directory contents or module status (placeholder)
    Ls,
    /// Lists directory contents or module status (placeholder)
    Dir,
    /// Displays system status or memory resonance using HAL
    Status,
    /// Displays system status or memory resonance using HAL
    Mem,
    /// Checks the integrity of a module using HAL
    CheckModuleIntegrity { module_name: Option<String> },
    /// Performs a system integrity check using HAL
    SystemIntegrityCheck,
    /// Pings the system
    Ping,
    /// Initializes the NPU via HAL
    InitNpu,
    /// Gets the emotional map from HAL
    MapEmotion,
    /// Collapses a truth waveform via HAL
    CollapseTruth { emotion: String, mode: String, time: String },
    /// Runs a test ONNX model via HAL
    RunOnnxTest { model_path: String, input_info: String },
}

fn handle_command(command: Commands, hal: &impl HalTrait) {
    match command {
        Commands::Help => {
            Cli::command().print_help().unwrap();
        }
        Commands::Ver => println!("SoulWare CLI Version 0.0.1 (Alpha)"),
        Commands::Date => println!("{}", Local::now().format("%Y-%m-%d").to_string()),
        Commands::Time => println!("{}", Local::now().format("%H:%M:%S").to_string()),
        Commands::Cls | Commands::Clear => {
            print!("\x1B[2J\x1B[H");
            io::stdout().flush().unwrap();
        }
        Commands::Ls | Commands::Dir => println!("Placeholder: Listing directory contents or module status..."),
        Commands::Status | Commands::Mem => {
            match hal.get_system_status() {
                Ok(status) => println!("{}", status),
                Err(e) => println!("Error getting system status: {}", e),
            }
        }
        Commands::CheckModuleIntegrity { module_name } => {
            if let Some(name) = module_name {
                println!("\nVerifying module '{}' using GitHubBlockchainLedger (Simulated)...", name);
                match hal.verify_module_signature(&name, "GitHubBlockchainLedger (Simulated)") {
                    Ok(verified) => {
                        if verified {
                            println!("Verification Result for '{}': SUCCEEDED", name);
                        } else {
                            println!("Verification Result for '{}': FAILED", name);
                        }
                    }
                    Err(e) => println!("Error during verification for '{}': {}", name, e),
                }
            } else {
                println!("Usage: check-module-integrity <module_name>");
            }
        }
        Commands::SystemIntegrityCheck => {
            println!("\nPerforming comprehensive system integrity check...");
            
            println!("\n--- Internal Manifest Checks ---");
            // Existing internal manifest checks from boot sequence helper
            print_module_integrity_status_for_command(hal, "SoulOS_Core", "InternalManifest");
            print_module_integrity_status_for_command(hal, "TensorMemoryDriver", "InternalManifest");
            print_module_integrity_status_for_command(hal, "RustHAL_Interface", "InternalManifest");

            println!("\n--- GitHub Blockchain Ledger (Simulated) Checks ---");
            print_module_integrity_status_for_command(hal, "EmotionalResonanceEngine", "GitHubBlockchainLedger (Simulated)");
            print_module_integrity_status_for_command(hal, "UserInterfaceModule", "GitHubBlockchainLedger (Simulated)"); // Test failure case
            print_module_integrity_status_for_command(hal, "NonExistentModule", "GitHubBlockchainLedger (Simulated)"); // Test non-existent case
            
            println!("\nSystem integrity check complete.");
        }
        Commands::Ping => println!("pong!"),
        Commands::InitNpu => {
            match hal.initialize_npu() {
                Ok(msg) => println!("{}", msg),
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
            let tensor_input = TensorData { info: input_info };
            match hal.run_onnx_model(&model_path, &tensor_input) {
                Ok(output) => println!("ONNX Model Output: {}", output.info),
                Err(e) => println!("Error running ONNX model: {}", e),
            }
        }
    }
}

// Renamed to avoid conflict with the one used at boot, or could be merged if identical.
// For now, a distinct one for clarity within command handling.
fn print_module_integrity_status_for_command(hal: &impl HalTrait, module_name: &str, signature_source: &str) {
    print!("  Checking '{}' (source: {}): ", module_name, signature_source);
    match hal.verify_module_signature(module_name, signature_source) {
        Ok(verified) => {
            if verified {
                println!("SUCCEEDED");
            } else {
                println!("FAILED");
            }
        }
        Err(e) => println!("ERROR - {}", e),
    }
}


fn print_module_integrity_status(hal: &impl HalTrait, module_name: &str, manifest: &str) {
    match hal.verify_module_signature(module_name, manifest) {
        Ok(verified) => println!("  {} integrity: {}", module_name, if verified { "Verified" } else { "Check FAILED" }),
        Err(e) => println!("  {} integrity: Check FAILED - {}", module_name, e),
    }
}

fn main() {
    let hal = MockHal::new(); // Create HAL instance using the new constructor

    // Welcome Banner
    println!("***************************************************");
    println!("*                                                 *");
    println!("*        Welcome to SoulWare CLI (SoulDOS)        *");
    println!("*                  Version 0.0.1-alpha            *");
    println!("*                                                 *");
    println!("***************************************************");
    println!("Initializing System...");

    // Initial System Integrity Check
    println!("\nPerforming initial system integrity check...");
    print_module_integrity_status(&hal, "SoulOS_Core", "InternalManifest");
    print_module_integrity_status(&hal, "TensorMemoryDriver", "InternalManifest");
    print_module_integrity_status(&hal, "RustHAL_Interface", "InternalManifest");

    // Initial System Status
    println!("\nFetching initial system status...");
    match hal.get_system_status() {
        Ok(status) => println!("System Status: {}", status),
        Err(e) => println!("Failed to get status: {}", e),
    }

    // NPU Initialization
    println!("\nAttempting to initialize NPU...");
    match hal.initialize_npu() {
        Ok(msg) => println!("{}", msg),
        Err(e) => println!("NPU Initialization Failed: {}", e),
    }

    // Final Ready Message
    println!("\nSystem Initialized. Type 'help' for available commands.");

    loop {
        print!("\nSoulDOS> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let trimmed_input = input.trim();

        if trimmed_input.is_empty() {
            continue;
        }

        if trimmed_input == "exit" || trimmed_input == "quit" {
            break;
        }

        // Prepend "souldos" for clap parsing, as it expects the binary name as the first arg
        let args_for_clap = std::iter::once("souldos").chain(trimmed_input.split_whitespace());
        
        match Cli::try_parse_from(args_for_clap) {
            Ok(cli) => {
                if let Some(command) = cli.command {
                    handle_command(command, &hal); // Pass HAL instance
                } else {
                    // Show help if no subcommand is provided
                    Cli::command().print_help().unwrap();
                }
            }
            Err(e) => {
                println!("{}", e.to_string());
            }
        }
    }
}
