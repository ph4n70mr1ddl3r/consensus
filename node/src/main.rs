//! Poker Consensus Engine Node
//!
//! A Substrate-based blockchain node implementing BABE/GRANDPA consensus
//! with BLS threshold signatures for trustless poker game coordination.

#![warn(unused_crate_dependencies)]

mod chain_spec;
#[cfg(feature = "cli")]
mod cli;
mod rpc;
mod service;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

use crate::service::{new_partial, ExecutorDispatch};
use sc_cli::{ChainSpec, RuntimeVersion, SubstrateCli};
use sc_service::PartialComponents;
use std::sync::Arc;

#[cfg(feature = "cli")]
use {
    crate::cli::{Cli, Subcommand},
    sc_cli::run_cmd,
};

const IMPL_NAME: &str = "Poker Consensus Engine";
const IMPL_VERSION: &str = "1.0.0";
const TRANSACTION_VERSION: u32 = 1;
const BLOCKS_VERSION: u32 = 1;

fn main() {
    #[cfg(feature = "cli")]
    {
        let cli = Cli::new();

        match &cli.run.subcommand {
            Some(Subcommand::BuildSpec(cmd)) => run_cmd(cmd.build_spec(&*cli.chain_spec)),
            Some(Subcommand::CheckBlock(cmd)) => {
                run_cmd(cmd.check_block(&*cli.chain_spec, &cli.database))
            }
            Some(Subcommand::ExportBlocks(cmd)) => {
                run_cmd(cmd.export_blocks(&*cli.chain_spec, &cli.database))
            }
            Some(Subcommand::ExportState(cmd)) => {
                run_cmd(cmd.export_state(&*cli.chain_spec, &cli.database))
            }
            Some(Subcommand::ImportBlocks(cmd)) => {
                run_cmd(cmd.import_blocks(&*cli.chain_spec, &cli.database))
            }
            Some(Subcommand::PurgeChain(cmd)) => run_cmd(cmd.purge_chain(&cli.database)),
            Some(Subcommand::Revert(cmd)) => run_cmd(cmd.revert(&*cli.chain_spec, &cli.database)),
            Some(Subcommand::Benchmark(cmd)) => {
                if cfg!(feature = "runtime-benchmarks") {
                    run_cmd(cmd.benchmark_overall(
                        &*cli.chain_spec,
                        &cli.database,
                        Arc::new(ExecutorDispatch::new()),
                    ))
                } else {
                    log::error!(
                        "Runtime benchmarking wasn't enabled when building the node. \
                        You can enable it with `--features runtime-benchmarks`."
                    );
                    Ok(())
                }
            }
            None => run_cmd(cli.run(&*cli.chain_spec)),
        }
        .map_err(Into::into)
        .map_err(sc_cli::Error::WithOrigin)
        .and_then(|()| Ok(()))
        .exit()
    }

    #[cfg(not(feature = "cli"))]
    {
        panic!("CLI feature not enabled. Rebuild with --features cli");
    }
}

#[cfg(feature = "cli")]
impl SubstrateCli for Cli {
    fn impl_name() -> String {
        IMPL_NAME.to_string()
    }

    fn impl_version() -> String {
        IMPL_VERSION.to_string()
    }

    fn description() -> String {
        env!("CARGO_PKG_DESCRIPTION").to_string()
    }

    fn author() -> String {
        env!("CARGO_PKG_AUTHORS").to_string()
    }

    fn support_url() -> String {
        "https://github.com/poker-consensus/engine/issues".to_string()
    }

    fn copyright_start_year() -> i32 {
        2025
    }

    fn load_spec(&self, id: &str) -> Result<Box<dyn sc_service::ChainSpec>, String> {
        Ok(Box::new(
            match id {
                "dev" => chain_spec::development_config(),
                "local" => chain_spec::local_testnet_config(),
                "" | "poker" => chain_spec::poker_mainnet_config(),
                path => Box::new(chain_spec::ChainSpec::from_json_file(
                    std::path::PathBuf::from(path),
                )?),
            },
        ))
    }

    fn native_runtime_version(spec: &Box<dyn ChainSpec>) -> &'static RuntimeVersion {
        if spec.chain_type == ChainSpec::LocalTestnet {
            &runtime::VERSION
        } else {
            &runtime::VERSION
        }
    }
}
