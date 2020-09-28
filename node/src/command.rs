// This file is part of Substrate.

// Copyright (C) 2017-2020 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::chain_spec;
use crate::cli::Cli;
use crate::service;
use log::info;
use sc_cli::{SubstrateCli, RuntimeVersion, Role, ChainSpec};
use sc_service::PartialComponents;
use crate::service::new_partial;

impl SubstrateCli for Cli {
	fn impl_name() -> String {
		"Uni-arts Node".into()
	}

	fn impl_version() -> String {
		env!("SUBSTRATE_CLI_IMPL_VERSION").into()
	}

	fn description() -> String {
		env!("CARGO_PKG_DESCRIPTION").into()
	}

	fn author() -> String {
		env!("CARGO_PKG_AUTHORS").into()
	}

	fn support_url() -> String {
		"https://github.com/uni-arts-chain/uni-arts-network/issues".into()
	}

	fn copyright_start_year() -> i32 {
		2020
	}

	fn load_spec(&self, id: &str) -> Result<Box<dyn sc_service::ChainSpec>, String> {
		Ok(match id {
			"dev" => Box::new(chain_spec::development_config()?),
			"" | "local" => Box::new(chain_spec::local_testnet_config()?),
			path => Box::new(chain_spec::ChainSpec::from_json_file(
				std::path::PathBuf::from(path),
			)?),
		})
	}

	fn native_runtime_version(_: &Box<dyn ChainSpec>) -> &'static RuntimeVersion {
		&uart_runtime::VERSION
	}
}

/// Parse and run command line arguments
pub fn run() -> sc_cli::Result<()> {
	let cli = Cli::from_args();

	match cli.subcommand {

		Some(ref subcommand) => {
			let runner = cli.create_runner(subcommand)?;
			runner.run_subcommand(subcommand, |config| {
				let PartialComponents { client, backend, task_manager, import_queue, .. }
					= new_partial(&config)?;
				Ok((client, backend, import_queue, task_manager))
			})
		}

		None => {
			let runner = cli.create_runner(&cli.run)?;

			info!("  _    _       _                    _          _____ _           _       ");
			info!(" | |  | |     (_)        /\\        | |        / ____| |         (_)      ");
			info!(" | |  | |_ __  _ ______ /  \\   _ __| |_ ___  | |    | |__   __ _ _ _ __  ");
			info!(" | |  | | '_ \\| |______/ /\\ \\ | '__| __/ __| | |    | '_ \\ / _` | | '_ \\ ");
			info!(" | |__| | | | | |     / ____ \\| |  | |_\\__ \\ | |____| | | | (_| | | | | |");
			info!(" \\____/|_| |_|_|    /_/    \\_\\_|   \\__|___/  \\_____|_| |_|\\__,_|_|_| |_|");
			info!("                                                                         ");
			info!("                                                                         ");

			runner.run_node_until_exit(|config| match config.role {
				Role::Light => service::new_light(config),
				_ => service::new_full(config),
			})
		}
	}
}
