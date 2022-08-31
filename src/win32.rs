use windows::{
    core::PCWSTR,
    Win32::{
        Graphics::Gdi::{
            ChangeDisplaySettingsExW, EnumDisplayDevicesW, EnumDisplaySettingsExW,
            EnumDisplaySettingsW, CDS_GLOBAL, CDS_UPDATEREGISTRY, DEVMODEW, DISPLAY_DEVICEW,
            DISP_CHANGE_SUCCESSFUL, ENUM_CURRENT_SETTINGS,
        },
        UI::WindowsAndMessaging::{EDD_GET_DEVICE_INTERFACE_NAME, EDS_RAWMODE},
    },
};

use std::mem::size_of;

pub(crate) fn set_display_settings(display_device_name: PCWSTR, dev_mode: &DEVMODEW) -> bool {
    unsafe {
        ChangeDisplaySettingsExW(
            display_device_name,
            dev_mode,
            None,
            CDS_GLOBAL | CDS_UPDATEREGISTRY,
            std::ptr::null(),
        ) == DISP_CHANGE_SUCCESSFUL
    }
}

pub(crate) fn get_display_settings(display_device_name: PCWSTR) -> Option<DEVMODEW> {
    let mut dev_mode = DEVMODEW {
        dmSize: std::mem::size_of::<DEVMODEW>().try_into().unwrap(),
        dmDriverExtra: 0,
        ..Default::default()
    };

    let r = unsafe {
        EnumDisplaySettingsExW(
            display_device_name,
            ENUM_CURRENT_SETTINGS,
            &mut dev_mode,
            EDS_RAWMODE,
        )
    }
    .as_bool();

    r.then_some(dev_mode)
}

pub(crate) fn get_display_device(index: u32) -> Option<DISPLAY_DEVICEW> {
    let mut dm_info = DISPLAY_DEVICEW {
        cb: size_of::<DISPLAY_DEVICEW>().try_into().unwrap(),
        ..Default::default()
    };

    let r = unsafe {
        EnumDisplayDevicesW(None, index, &mut dm_info, EDD_GET_DEVICE_INTERFACE_NAME).as_bool()
    };

    r.then_some(dm_info)
}

pub(crate) fn get_display_x_y_position(display_device_name: PCWSTR) -> Option<(i32, i32)> {
    let mut dev_mode = DEVMODEW {
        dmSize: size_of::<DEVMODEW>().try_into().unwrap(),
        dmDriverExtra: 0,
        ..Default::default()
    };

    let r = unsafe {
        EnumDisplaySettingsW(display_device_name, ENUM_CURRENT_SETTINGS, &mut dev_mode).as_bool()
    };

    r.then_some(unsafe {
        (
            dev_mode.Anonymous1.Anonymous2.dmPosition.x,
            dev_mode.Anonymous1.Anonymous2.dmPosition.y,
        )
    })
}
