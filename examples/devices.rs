use coremidi_sys::{ItemCount, MIDIDeviceRef, MIDIGetDevice, MIDIGetExternalDevice, MIDIGetNumberOfDevices, MIDIGetNumberOfExternalDevices};
use coremidi::{Object, Properties};

fn main() {
    let nb_devices = get_number_of_devices();
    println!("Number of devices: {}", nb_devices);

    for i in 0..nb_devices {
        let device = get_device(i);
        println!("Devices[{}]", i);

        println!(".. Unique ID: {:?}", device.unique_id());
        println!(".. Device ID: {:?}", device.get_property::<i32>(&Properties::device_id()));
        println!(".. Name: {:?}", device.name());
        println!(".. Display Name: {:?}", device.display_name());
        println!(".. Manufacturer: {:?}", device.get_property::<String>(&Properties::manufacturer()));
        println!(".. Model: {:?}", device.get_property::<String>(&Properties::model()));
        // println!(".. Supports MMC: {:?}", device.get_property::<bool>(&Properties::supports_mmc()));
        // println!(".. Supports General MIDI: {:?}", device.get_property::<bool>(&Properties::supports_general_midi()));
        // println!(".. Supports Show Control: {:?}", device.get_property::<bool>(&Properties::supports_show_control()));
        // println!(".. Max Sysex Speed: {:?}", device.get_property::<i32>(&Properties::max_sysex_speed()));
        // println!(".. Editor App: {:?}", device.get_property::<String>(&Properties::driver_device_editor_app()));
        // println!(".. Pan Disrupts Stereo: {:?}", device.get_property::<bool>(&Properties::pan_disrupts_stereo()));
        // println!(".. Protocol ID: {:?}", device.get_property::<i32>(&Properties::protocol_id()));
        // println!(".. Transmits MTC: {:?}", device.get_property::<bool>(&Properties::transmits_mtc()));
        // println!(".. Receives MTC: {:?}", device.get_property::<bool>(&Properties::receives_mtc()));
        // println!(".. Transmits Clock: {:?}", device.get_property::<bool>(&Properties::transmits_clock()));
        // println!(".. Receives Clock: {:?}", device.get_property::<bool>(&Properties::receives_clock()));
        // println!(".. Advence Schedule Time MuSec: {:?}", device.get_property::<i32>(&Properties::advance_schedule_time_musec()));
    }
}

fn get_number_of_devices() -> usize {
    unsafe {
        MIDIGetNumberOfDevices() as usize
    }
}

fn get_device(device_num: usize) -> Object {
    unsafe {
        Object(MIDIGetDevice(device_num as ItemCount))
    }
}

fn get_number_of_external_devices() -> usize {
    unsafe {
        MIDIGetNumberOfExternalDevices() as usize
    }
}

fn get_external_device(device_num: usize) -> Object {
    unsafe {
        Object(MIDIGetExternalDevice(device_num as ItemCount))
    }
}
