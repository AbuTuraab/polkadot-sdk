

// Usage: frame-omni-bencher v1 benchmark pallet [OPTIONS]

// Options:
// ...
//       --chain <CHAIN_SPEC>                               Specify the chain specification
//       --runtime <RUNTIME>                                Optional runtime blob to use instead of the one from the genesis config
//       --genesis-builder <GENESIS_BUILDER>                How to construct the genesis state [possible values: none, runtime, spec-runtime, spec-genesis]
//       --genesis-builder-preset <GENESIS_BUILDER_PRESET>  The preset that we expect to find in the GenesisBuilder runtime API [default: development]
// ...

use clap::{error::ErrorKind, Subcommand, Args, CommandFactory, Parser};
use codec::{Decode, Encode};
use sc_chain_spec::json_patch::merge;
use sc_cli::{CliConfiguration,SharedParams};
use log::info;
use polkadot_parachain_primitives::primitives::Id as ParaId;
use sc_block_builder::BlockBuilderApi;
use sc_chain_spec::{ChainSpec, ChainSpecExtension, GenesisBlockBuilder, GenericChainSpec, 
json_patch, set_code_substitute_in_json_chain_spec, update_code_in_json_chain_spec, ChainType,
GenesisConfigBuilderRuntimeCaller,
};

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{
	borrow::Cow,
	fs,
	path::{Path, PathBuf},
};


#[derive(Debug, Parser, Clone)]
pub struct GenesisPatchCmd {

    #[arg(
        long,
        value_name = "PATH",
        help = "Path to a JSON file containing genesis state modifications",
        default_value = "./genesis_patch.json",
    )]
    pub genesis_patch: Option<PathBuf>,
}

pub fn read_genesis_patch(    
     chain_spec: &mut GenericChainSpec<()>,
     cmd: GenesisPatchCmd
) -> Result<String, String> {
   
   //let mut chain_spec = GenericChainSpec::<E, EHF>::from_json_bytes

    // Read the genesis patch file here
   if let Some(patch_path) = cmd.genesis_patch {
        let patch = fs::read(patch_path.as_path())
            .map_err(|e| format!("Failed to read {patch_path:?}: {e}"))?;
        let patch_str = String::from_utf8(patch)
            .map_err(|e| format!("Failed to convert patch to string: {e}"))?;
           Ok(patch_str)
    } else {
        Err("No genesis patch file provided".to_string())
    }
     // apply_genesis_patch(&mut chain_spec, &patch)?;
}

// pub fn apply_genesis_patch(
//     chain_spec: &mut GenericChainSpec<()>,
//     patch: &str,
//  ) -> Result<(), sc_cli::Error> {
//     let patch_value: Value = from_str(patch)
//     .map_err(|e| sc_cli::Error::Input(format!("Invalid JSON patch: {} ", e)))?;

//     let mut genesis_config = chain_spec.as_json()?;
//     merge(&mut genesis_config, &patch_value);
    
//     *chain_spec = GenericChainSpec::from_json_bytes(serde_json::to_vec(&genesis_config)?)
//     .map_err(|e| sc_cli::Error::Input(e.to_string()))?;

//     Ok(())
//  }

