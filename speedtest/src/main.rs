#![allow(unused)]
#![allow(dead_code)]
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

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    if cli.list_servers {
        let probe_client = network::build_client();
        let user_loc = server::locate_user(&probe_client).await;

        println!("{:<22}  {:<30}  {:<22}  {:>8}", "ID", "Name", "Location", "Dist (km)");
        println!("{}", "─".repeat(88));

        let mut rows: Vec<_> = server::SERVERS
            .iter()
            .map(|s| {
                let dist =
                    user_loc.as_ref().map(|u| server::haversine_km(u.lat, u.lon, s.lat, s.lon));
                (s, dist)
            })
            .collect();

        // Sort by distance if we have location, else keep original order.
        rows.sort_by(|a, b| match (a.1, b.1) {
            (Some(da), Some(db)) => da.partial_cmp(&db).unwrap(),
            _ => std::cmp::Ordering::Equal,
        });

        for (s, dist) in &rows {
            let dist_str = dist.map(|d| format!("{:>8.0}", d)).unwrap_or_else(|| "       ?".into());
            println!("{:<22}  {:<30}  {:<22}  {}", s.id, s.name, s.location, dist_str);
        }

        if let Some(u) = &user_loc {
            println!();
            println!("  Your location: {}, {} ({:.4}, {:.4})", u.city, u.country, u.lat, u.lon);
        }
        return;
    }

    let probe_client = network::build_client();

    if !cli.json {
        eprint!("  Locating you and selecting nearest server…");
    }

    let (server, user_loc) = match server::select_best(&probe_client, cli.server.as_deref()).await {
        Ok(s) => s,
        Err(e) => {
            eprintln!("\n  Error: {}", e);
            std::process::exit(1);
        },
    };

    if !cli.json {
        eprintln!(
            "\r  Your location : {}, {} ({:.4}, {:.4})         ",
            user_loc.city, user_loc.country, user_loc.lat, user_loc.lon
        );
        eprintln!("  Selected server: {} — {}                      ", server.name, server.location);
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
