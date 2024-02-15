use super::csv_processor::{
    equal_column::EqualColumnValidator,
    first_value::FirstValueFetcher,
    populated_column::PopulatedColumn,
    row_printer::RowPrinter,
};
use crate::{
    cli::Cli,
    cli_handler::*,
    csv_processor::path_validator::{
        DirectoryStatus,
        PathValidationOptions,
        PathValidator,
    },
};
use inquire::{
    Confirm,
    Text,
};
use log::info;
use std::{
    error::Error,
    fs,
    path::PathBuf,
};
// use tempfile::TempDir;

fn get_cli_handler() -> Box<dyn CliHandler>
{
    #[cfg(target_os = "macos")]
    return Box::new(MacosCliHandler {});

    #[cfg(target_os = "linux")]
    return Box::new(LinuxCliHandler {});
}

pub struct Agent
{
    args: Cli,
    cli_handler: Box<dyn CliHandler>,
}

impl Agent
{
    pub fn new(args: Cli) -> Result<Agent, Box<dyn Error>>
    {
        let cli_handler = get_cli_handler();

        Ok(Agent { args, cli_handler })
    }

    pub fn create_dir_or_prompt_if_exists(
        &self,
        path: &PathBuf,
    ) -> Result<(), Box<dyn Error>>
    {
        let path_s = path
            .to_str()
            .ok_or(format!("Directory path could not be generated."))?;

        if path.exists() {
            if path.is_file() {
                return Err(format!(
                    "File {path_s} already exists and is a file, but should \
                     be a directory."
                )
                .into());
            }

            if Confirm::new(&format!(
                "{path_s} already exists.  Would you like to continue \
                 importing? (Yes/No)"
            ))
            .prompt()?
            {
                Ok(())
            } else {
                Err(format!(
                    "File {path_s} already exists and user has declined to \
                     continue."
                )
                .into())
            }
        } else if self.args.dry_run {
            println!("Dry Run: Skipping creating {path_s}");
            Ok(())
        } else {
            // Create the directory.
            self.create_directory(&path)?;
            Ok(())
        }
    }

    pub fn get_input_csv_path(&self) -> PathBuf
    {
        let mut path = self.args.csv_path.clone();

        loop {
            match path {
                | None => {
                    path =
                        Text::new("Please provide the path to the input CSV:")
                            .prompt()
                            .ok();
                }
                | Some(p) => {
                    let path_pb = PathBuf::from(p);

                    match path_pb.validate_path(PathValidationOptions::Exists(
                        DirectoryStatus::IsNotDirectory,
                    )) {
                        | Err(e) => {
                            eprintln!("Error with input CSV Path: {e}");
                            path = None;
                        }
                        | Ok(()) => {
                            return path_pb;
                        }
                    }
                }
            }
        }
    }

    pub fn get_output_parent(&self) -> PathBuf
    {
        let mut path = self.args.output_parent_path.clone();

        loop {
            match path {
                | None => {
                    path = Text::new(
                        "Please provide the path to the output parent \
                         directory:",
                    )
                    .prompt()
                    .ok();
                }
                | Some(p) => {
                    let path_pb = PathBuf::from(p);

                    match path_pb.validate_path(PathValidationOptions::Exists(
                        DirectoryStatus::IsDirectory,
                    )) {
                        | Err(e) => {
                            eprintln!(
                                "Error while with output parent directory: {e}"
                            );
                            path = None;
                        }
                        | Ok(()) => {
                            return path_pb;
                        }
                    }
                }
            }
        }
    }

    pub fn print_all_rows(
        &mut self,
        source_csv_path: &PathBuf,
    ) -> Result<(), Box<dyn Error>>
    {
        source_csv_path.print_all_rows()
    }

    pub fn assert_equal_column_values(
        &self,
        column_header: &String,
        source_csv_path: &PathBuf,
    ) -> Result<(), Box<dyn Error>>
    {
        source_csv_path.assert_equal_column_values(column_header)
    }

