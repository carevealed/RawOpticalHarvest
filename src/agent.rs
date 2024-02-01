use super::csv_processor::{
    equal_column::EqualColumnValidator,
    first_value::FirstValueFetcher,
    populated_column::PopulatedColumn,
    row_printer::RowPrinter,
};
use crate::cli_handler::*;
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
    source_csv_path: PathBuf,
    dry: bool,
    cli_handler: Box<dyn CliHandler>,
}

impl Agent
{
    pub fn new(
        source_csv_path: PathBuf,
        dry: bool,
    ) -> Result<Agent, Box<dyn Error>>
    {
        let cli_handler = get_cli_handler();

        Ok(Agent {
            source_csv_path,
            dry,
            cli_handler,
        })
    }

    pub fn print_all_rows(&mut self) -> Result<(), Box<dyn Error>>
    {
        self.source_csv_path.print_all_rows()
    }

    pub fn assert_equal_column_values(
        &self,
        column_header: &String,
    ) -> Result<(), Box<dyn Error>>
    {
        self.source_csv_path
            .assert_equal_column_values(column_header)
    }

    pub fn pick_populated_column<'a>(
        &self,
        column_header_one: &'a String,
        column_header_two: &'a String,
    ) -> Result<&'a String, Box<dyn Error>>
    {
        self.source_csv_path
            .get_populated_column(column_header_one, column_header_two)
    }

    pub fn first_value(
        &mut self,
        column_header: &String,
    ) -> Result<String, Box<dyn Error>>
    {
        self.source_csv_path.get_first_value(column_header)
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

        if self.dry {
            info!("Dry run: Skipping creating the output directory.");
            return Ok(());
        }

        fs::create_dir(pdl)?;

        Ok(())
    }

    pub fn select_rom_device(&self) -> Result<String, Box<dyn Error>>
    {
        self.cli_handler.select_rom_device().map_err(|e| {
            format!("Error while selecting ROM device: {e}").into()
        })
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
        info!("Creating ISO from {from:?} to at {to:?}.");

        if self.dry {
            info!("Dry run: Skipping ISO dump.");
            return Ok(());
        }

        println!("Please wait...");
        let result = self.cli_handler.dump_iso(from, to).map_err(|e| {
            format!(
                "Error while dumping ISO (from: '{from:?}', to: '{to:?}'): {e}"
            )
            .into()
        });
        println!("ISO dump finished.");

        result
    }

    pub fn fix_permissions(
        &self,
        in_path: &PathBuf,
    ) -> Result<(), Box<dyn Error>>
    {
        info!("Fixing permissions in {in_path:?}.");

        if self.dry {
            info!("Dry run: Skipping permissions fix.");
            return Ok(());
        }

        self.cli_handler.fix_permissions(in_path)
    }

    // pub fn extract_iso(
    //     &self,
    //     from: PathBuf,
    //     to: PathBuf,
    // ) -> Result<(), Box<dyn Error>>
    // {
    //     if self.dry {
    //         info!("Dry run: Skipping ISO extraction.");
    //         return Ok(());
    //     }

    //     let mount_point = TempDir::new()?;
    //     let mount_point = mount_point.path().into();

    //     self.cli_handler.mount_iso(&from, &mount_point)?;

    //     self.cli_handler.copy_rec(&mount_point, &to)
    // }
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
