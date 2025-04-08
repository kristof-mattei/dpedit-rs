#![cfg_attr(not(debug_assertions), deny(warnings))]
// /*--------------------------------------------------------------
//  | Display Position Editor v1.1.0                               |
//  | By Benjamin J. Pryor                                         |
//  |--------------------------------------------------------------|
//  | A simple command line utility to accurately set the relative |
//  | position of monitors in a dual- or multi- monitor setup.     |
//   --------------------------------------------------------------*/
// /*
// * Original code from here:
// * https://what.thedailywtf.com/topic/26137/windows-10-display-arranging-sucks-so-i-fixed-it-by-setting-the-pixel-positions-manually
// * Credit to LB_ for the basic methods used in this utility.
// * Credit to GreenYun for the /L routine, and for correcting my mistaken understanding of display indices
// */
mod win32;

use windows::Win32::Graphics::Gdi::{
    DISPLAY_DEVICE_ACTIVE, DISPLAY_DEVICE_MIRRORING_DRIVER, DISPLAY_DEVICE_MODESPRUNED,
    DISPLAY_DEVICE_PRIMARY_DEVICE, DISPLAY_DEVICE_REMOVABLE, DISPLAY_DEVICE_VGA_COMPATIBLE,
    DM_POSITION,
};
use windows::core::PCWSTR;

use crate::win32::{get_display_device, get_display_settings, get_display_x_y_position};

fn main() {
    let args = std::env::args().collect::<Vec<_>>();

    if args.len() > 1 {
        if args.get(1).map_or(false, |a| a == "/L") {
            list_displays();
        } else {
            set_positions(&args);
        }
    } else {
        println!("Help");
        println!("DPEdit 1.1.0");
        println!(
            "A command line utility to accurately position displays in a multi-monitor setup.\n"
        );
        println!("Usage: dpedit.exe");
        println!("       dpedit.exe /L");
        println!(
            "       dpedit.exe <displayNum> <xPos> <yPos> [<displayNum2> <xPos2> <yPos2>] ...\n"
        );
        println!("  Options:\n");
        println!("  /L              Lists all displays and their indices");
        println!("  <displayNum>    The index of the display to position");
        println!(
            "  <xPos>          The X (horizontal) position of the top-left corner of display <displayNum>."
        );
        println!(
            "  <YPos>          The Y (vertical) position of the top-left corner of display <displayNum>.\n"
        );
        println!("Example: dpedit.exe 1 0 0 2 -1920 21");
        println!(
            "         Moves Display #1 to coords {{0, 0}} and positions Display #2 to the left of"
        );
        println!(
            "         and 21 pixels lower than Display #1 (coords {{-1920, 21}}). This example assumes"
        );
        println!("         Display #2 to be 1080p.\n");
        println!("Notes: This utility should work for any number and any size(s) of monitors.");
        println!("       The display numbers do not need to be in order.\n");
        println!("THIS UTILITY MODIFIES THE REGISTRY! USE AT YOUR OWN RISK!");
    }
}

fn set_positions(args: &[String]) {
    for chunk in args[1..].chunks_exact(3) {
        let display_index = chunk[0].parse::<u32>().unwrap() - 1;
        let x_pos = chunk[1].parse().unwrap();
        let y_pos = chunk[2].parse().unwrap();

        println!("Applying position {{{x_pos}, {y_pos}}} to Display #{display_index}...");

        match set_display_settings(display_index, x_pos, y_pos) {
            Err(e) => println!("{e}\nSkipping...\n"),
            _ => println!("Done!"),
        }
    }
}

fn set_display_settings(display_index: u32, x_pos: i32, y_pos: i32) -> Result<(), &'static str> {
    let dm_info = get_display_device(display_index)
        .ok_or("Operation failed! Unable to connect to display.")?;

    let mut dev_mode = get_display_settings(PCWSTR(dm_info.DeviceName.as_ptr()))
        .ok_or("Operation failed! Unable to read display settings.")?;

    dev_mode.dmFields = DM_POSITION;
    dev_mode.Anonymous1.Anonymous2.dmPosition.x = x_pos;
    dev_mode.Anonymous1.Anonymous2.dmPosition.y = y_pos;

    win32::set_display_settings(PCWSTR(dm_info.DeviceName.as_ptr()), &dev_mode)
        .then_some(())
        .ok_or("Operation failed! Unable to write to display settings.")
}

fn list_displays() {
    let mut index = 0;

    while let Some(display_device) = get_display_device(index) {
        index += 1;

        println!(
            "Display #{index}\n\
            Device name: {}\n\
            Device string: {}\n\
            Active: {:?}\n\
            Mirroring: {:?}\n\
            Modes pruned: {:?}\n\
            Primary: {:?}\n\
            Removable: {:?}\n\
            VGA compatible: {:?}",
            String::from_utf16_lossy(&display_device.DeviceName),
            String::from_utf16_lossy(&display_device.DeviceString),
            display_device.StateFlags & DISPLAY_DEVICE_ACTIVE,
            display_device.StateFlags & DISPLAY_DEVICE_MIRRORING_DRIVER,
            display_device.StateFlags & DISPLAY_DEVICE_MODESPRUNED,
            display_device.StateFlags & DISPLAY_DEVICE_PRIMARY_DEVICE,
            display_device.StateFlags & DISPLAY_DEVICE_REMOVABLE,
            display_device.StateFlags & DISPLAY_DEVICE_VGA_COMPATIBLE
        );

        if let Some((x, y)) = get_display_x_y_position(PCWSTR(display_device.DeviceName.as_ptr())) {
            println!("Position: {{{}, {}}}", x, y);
        }

        println!();
    }
}
