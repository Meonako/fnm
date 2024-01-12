use std::io::{stdout, Write};

use crate::config::FnmConfig;
use crate::remote_node_index;
use comfy_table::{Cell, Table};
use thiserror::Error;

#[derive(clap::Parser, Debug)]
pub struct LsRemote {
    /// List all versions, otherwise just 60 latest version
    #[clap(short, long, conflicts_with_all = &["lts"])]
    pub all: bool,

    /// List only LTS versions
    #[clap(long, conflicts_with_all = &["all"])]
    pub lts: bool,
}

impl super::command::Command for LsRemote {
    type Error = Error;

    fn apply(self, config: &FnmConfig) -> Result<(), Self::Error> {
        print!("Fetching all versions...\r");
        stdout().flush().unwrap();
        let all_versions = remote_node_index::list(&config.node_dist_mirror)?;

        let mut table = Table::new();
        table
            .load_preset(comfy_table::presets::UTF8_FULL_CONDENSED)
            .set_content_arrangement(comfy_table::ContentArrangement::DynamicFullWidth);

        let version_iter = if !self.all && !self.lts {
            &all_versions[0..=60]
        } else {
            &all_versions
        };

        let mut a_row = Vec::with_capacity(3);
        for version in version_iter {
            if self.lts && version.lts.is_none() {
                continue;
            }

            let formatted = if let Some(lts) = &version.lts {
                format!("{} ({lts})", version.version)
            } else {
                format!("{}", version.version)
            };

            a_row.push(Cell::new(formatted).set_alignment(comfy_table::CellAlignment::Center));

            if a_row.len() == 4 {
                table.add_row(a_row);
                a_row = Vec::with_capacity(3);
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
