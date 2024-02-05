use super::{
    CliHandler,
    CliHandlerExtras,
};
use inquire::Text;
use log::debug;
use std::{
    error::Error,
    path::{
        self,
        PathBuf,
    },
    process::Command,
};

pub struct MacosCliHandler {}

impl CliHandler for MacosCliHandler
{
    fn select_rom_device(&self) -> Result<String, Box<dyn Error>>
    {
        println!("{}", Command::new("diskutil").arg("list").run()?);

        let dev = Text::new(
            "Enter the DISK identifier you would like to image from for \
             this session.  (Do not enter the partition identifier.  For example, disk4 is correct, but disk4s1 is not.):",
        )
        .prompt()?;

        Ok(dev)
    }

    fn eject_tray(&self) -> Result<(), Box<dyn Error>>
    {
        Command::new("drutil")
            .arg("tray")
            .arg("eject")
            .run()
            .map(|_| ())
    }

    fn get_rom_device_label(
        &self,
        dev: &String,
    ) -> Result<String, Box<dyn Error>>
    {
        // TODO: parse diskutil output
        diskutil_dev_label(dev)?
            .lines()
            .into_iter()
            .next()
            .ok_or(format!("Device '{dev}' label could not be found.").into())
            .map(|label| label.into())
    }

    fn dump_iso(
        &self,
        from: &PathBuf,
        to: &PathBuf,
    ) -> Result<(), Box<dyn Error>>
    {
        debug!("Dumping files from: {from:?} to: {to:?}");

        let from = path::absolute(from)?
            .into_os_string()
            .into_string()
            .unwrap();
        let to = path::absolute(to)?.into_os_string().into_string().unwrap();

        Command::new("hdiutil")
            .arg(format!("makehybrid"))
            .arg(format!("-iso"))
            .arg(format!("-joliet"))
            .arg(format!("-o"))
            .arg(format!("{to}"))
            .arg(format!("{from}"))
            .run()
            .map(|_| ())
            .map_err(|e| {
                format!(
                    "Failure while dumping ISO. Details:\n{e}"
                )
                .into()
            })
    }

    fn fix_permissions(
        &self,
        in_path: &PathBuf,
    ) -> Result<(), Box<dyn Error>>
    {
        debug!("Fixing permissions fo files in {in_path:?}");

        let in_path = path::absolute(in_path)?.into_os_string();

        Command::new("chmod")
            .arg("666")
            .arg("--recursive")
            .arg(in_path)
            .run()
            .map(|_| ())
            .map_err(|e| {
                format!("Failure while trying to fix permissions: {e}").into()
            })
    }

    // fn mount_iso(
    //     &self,
    //     iso_path: &PathBuf,
    // ) -> Result<PathBuf, Box<dyn Error>>
    // {
    //     debug!(
    //         "Mounting ISO file from {iso_path:?} at mount point \
    //          {mount_point:?}"
    //     );

    //     Command::new("hdiutil")
    //         .arg("mount")
    //         .arg(iso_path)
    //         .run()
    //         .map(|_| ())
    //         .map_err(|e| {
    //             format!("Failure while trying to mount ISO: {e}").into()
    //         })
    // }

    fn copy_rec(
        &self,
        from: &PathBuf,
        to: &PathBuf,
    ) -> Result<(), Box<dyn Error>>
    {
        debug!("Copying files from {from:?} to {to:?}");

        Command::new("cp")
            .arg("--recursive")
            .arg(from)
            .arg(to)
            .run()
            .map(|_| ())
            .map_err(|e| format!("Failure while copying files: {e}").into())
    }
}

fn diskutil_dev_label(dev: &String) -> Result<String, Box<dyn Error>>
{
    debug!("Searching for label for device {dev:?}");

    Command::new("diskutil").arg("information").arg(dev).run()
}
