#![feature(absolute_path)]
use carroh::{
    cli::Args,
    csv_processor::CsvProcessor,
};
use clap::Parser;
use log::info;
use std::{
    error::Error,
    path,
};

// Take the first argument as the csv location.
// TODO: If no argument, prompt for location
// TODO: Ensure marc column exists.  If it is not there, indicate the error and
// exit.
// TODO: For every line in the CSV, verify that the marc column is equal
// to the previous row's value.  If a row has a different value, print the
// invalid lines and exit.
// TODO: For every line in the CSV, verify that the grant_cycle column is equal
// to the previous row's value.  If a row has a different value, print the
// invalid lines and exit.
// TODO: Determine if the file should use the obj_call_number column (ocn) or
// obj_temporary_id column (oti) field as the per-item identifier column (pit)
// - Check every line in the CSV for the existence of either ocn or oti.
// - If ocn is not available in every line, and oti is not available on every
//   line, print an error that one is required on all lines and exit.
// - If the ocn exists on all lines but not oti, use the ocn as the pit. Print
//   the selected choice.
// - If the oti exists on all lines but not ocn, use the oti as the pit. Print
//   the selected choice.
// - If all lines contain both ocn and oti, use the ocn as the pit. Print the
//   selected choice.
// TODO: Compute the grant cycle descriptor (gcd):
// - Take the obj_grant_cycle field from the first row.
// - Substitute any "/" characters for "-", giving the gcd.
// TODO: Compute the parent directory location (pdl):
// - Prompt the user for the location of the output folder parent (ofp).
// - Calculate pdl as ofp/gcd + "_" + marc
// TODO: Check if the pdl exists.  If it does, prompt the user to continue.  If
// it does not, create it. Compute the raw file directory location (rdl) as
// pdl/marc + "_" + gcd + "_Raw".
// TODO: Check if the rdl exists.  If it does, prompt the user to continue.  If
// it does not, create it.
// TODO: Prompt the user to select the imaging device (imd) from the local
// system devices. Use a hard-coded default.
// TODO: For every line in the CSV:
// - For each semi-colon-separated value in the pit (cvp):
//   - Prompt the user to locate and insert the disc associated with the cvp.
//   - Wait for the user to press enter to continue.
//   - Retain the system's disk label (sdl) from the imd.
//   - Compute the cvp's iso location (cil) as rdl/sdl + ".iso"
//   - Compute the cvp's file location (cfl) as rdl/sdl.
//   - Generate the imd's ISO and write it to cil.
//   - Extract the contents of the cil to the cfl.
//   - Eject the disk.
fn main() -> Result<(), Box<dyn Error>>
{
    let args = Args::parse();
    env_logger::Builder::new()
        .filter_level(args.verbose.log_level_filter())
        .init();


    let abs_csv_path = path::absolute(args.csv_path)?;

    info!("Reading CSV file from \"{abs_csv_path:?}\"");

    let mut cp = CsvProcessor::new(abs_csv_path)?;

    cp.print_all_rows()?;
    cp.assert_equal_column_values(&"marc".to_string()).map_err(|e| format!("{e}"))?;
    cp.assert_equal_column_values(&"obj_grant_cycle".to_string()).map_err(|e| format!("{e}"))?;

    Ok(())
}