    pub fn pick_populated_column<'a>(
        &self,
        column_header_one: &'a String,
        column_header_two: &'a String,
        source_csv_path: &PathBuf,
    ) -> Result<&'a String, Box<dyn Error>>
    {
        source_csv_path
            .get_populated_column(column_header_one, column_header_two)
    }

    pub fn first_value(
        &mut self,
        column_header: &String,
        source_csv_path: &PathBuf,
    ) -> Result<String, Box<dyn Error>>
    {
        source_csv_path.get_first_value(column_header)
    }

    pub fn create_directory(
        &self,
        pdl: &PathBuf,
    ) -> Result<(), Box<dyn Error>>
    {
        info!("Create output location: {pdl:?}");
        if pdl.exists() {
            return Err(
                format!("Output location {pdl:?} already exists.",).into()
            );
        }

        if self.args.dry_run {
            info!("Dry run: Skipping creating the output directory.");
            return Ok(());
        }

        fs::create_dir(pdl)?;

        Ok(())
    }

    pub fn select_rom_device(&self) -> Result<String, Box<dyn Error>>
    {
        match &self.args.rom_device {
            | Some(d) => Ok(d.clone()),
            | None => {
                self.cli_handler.select_rom_device().map_err(|e| {
                    format!("Error while selecting ROM device: {e}").into()
                })
            }
        }
    }

    pub fn eject_tray(&self) -> Result<(), Box<dyn Error>>
    {
        self.cli_handler
            .eject_tray()
            .map_err(|e| format!("Error while ejecting: {e}").into())
    }

    pub fn get_rom_device_label(
        &self,
        dev: &String,
    ) -> Result<String, Box<dyn Error>>
    {
        let label = self.cli_handler.get_rom_device_label(dev)?;

        println!("Disk has label: {label}");

        Ok(label)
    }

    pub fn dump_iso(
        &self,
        from: &PathBuf,
        to: &PathBuf,
    ) -> Result<(), Box<dyn Error>>
    {
        println!("Creating ISO from {from:?} at {to:?}.");

        if self.args.dry_run {
            info!("Dry run: Skipping ISO dump.");
            return Ok(());
        }

        println!("Please wait...");
        self.cli_handler.dump_iso(from, to)?;
        println!("ISO dump finished.");

        Ok(())
    }

    pub fn fix_permissions(
        &self,
        in_path: &PathBuf,
    ) -> Result<(), Box<dyn Error>>
    {
        info!("Fixing permissions in {in_path:?}.");

        if self.args.dry_run {
            info!("Dry run: Skipping permissions fix.");
            return Ok(());
        }

        self.cli_handler.fix_permissions(in_path)
    }

    pub fn copy_rec(
        &self,
        from: PathBuf,
        to: PathBuf,
    ) -> Result<(), Box<dyn Error>>
    {
        println!("Copying files from {from:?} to {to:?}.");

        println!("Please wait...");
        if self.args.dry_run {
            info!("Dry run: Skipping Copy.");
            return Ok(());
        }

        self.cli_handler.copy_rec(&from, &to)?;
        println!("File copy finished.");

        Ok(())
    }
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_cli_handler_copy_single_file()
    {
        let from = PathBuf::from("./demo/out_exists.csv");
        assert!(from.exists());
        assert!(!from.is_dir());

        let to = PathBuf::from("./demo/out/out_exists_copy.csv");
        assert!(!to.exists());

        let clih = get_cli_handler();

        clih.copy_rec(&from, &to).unwrap();
        assert!(to.exists());
        assert!(!to.is_dir());

        fs::remove_file(to).unwrap();
    }

    #[test]
    fn test_cli_handler_copy_dir()
    {
        let from = PathBuf::from("./demo/ram_disk_template_contents");
        assert!(from.exists());
        assert!(from.is_dir());

        let to = PathBuf::from("./demo/out/ram_disk_template_contents");
        assert!(!to.exists());

        let clih = get_cli_handler();

        clih.copy_rec(&from, &to).unwrap();
        assert!(to.exists());
        assert!(to.is_dir());

        assert_eq!(1, to.read_dir().unwrap().collect::<Vec<_>>().len());

        fs::remove_dir_all(to).unwrap();
    }

    // #[test]
    // fn test_mount_iso()
    // {
    //     struct DumpTestManager {}
    //     impl Drop for DumpTestManager
    //     {
    //         fn drop(&mut self)
    //         {
    //             // fs::remove_dir_all("./demo/out/demo_iso").unwrap();
    //         }
    //     }

    //     let dtm = DumpTestManager {};
    //     let from = PathBuf::from("./demo/demo.iso");
    //     assert!(from.exists());
    //     assert!(!from.is_dir());

    //     let to = PathBuf::from("./demo/out/demo_iso");
    //     assert!(!to.exists());

    //     let clih = get_cli_handler();

    //     clih.mount_iso(&from, &to).unwrap();

    //     assert_eq!(
    //         1,
    //         to.read_dir().unwrap().collect::<Vec<_>>().len()
    //     );

    //     // assert_eq!(
    //     //     "testfile.txt",
    //     //     mount_point.read_dir().unwrap().next().unwrap().unwrap().
    //     // file_name() );

    //     // clih.copy_rec(&mount_point, &to).unwrap();

    //     // let mut test_file = to.clone();
    //     // test_file.push("testfile.txt");
    //     // assert!(test_file.exists());
    //     // assert!(!test_file.is_dir());

    //     // let contained_file_contents = "Some content";
    //     // assert_eq!(
    //     //     contained_file_contents,
    //     //     fs::read_to_string(test_file).unwrap()
    //     // );

    //     drop(dtm);
    // }

    // #[test]
    // fn test_iso_dump()
    // {
    //     struct DumpTestManager {}
    //     impl Drop for DumpTestManager
    //     {
    //         fn drop(&mut self)
    //         {
    //             // fs::remove_dir_all("./demo/out/demo_iso").unwrap();
    //         }
    //     }

    //     let dtm = DumpTestManager {};
    //     let from = PathBuf::from("./demo/demo.iso");
    //     assert!(from.exists());
    //     assert!(!from.is_dir());

    //     let to = PathBuf::from("./demo/out/demo_iso");
    //     assert!(!to.exists());
    //     fs::create_dir(&to).unwrap();

    //     let mount_point = TempDir::new().unwrap();
    //     let mount_point = mount_point.path().to_path_buf().into();

    //     let clih = get_cli_handler();

    //     clih.mount_iso(&from, &mount_point).unwrap();

    //     // assert_eq!(
    //     //     1,
    //     //     mount_point.read_dir().unwrap().collect::<Vec<_>>().len()
    //     // );

    //     // assert_eq!(
    //     //     "testfile.txt",
    //     //     mount_point.read_dir().unwrap().next().unwrap().unwrap().
    //     // file_name() );

    //     // clih.copy_rec(&mount_point, &to).unwrap();

    //     // let mut test_file = to.clone();
    //     // test_file.push("testfile.txt");
    //     // assert!(test_file.exists());
    //     // assert!(!test_file.is_dir());

    //     // let contained_file_contents = "Some content";
    //     // assert_eq!(
    //     //     contained_file_contents,
    //     //     fs::read_to_string(test_file).unwrap()
    //     // );

    //     drop(dtm);
    // }
}
