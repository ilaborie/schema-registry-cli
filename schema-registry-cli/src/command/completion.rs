use std::io::stdout;

use clap::CommandFactory;

use crate::{Completion, Result, Settings};

const BIN_NAME: &str = env!("CARGO_CRATE_NAME");

pub(crate) fn generate_completion(completion: Completion) -> Result<()> {
    let Completion { shell, out_dir } = completion;
    let mut cmd = <Settings as CommandFactory>::command();

    if let Some(out_dir) = &out_dir {
        clap_complete::generate_to(shell, &mut cmd, BIN_NAME, out_dir)?;
    } else {
        clap_complete::generate(shell, &mut cmd, BIN_NAME, &mut stdout());
    }
    Ok(())
}
