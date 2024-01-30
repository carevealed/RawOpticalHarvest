Find the index of the marc column using the header.  If it is not there, indicate the error and exit.
For every line in the CSV, verify that the marc column is equal to the previous row's value.  If a row has a different value, print the invalid lines and exit.
- Should this be done for the grant cycle as well?
Determine if the file should use the obj_call_number column (ocn) or obj_temporary_id column (oti) field as the per-item identifier column (pit)
- Check every line in the CSV for the existence of either ocn or oti.
- If ocn is not available in every line, and oti is not available on every line, print an error that one is required on all lines and exit.
- If the ocn exists on all lines but not oti, use the ocn as the pit. Print the selected choice.
- If the oti exists on all lines but not ocn, use the oti as the pit. Print the selected choice.
- If all lines contain both ocn and oti, use the ocn as the pit. Print the selected choice.
Compute the grant cycle descriptor (gcd):
- Take the obj_grant_cycle field from the first row.
- Substitute any "/" characters for "-", giving the gcd.
Compute the parent directory location (pdl):
- Prompt the user for the location of the output folder parent (ofp).
- Calculate pdl as ofp/gcd + "_" + marc
Check if the pdl exists.  If it does, prompt the user to continue.  If it does not, create it.
Compute the raw file directory location (rdl) as pdl/marc + "_" + gcd + "_Raw".
Check if the rdl exists.  If it does, prompt the user to continue.  If it does not, create it.
Prompt the user to select the imaging device (imd) from the local system devices.  Use a hard-coded default.
For every line in the CSV:
- For each semi-colon-separated value in the pit (cvp):
  - Prompt the user to locate and insert the disc associated with the cvp.
  - Wait for the user to press enter to continue.
  - Retain the system's disk label (sdl) from the imd.
  - Compute the cvp's iso location (cil) as rdl/sdl + ".iso"
  - Compute the cvp's file location (cfl) as rdl/sdl.
  - Generate the imd's ISO and write it to cil.
  - Extract the contents of the cil to the cfl.
  - Eject the disk.