use std::fs::write;
use serde::Serialize;
use crate::SymbolInfo;

pub fn generate_html_report(data: &[SymbolInfo], output_path: &str) -> anyhow::Result<()> {
    let json_data = serde_json::to_string(data)?;
    let template = include_str!("../template.html");
    let output = template.replace("{{DATA}}", &json_data);
    write(output_path, output)?;
    Ok(())
}

pub fn export_json(data: &[SymbolInfo], path: &str) -> anyhow::Result<()> {
    let json = serde_json::to_string_pretty(data)?;
    write(path, json)?;
    Ok(())
}

pub fn export_csv(data: &[SymbolInfo], path: &str) -> anyhow::Result<()> {
    let mut wtr = csv::Writer::from_path(path)?;
    for row in data {
        wtr.serialize(row)?;
    }
    wtr.flush()?;
    Ok(())
}
