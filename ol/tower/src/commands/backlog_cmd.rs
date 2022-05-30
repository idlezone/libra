//! `backlog`

use abscissa_core::{Command, Options, Runnable};
use diem_logger::{Level, Logger};
use ol_types::config::TxType;
use std::process::exit;
use txs::tx_params::TxParams;

use crate::{backlog, entrypoint};
use crate::{entrypoint::EntryPointTxsCmd, prelude::*};

use std::{fs::File};
// use anyhow::{anyhow, Error, Result, bail};
use ol_types::block::VDFProof;
use std::io::BufReader;
use txs::submit_tx::{eval_tx_status};
use crate::commit_proof::commit_proof_tx;

/// `backlog` subcommand
#[derive(Command, Default, Debug, Options)]
pub struct BacklogCmd {
    /// Option for --submit, sends backlogged transactions.
    #[options(
    short = "s",
    help = "Submit backlogged transactions"
    )]
    submit: bool,

    /// option for submit zero
    #[options(
        short = "z",
        help = "Submit zero"
        )]
    submit_zero: bool,

    /// Submits the specified proof from the proof directory
    #[options(
    short = "n",
    help = "Submit specific proof, given as numerical argument, e.g. 1337 to submit proof no. 1337"
    )]
    submit_specific: Option<u64>,

    /// show backlog
    #[options(short = "l", help = "List backlog")]
    list_backlog: bool,
}

impl Runnable for BacklogCmd {
    /// Start the application.
    fn run(&self) {
        let EntryPointTxsCmd {
            url,
            waypoint,
            swarm_path,
            swarm_persona,
            is_operator,
            use_first_url,
            ..
        } = entrypoint::get_args();

        // Setup logger
        let mut logger = Logger::new();
        logger
            .channel_size(1024)
            .is_async(true)
            .level(Level::Info)
            .read_env();
        logger.build();

        // config reading respects swarm setup
        // so also cfg.get_waypoint will return correct data
        let cfg = app_config().clone();

        let waypoint = if waypoint.is_none() {
            match cfg.get_waypoint(None) {
                Ok(w) => Some(w),
                Err(e) => {
                    status_err!("Cannot start without waypoint. Message: {:?}", e);
                    exit(-1);
                }
            }
        } else {
            waypoint
        };

        let tx_params = TxParams::new(
            cfg.clone(),
            url,
            waypoint,
            swarm_path,
            swarm_persona,
            TxType::Miner,
            is_operator,
            use_first_url,
            None,
        ).expect("could not get tx parameters");

        let mut processed_commands = 0u8;

        if let Some(specific_proof) = self.submit_specific {
            match backlog::submit_proof_by_number(&cfg, &tx_params, specific_proof) {
                Ok(()) => {},
                Err(e) => {
                    println!("WARN: Unable to submit proof: {:?}", e);
                }
            }

            processed_commands = processed_commands + 1;
        }

        if self.submit {
            match backlog::process_backlog(&cfg, &tx_params) {
                Ok(()) => {},
                Err(e) => {
                    println!("WARN: Unable to submit backlog: {:?}", e);
                }
            }

            processed_commands = processed_commands + 1;
        }

        if self.submit_zero {
            // let path =PathBuf::from(format!("proof_0.json");
            // let file = File::open("proof_0.json").map_err(|e| Error::from(e))?;
            let mut config_path = entrypoint::get_node_home();
            println!("my path: {:?}", config_path);
            let args = entrypoint::get_args();
            let xff_folder = args.xff_folder.unwrap();
            // let xff_folder = "vdf_proofs_D3101358BE81A590FF39B5127F637DC4/";
            config_path.push(xff_folder);
            config_path.push("proof_0.json");
            println!("current path: {:?}", config_path);
            let file = File::open(config_path).unwrap();
            let reader = BufReader::new(file);
            // let block: VDFProof =
            //     serde_json::from_reader(reader).map_err(|e| Error::from(e))?;
            let block: VDFProof = serde_json::from_reader(reader).unwrap();
            let view = commit_proof_tx(&tx_params, block).unwrap();
            match eval_tx_status(view) {
                Ok(_) => {}
                Err(e) => {
                    warn!(
                                "WARN: could not fetch proof_0 TX status, aborting. Message: {:?} ",
                                e
                            );
                    // return Err(e)
                }
            };
            processed_commands = processed_commands + 1;
        }

        if processed_commands == 0 || self.list_backlog {
            // Check for, and submit backlog proofs.
            match backlog::show_backlog(&cfg, &tx_params) {
                Ok(()) => {},
                Err(e) => {
                    println!("WARN: Unable to list backlog: {:?}", e);
                }
            }
        }
    }
}
