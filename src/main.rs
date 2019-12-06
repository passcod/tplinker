extern crate tplinker;

use tplinker::{
    devices::{Device, DeviceActions, Switch, LB110},
    discovery::discover,
};

fn pad(value: &str, padding: usize) -> String {
    let pad = " ".repeat(padding.saturating_sub(value.len()));
    format!("{}{}", value, pad)
}

fn is_on<T: Switch>(device: &T) {
    println!("{:?}", device.is_on());
}

fn main() {
    for (addr, data) in discover().unwrap() {
        let device = Device::from_data(addr, &data);
        let sysinfo = data.sysinfo();
        println!(
            "{}\t{}\t{}\t{}\t{}",
            addr,
            pad(&sysinfo.alias, 18),
            pad(&sysinfo.hw_type, 20),
            pad(&sysinfo.dev_name, 40),
            sysinfo.model,
        );
        match device {
            Device::HS100(device) => is_on(&device),
            Device::HS110(device) => is_on(&device),
            Device::LB110(device) => is_on(&device),
            _ => println!("{} not switchable", sysinfo.alias),
        }
    }

    let device = LB110::new("192.168.0.25:9999").unwrap();

    println!("{:?}", device.sysinfo().unwrap());
    println!("{:?}", device.is_on());
    device.switch_on().unwrap();
}