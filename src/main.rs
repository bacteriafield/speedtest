mod cli;
mod helpers;
mod json;
mod metrics;
mod network;
mod renderer;
mod server;
mod speedtest;

use clap::Parser;
use cli::Cli;
use crossterm::terminal;

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    if cli.list_servers {
        println!("{:<16}  {:<28}  {}", "ID", "Name", "Location");
        println!("{}", "─".repeat(60));
        for s in server::SERVERS {
            println!("{:<16}  {:<28}  {}", s.id, s.name, s.location);
        }
        return;
    }

    let probe_client = network::build_client();

    if !cli.json {
        eprint!("  Selecting server…");
    }

    let server = match server::select_best(&probe_client, cli.server.as_deref()).await {
        Ok(s) => s,
        Err(e) => {
            eprintln!("\n  Error: {}", e);
            std::process::exit(1);
        }
    };

    if !cli.json {
        eprintln!(
            "\r  Server: {} ({})             ",
            server.name, server.location
        );
    }

    // Hide cursor during the live display.
    if !cli.json {
        let _ = crossterm::execute!(std::io::stdout(), crossterm::cursor::Hide,);
    }

    let result = speedtest::run(&cli, server).await;

    // Restore cursor.
    if !cli.json {
        let _ = crossterm::execute!(std::io::stdout(), crossterm::cursor::Show,);
    }

    if cli.json {
        println!("{}", json::to_json(&result));
    } else {
        json::print_summary(&result);
    }

    if let Some(ref path) = cli.export {
        if let Err(e) = json::export(&result, path) {
            eprintln!("  Export error: {}", e);
        }
    }
}
