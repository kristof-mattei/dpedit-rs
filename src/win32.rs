use std::mem::size_of;

use windows::Win32::Graphics::Gdi::{
    CDS_GLOBAL, CDS_UPDATEREGISTRY, ChangeDisplaySettingsExW, DEVMODEW, DISP_CHANGE_SUCCESSFUL,
    DISPLAY_DEVICEW, EDS_RAWMODE, ENUM_CURRENT_SETTINGS, EnumDisplayDevicesW,
    EnumDisplaySettingsExW, EnumDisplaySettingsW,
};
use windows::Win32::UI::WindowsAndMessaging::EDD_GET_DEVICE_INTERFACE_NAME;
use windows::core::PCWSTR;

pub(crate) fn set_display_settings(display_device_name: PCWSTR, dev_mode: &DEVMODEW) -> bool {
    // SAFETY: API call
    unsafe {
        ChangeDisplaySettingsExW(
            display_device_name,
            Some(dev_mode),
            None,
            CDS_GLOBAL | CDS_UPDATEREGISTRY,
            None,
        ) == DISP_CHANGE_SUCCESSFUL
    }
}

pub(crate) fn get_display_settings(display_device_name: PCWSTR) -> Option<DEVMODEW> {
    let mut dev_mode = DEVMODEW {
        dmSize: std::mem::size_of::<DEVMODEW>().try_into().unwrap(),
        dmDriverExtra: 0,
        ..DEVMODEW::default()
    };

    // SAFETY: API call
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
        ..DISPLAY_DEVICEW::default()
    };

    // SAFETY: API call
    unsafe {
        EnumDisplayDevicesW(None, index, &mut dm_info, EDD_GET_DEVICE_INTERFACE_NAME).as_bool()
    }
    .then_some(dm_info)
}

pub(crate) fn get_display_x_y_position(
    display_device_name: PCWSTR,
) -> Option<((u32, u32), (i32, i32))> {
    let mut dev_mode = DEVMODEW {
        dmSize: size_of::<DEVMODEW>().try_into().unwrap(),
        ..DEVMODEW::default()
    };

    unsafe {
        EnumDisplaySettingsW(display_device_name, ENUM_CURRENT_SETTINGS, &mut dev_mode)
            .as_bool()
            .then_some({
                (
                    (dev_mode.dmPelsWidth, dev_mode.dmPelsHeight),
                    (
                        dev_mode.Anonymous1.Anonymous2.dmPosition.x,
                        dev_mode.Anonymous1.Anonymous2.dmPosition.y,
                    ),
                )
            })
    }
}
