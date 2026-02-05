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
mod build_env;
mod win32;

use build_env::get_build_env;
use windows::Win32::Graphics::Gdi::{
    DISPLAY_DEVICE_ACTIVE, DISPLAY_DEVICE_MIRRORING_DRIVER, DISPLAY_DEVICE_MODESPRUNED,
    DISPLAY_DEVICE_PRIMARY_DEVICE, DISPLAY_DEVICE_REMOVABLE, DISPLAY_DEVICE_VGA_COMPATIBLE,
    DM_POSITION,
};
use windows::core::PCWSTR;

use crate::win32::{get_display_device, get_display_settings, get_display_x_y_position};

fn print_header() {
    const NAME: &str = env!("CARGO_PKG_NAME");
    const VERSION: &str = env!("CARGO_PKG_VERSION");

    let build_env = get_build_env();

    println!(
        "{} v{} - built for {} ({})",
        NAME,
        VERSION,
        build_env.get_target(),
        build_env.get_target_cpu().unwrap_or("base cpu variant"),
    );
}

fn main() {
    print_header();

    let args = std::env::args().collect::<Vec<_>>();

    if let Some(first) = args.get(1) {
        if first.eq_ignore_ascii_case("/H") || first.eq_ignore_ascii_case("/?") {
            show_help();
        } else if first.eq_ignore_ascii_case("/L") {
            let all = args.get(2).is_some_and(|a| a.eq_ignore_ascii_case("/A"));

            list_displays(all);
        } else {
            set_positions(&args);
        }
    } else {
        show_help();
    }
}

fn show_help() {
    println!("Help");
    println!("DPEdit 1.1.0");
    println!("A command line utility to accurately position displays in a multi-monitor setup.\n");
    println!("Usage: dpedit.exe");
    println!("       dpedit.exe /L");
    println!("       dpedit.exe <displayNum> <xPos> <yPos> [<displayNum2> <xPos2> <yPos2>] ...\n");
    println!("  Options:\n");
    println!("  /L              Lists all displays and their indices");
    println!("  /A              Forces /L to list all registered displays");
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

fn set_positions(args: &[String]) {
    for chunk in args[1..].chunks_exact(3) {
        let display_index = chunk[0].parse::<u32>().unwrap() - 1;
        let x_pos = chunk[1].parse().unwrap();
        let y_pos = chunk[2].parse().unwrap();

        println!("Applying position {{{x_pos}, {y_pos}}} to Display #{display_index}...");

        match set_display_settings(display_index, x_pos, y_pos) {
            Err(error) => println!("{error}\nSkipping...\n"),
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

fn list_displays(show_all: bool) {
    let mut index = 0;

    while let Some(display_device) = get_display_device(index) {
        index += 1;

        if show_all || (display_device.StateFlags.contains(DISPLAY_DEVICE_ACTIVE)) {
            println!();
            println!("Display #{}", index + 1);
            println!(
                "Device name: {}",
                String::from_utf16_lossy(&display_device.DeviceName)
            );
            println!(
                "Device string: {}",
                String::from_utf16_lossy(&display_device.DeviceString)
            );
            println!(
                "Active: {}",
                (display_device.StateFlags.contains(DISPLAY_DEVICE_ACTIVE))
            );
            println!(
                "Mirroring: {}",
                (display_device
                    .StateFlags
                    .contains(DISPLAY_DEVICE_MIRRORING_DRIVER))
            );
            println!(
                "Modes pruned: {}",
                (display_device
                    .StateFlags
                    .contains(DISPLAY_DEVICE_MODESPRUNED))
            );
            println!(
                "Primary: {}",
                (display_device
                    .StateFlags
                    .contains(DISPLAY_DEVICE_PRIMARY_DEVICE))
            );
            println!(
                "Removable: {}",
                (display_device.StateFlags.contains(DISPLAY_DEVICE_REMOVABLE))
            );
            println!(
                "VGA compatible: {}",
                (display_device
                    .StateFlags
                    .contains(DISPLAY_DEVICE_VGA_COMPATIBLE))
            );

            if let Some(((width, height), (x, y))) =
                get_display_x_y_position(PCWSTR(display_device.DeviceName.as_ptr()))
            {
                println!("Dimensions: {{{}, {}}}", width, height);
                println!("Position: {{{}, {}}}", x, y);
            }
        }
    }
}
