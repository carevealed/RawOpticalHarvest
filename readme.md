# What does it do?
This tool facilitates the internal California Revealed process of using a CSV file to intake digital objects from partners.
That is, it helps the organization archive received disks.

In broad terms this program will:
  1. Read a list of disks to archive to a specific directory
  2. Prompt the user to insert the disks
  3. Create ISO and file-system copies of the disk
  4. Eject the disk
  5. Check for any remaining disks to read and repeat from 2.

# How do I use it?
The program functions as a command-line tool.
Users should run the compiled binary directly from the terminal.
Use the `-h` flag to view documentation, such as:
`carroh -h`
which should provide the following output:
```
California Revealed Raw Optical Harvest

Usage: carroh [OPTIONS] [Input CSV] [Output Parent Directory] [ROM Device]

Arguments:
  [Input CSV]                Path to the CSV file we want to process
  [Output Parent Directory]  Output parent directory
  [ROM Device]               Device to use as ISO generation source.  If none is provided, the user will be prompted to select a device

Options:
  -d, --dry-run     Don't actually create or modify any files
  -v, --verbose...  Increase logging verbosity
  -q, --quiet...    Decrease logging verbosity
  -h, --help        Print help (see more with '--help')
  -V, --version     Print version
```

## Arguments
### Input CSV
This is an exported document from the California Revealed Archipelago instance.
Users should have previously understood the export process to understand which objects are planned to be imported.
Once downloaded, the *path to the CSV* is provided in this argument.

### Output Parent Directory
This is probably a large mass storage device such as an external drive.
The program will use this directory to create the appropriate `marc`- and `identifier`-based folders for the "in-taken" ISO and file system copies.

### ROM Device
If the third argument is not provided, the program will attempt to help the user decide the appropriate device from which to read.
It does this by printing the results of the system-appropriate command, listing all available devices.
This process can be confusing to non-technical users, but, generally, the ROM Device will be something like `Disk4` on macOS.
Following the in-program instructions should lead users to the same results.
Subsequent runs of the program which should use the same device can include the ROM Device argument to skip this step.

Once a ROM Device has been identified, the same import CSV is linked to that device for the entirety of the CSV intake process.
It is not possible at this time, for example, to use multiple disk drives for the same import, though the inverse is possible (but not suggested).

## Caveats
### Conservative
The program attempts to be very conservative about what changes it makes to the output directory.
If the program detects that files it would otherwise write to already exist, it will prompt the user for guidance.
For existing parent directories, the process is resumable, but if an identifier is detected on disk, the user must decide to exit the program or skip the identifier.
This is because the project does not currently make an attempt to merge directories.

### Dry Run
If users are unsure if they would like to actually modify the contents of the disk but instead just view what the program would do, they can use the `-d` flag.

### Prompts
When prompted for `Yes` or `No` answers, the user may use abbreviated forms, such as `y` or `no`.

### Verbosity
The verbosity flag is flexible.
Users may issue a single `v` and up to four `v`'s to incur progressively more logging.
For example, `-v` will display some additional output such as commands being issued, and `-vvvv` will output all possible details.

### Disk Settling
Sadly there is no good way to identify when a disk has been inserted into the system until the media is mounted by the operating system.
For this reason, when the user is prompted to identify if the disk has been inserted, they should only answer `yes` when the disk has been mounted and is visible to them.
For example, if the Finder does not see the disk, the user should wait to answer the prompt.
If the program does not detect a disk when it is expected, it will wait 5 seconds and then check again.
If the disk is still not visible, the program will exit.

### Resumption
Because of the program's behavior regarding existing files, a given CSV may be interrupted after an identifier is successfully imported and resumed at a later date.
In these circumstances, the user will be prompted to skip the pre-existing imported items that are discovered on disk, until the program finds one it has not yet handled.

### Initial Disk
ROM Devices will not display to the device identification process unless they have media in them.
If the user is prompted to identify the disk in the drive while the media is inserted, but are unsure if the media matches the corresponding identifier, they may answer `No` to that prompt.
The system should then eject the media, allowing the user to positively identify the media and take any appropriate corrective action.