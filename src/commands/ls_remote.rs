use std::io::{stdout, Write};

use crate::config::FnmConfig;
use crate::remote_node_index;
use thiserror::Error;
use comfy_table::{Table, Cell};

#[derive(clap::Parser, Debug)]
pub struct LsRemote {}

impl super::command::Command for LsRemote {
    type Error = Error;

    fn apply(self, config: &FnmConfig) -> Result<(), Self::Error> {
        let mut stdout = stdout();
        print!("Fetching all versions...\r");
        stdout.flush().unwrap();
        let all_versions = remote_node_index::list(&config.node_dist_mirror)?;

        let mut table = Table::new();
        table.load_preset(comfy_table::presets::UTF8_FULL_CONDENSED).set_content_arrangement(comfy_table::ContentArrangement::DynamicFullWidth);
        let mut a_row = Vec::new();
        for version in all_versions.into_iter().rev().take(60) {
            let formatted = if let Some(lts) = &version.lts {
                format!("{} ({lts})", version.version)
            } else {
                format!("{}", version.version)
            };
            
            a_row.push(Cell::new(formatted).set_alignment(comfy_table::CellAlignment::Center));

            if a_row.len() == 3 {
                table.add_row(a_row);
                a_row = Vec::new();
            }
        }
        println!("{table}");

        Ok(())
    }
}

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    HttpError {
        #[from]
        source: crate::http::Error,
    },
}
