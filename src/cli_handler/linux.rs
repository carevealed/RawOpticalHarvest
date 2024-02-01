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

pub struct LinuxCliHandler {}

impl CliHandler for LinuxCliHandler
{
    fn select_rom_device(&self) -> Result<String, Box<dyn Error>>
    {
        println!(
            "{}",
            Command::new("lsblk")
                .arg("--all")
                .arg("-o")
                .arg("name,label,size")
                .run()?
        );

        let dev = Text::new(
            "Enter the device NAME you would like to image from for this \
             session:",
        )
        .prompt()?;

        Ok(dev)
    }

    fn eject_tray(&self) -> Result<(), Box<dyn Error>>
    {
        Command::new("eject").run().map(|_| ())
    }

    fn get_rom_device_label(
        &self,
        dev: &String,
    ) -> Result<String, Box<dyn Error>>
    {
        lsblk_dev_label(dev)?
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

        Command::new("dd")
            .arg(format!("if={from}"))
            .arg(format!("of={to}"))
            .arg("conv=noerror,sync")
            .arg("bs=1M")
            .run()
            .map(|_| ())
            .map_err(|e| {
                format!(
                    "Failure while trying to run dd.  Should this program be \
                     running as root? Details:\n{e}"
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
            .arg(format!("666"))
            .arg(format!("-R"))
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
    //     mount_point: &PathBuf,
    // ) -> Result<(), Box<dyn Error>>
    // {
    //     debug!(
    //         "Mounting ISO file from {iso_path:?} at mount point \
    //          {mount_point:?}"
    //     );

    //     Command::new("mount")
    //         .arg(format!("-o"))
    //         .arg(format!("loop"))
    //         .arg(iso_path)
    //         .arg(mount_point)
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

fn lsblk_dev_label(dev: &String) -> Result<String, Box<dyn Error>>
{
    let dev_path = {
        let mut p = PathBuf::from("/dev");
        p.push(dev);
        path::absolute(p)?.into_os_string()
    };

    debug!("Searching for label for device {dev:?}");

    Command::new("lsblk")
        .arg("--noheadings")
        .arg("--output")
        .arg("LABEL")
        .arg(dev_path)
        .run()
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn find_sda1_on_sulla()
    {
        assert_eq!(
            "Slow",
            LinuxCliHandler {}
                .get_rom_device_label(&"sda1".to_string())
                .unwrap()
        );
    }
    #[test]
    fn sda1_lsblk_label()
    {
        assert_eq!("Slow\n", lsblk_dev_label(&"sda1".to_string()).unwrap());
    }
}
