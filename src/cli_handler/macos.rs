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
        println!("{}", Command::new("drutil").arg("--status").run()?);

        let dev = Text::new(
            "Enter the device NAME you would like to image from for this \
             session:",
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
            .arg(format!("--recursive"))
            .arg(in_path)
            .run()
            .map(|_| ())
            .map_err(|e| {
                format!("Failure while trying to fix permissions: {e}").into()
            })
    }

    fn mount_iso(
        &self,
        iso_path: &PathBuf,
        mount_point: &PathBuf,
    ) -> Result<(), Box<dyn Error>>
    {
        debug!(
            "Mounting ISO file from {iso_path:?} at mount point \
             {mount_point:?}"
        );

        Command::new("mount")
            .arg(format!("--options"))
            .arg(format!("loop"))
            .arg(iso_path)
            .arg(mount_point)
            .run()
            .map(|_| ())
            .map_err(|e| {
                format!("Failure while trying to mount ISO: {e}").into()
            })
    }

    fn copy_rec(
        &self,
        from: &PathBuf,
        to: &PathBuf,
    ) -> Result<(), Box<dyn Error>>
    {
        debug!("Copying files from {from:?} to {to:?}");

        Command::new("cp")
            .arg(format!("--recursive"))
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
