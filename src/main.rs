#![feature(absolute_path)]
use carroh::{
    agent::Agent,
    cli::Cli,
    csv_processor::common::{
        header_searcher::HeaderSearcher,
        path_reader::PathReader,
    },
};
use clap::Parser;
use inquire::{
    Confirm,
    Select,
};
use log::info;
use std::{
    error::Error,
    path::PathBuf,
};

fn main() -> Result<(), Box<dyn Error>>
{
    let args = Cli::parse();

    env_logger::Builder::new()
        .filter_level(args.verbose.log_level_filter())
        .init();

    let mut agent = Agent::new(args)?;

    // Take the first argument as the csv location.
    let input_path = agent.get_input_csv_path();
    info!("Input CSV: {input_path:?}",);

    // Take the second argument as the output parent location/output file path
    // (ofp).
    let ofp = agent.get_output_parent();
    info!("Output parent location: {ofp:?}");

    // Ensure marc column exists.  If it is not there, indicate the error and
    // exit.
    // For every line in the CSV, verify that the marc column is equal
    // to the previous row's value.  If a row has a different value, print the
    // invalid lines and exit.
    agent.print_all_rows(&input_path)?;
    agent
        .assert_equal_column_values(&"marc".to_string(), &input_path)
        .map_err(|e| format!("{e}"))?;

    // Ensure obj_grant_cycle column exists.  If it is not there, indicate the
    // error and exit.
    // For every line in the CSV, verify that the grant_cycle column is equal
    // to the previous row's value.  If a row has a different value, print the
    // invalid lines and exit.
    agent
        .assert_equal_column_values(&"obj_grant_cycle".to_string(), &input_path)
        .map_err(|e| format!("{e}"))?;

    // Determine if the file should use the obj_call_number column (ocn) or
    // obj_temporary_id column (oti) field as the per-item identifier column
    // (pit)
    // - Check every line in the CSV for the existence of either ocn or oti.
    // - If ocn is not available in every line, and oti is not available on
    //   every line, print an error that one is required on all lines and exit.
    // - If the ocn exists on all lines but not oti, use the ocn as the pit.
    //   Print the selected choice.
    // - If the oti exists on all lines but not ocn, use the oti as the pit.
    //   Print the selected choice.
    // - If all lines contain both ocn and oti, use the ocn as the pit. Print
    //   the selected choice.
    let ocn_col = "obj_call_number".to_string();
    let oti_col = "obj_temporary_id".to_string();
    let pit_col =
        agent.pick_populated_column(&ocn_col, &oti_col, &input_path)?;
    let pit_col_i = input_path.find_single_header_index(pit_col)?;

    // Compute the grant cycle descriptor (gcd):
    // - Take the obj_grant_cycle field from the first row.
    // - Substitute any "/" characters for "-", giving the gcd.
    let ogc = agent.first_value(&"obj_grant_cycle".to_string(), &input_path)?;
    let marc = agent.first_value(&"marc".to_string(), &input_path)?;
    let gcd = ogc.replace("/", "-");

    // Compute the parent directory location (pdl) as ofp/gcd + "_" + marc
    let mut pdl = ofp.clone();
    pdl.push(format!("{gcd}_{marc}"));

    // Handle a potentially existing pdl.
    agent.create_dir_or_prompt_if_exists(&pdl)?;

    // Compute the raw file directory location (rdl) as
    // pdl/marc + "_" + gcd + "_Raw".
    let mut rdl = pdl.clone();
    rdl.push(format!("{marc}_{gcd}_Raw"));

    // Handle a potentially existing rdl.
    agent.create_dir_or_prompt_if_exists(&rdl)?;

    // Prompt the user to select the imaging device (imd) from the local
    // system devices. Use third argument as default.
    let dev = agent.select_rom_device()?;
    info!("Using device '{dev}' for imaging.");

    // For every line in the CSV:
    for row in input_path.csv()?.records() {
        // For each semi-colon-separated value in the pit (cvp):
        let pit_value = row?[pit_col_i].to_string();

        info!("All row identifiers: {pit_value}");

        let all_cvps = pit_value.split(";");

        for cvp in all_cvps {
            info!("Working on item identifier: {cvp}");
            // Prompt the user to locate and insert the disc associated with the
            // cvp.
            loop {
                println!("Please insert disk associated with {cvp}.");

                // Wait for the user to press enter to continue.
                if Confirm::new(&format!(
                    "Is the disk associated with {cvp} inserted into {dev}? \
                     (Yes/No)"
                ))
                .prompt()?
                {
                    break;
                }

                agent.eject_tray()?;
            }

            // Retain the system's disk label (sdl) from the imd.
            let sdl = agent.get_rom_device_label(&dev)?;

            // Compute the cvp's iso location (cil) as rdl/cvp_sdl + ".iso"
            let mut cil = rdl.clone();
            cil.push(format!("{cvp}_{sdl}.iso"));

            if cil.exists() {
                let cil_s = cil
                    .to_str()
                    .ok_or(format!("ISO path could not be generated."))?;

                let skip_option =
                    format!("Skip {cvp} and continue to the next identifier.");

                if Select::new(
                    &format!(
                        "The iso write location, {cil_s} already exists, so \
                         importing {cvp} cannot continue.  Would you like to \
                         skip importing {cvp} and move on to the remaining \
                         records?",
                    ),
                    vec!["Cancel Import and Exit Program", &skip_option],
                )
                .prompt()?
                .eq(&skip_option)
                {
                    continue;
                } else {
                    return Err("The import encountered existing files which \
                                cannot not be overwritten, and the user \
                                elected to cancel the import."
                        .into());
                }
            }

            // Write the imd's ISO and to cil.
            #[cfg(target_os = "linux")]
            let mount_point = {
                let mut dev_path = PathBuf::from("/dev");
                dev_path.push(&dev);
                dev_path
            };

            #[cfg(target_os = "macos")]
            let mount_point = {
                let mut dev_path = PathBuf::from("/Volumes");
                dev_path.push(&sdl);
                dev_path
            };

            agent.dump_iso(&mount_point, &cil)?;

            // Compute the cvp's file location (cfl) as rdl/cvp_sdl.
            let mut cfl = rdl.clone();
            cfl.push(format!("{cvp}_{sdl}"));

            if cfl.exists() {
                let cfl_s = cfl
                    .to_str()
                    .ok_or(format!("File dump path could not be generated."))?;

                let skip_option =
                    format!("Skip {cvp} and continue to the next identifier.");

                if Select::new(
                    &format!(
                        "The file dump location, {cfl_s} already exists, so \
                         importing {cvp} cannot continue.  Would you like to \
                         skip importing {cvp} and move on to the remaining \
                         records?",
                    ),
                    vec!["Cancel Import and Exit Program", &skip_option],
                )
                .prompt()?
                .eq(&skip_option)
                {
                    continue;
                } else {
                    return Err("The import encountered existing files which \
                                cannot not be overwritten, and the user \
                                elected to cancel the import."
                        .into());
                }
            }

            // Extract the contents of the disk to the cfl.
            agent.copy_rec(mount_point, cfl)?;

            // Fix permissions in the entire rdl since we're probably running as
            // root.
            // agent.fix_permissions(&pdl)?;

            // Eject the disk.
            agent.eject_tray()?;
        }
    }

    Ok(())
}
