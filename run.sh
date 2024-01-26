#! /bin/sh

defaultReadDiskIdentifier="sdd1"
defaultWriteIsoParent="~/Desktop"

function runImport(){
    echo "The import process will now start."

    echo "Reading from disk $readDiskIdentifier"
    echo "Writing ISO to \"$writeIsoPath\""

    echo "Please be patient..."

    read -p 'Would you like to eject the disk? (Y/n): ' ejectDisk
    ejectDisk=${ejectDisk:-'y'}

    echo $ejectDisk

    if [ "$ejectDisk" != "${ejectDisk#[Yy]}" ] ; then 
        if [ $(eject $readDiskIdentifier) ] ; then
            echo "Disk should now be ejected."
        else
            echo "Disk ejection failed.  Please eject the disk manually."
        fi
    else
        echo "Skipping disk ejection."
    fi

}



function startImport(){
    echo "These are the disks attached to the system:"
    lsblk -o name,label,size

    readDiskIdentifier=""
    read -p "Which identifier corresponds to the disk you would like to image? ($defaultReadDiskIdentifier)" readDiskIdentifier
    readDiskIdentifier=${readDiskIdentifier:-$defaultReadDiskIdentifier}

    readDiskLabel=$(lsblk --noheadings -o LABEL /dev/$readDiskIdentifier)

    writeIsoPath=""
    defaultWriteIsoPath="${defaultWriteIsoParent}/${readDiskLabel}.iso"
    read -p "Where would you like the ISO to be written? ($defaultWriteIsoPath)" writeIsoPath
    writeIsoPath="${writeIsoPath:-$defaultWriteIsoPath}"

    if [ ! $(test -d /dev/$readDiskIdentifier) ] ; then 
        echo "Please confirm the following details about the imaging process:"
        echo "    Disk to image: $readDiskIdentifier"
        echo "   Disk has label: $readDiskLabel"
        echo "  ISO Ouptut Path: $writeIsoPath"

        read -p 'Are these details correct? (y/N): ' detailsCorrect

        if [ "$detailsCorrect" != "${detailsCorrect#[Yy]}" ] ;then 
            runImport
        else
            echo "The import process was cancelled."
        fi
    else
        echo "The device '$readDiskIdentifier' could not be found."
    fi
}

function start(){
    echo "-------------------------------------------------------"
    echo "Welcome to the California Revealed Disk Import Utility."
    echo "-------------------------------------------------------"

    while : ; do
        startImport
        echo "-------------------------------------------------------"
        printf 'Would you like to perform another import? (y/N): '
        read importAgain
        [ "$importAgain" != "${importAgain#[Yy]}" ] || break
    done

    echo "Thank you for using the California Revealed Disk Import Utility."
    echo "The program will now exit."
    echo "Goodbye!"
}

start