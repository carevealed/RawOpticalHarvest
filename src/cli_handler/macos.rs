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
}

fn diskutil_dev_label(dev: &String) -> Result<String, Box<dyn Error>>
{
    debug!("Searching for label for device {dev:?}");

    Command::new("diskutil").arg("information").arg(dev).run()
}
