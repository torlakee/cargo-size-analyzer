mod report;

use clap::{Parser, Subcommand};
use object::{Object, ObjectSymbol};
use rustc_demangle::demangle;
use std::collections::HashMap;
use report::{generate_html_report, export_json, export_csv};
use serde::Serialize;
use tabled::{Table, Tabled};

#[derive(Parser, Debug)]
#[command(name = "cargo-size-analyzer", version, about = "Analyze Rust binary size and generate reports.")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    Analyze {
        path: String,

        #[arg(long)]
        html: Option<String>,

        #[arg(long)]
        json: Option<String>,

        #[arg(long)]
        csv: Option<String>,
    },
}

#[derive(Tabled, Serialize)]
pub struct SymbolInfo {
    #[tabled(rename = "Crate")]
    pub crate_name: String,
    #[tabled(rename = "Size (bytes)")]
    pub size: u64,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Command::Analyze { path, html, json, csv } => {
            let file_data = std::fs::read(&path)?;
            let binary = object::File::parse(&*file_data)?;

            let mut crate_sizes: HashMap<String, u64> = HashMap::new();

            for symbol in binary.symbols() {
                if symbol.is_definition() && symbol.size() > 0 {
                    let name = symbol.name().unwrap_or("<unknown>");
                    let demangled = demangle(name).to_string();
                    let crate_name = demangled.split("::").next().unwrap_or("<unknown>").to_string();
                    *crate_sizes.entry(crate_name).or_default() += symbol.size();
                }
            }

            let mut rows: Vec<SymbolInfo> = crate_sizes
                .into_iter()
                .map(|(crate_name, size)| SymbolInfo { crate_name, size })
                .collect();

            rows.sort_by(|a, b| b.size.cmp(&a.size));
            println!("{}", Table::new(&rows));

            if let Some(path) = html {
                generate_html_report(&rows, &path)?;
                println!("HTML report: {}", path);
            }

            if let Some(path) = json {
                export_json(&rows, &path)?;
                println!("JSON report: {}", path);
            }

            if let Some(path) = csv {
                export_csv(&rows, &path)?;
                println!("CSV report: {}", path);
            }
        }
    }

    Ok(())
}
