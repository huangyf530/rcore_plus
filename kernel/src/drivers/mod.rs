use alloc::prelude::*;
use core::any::Any;

use lazy_static::lazy_static;
use smoltcp::wire::EthernetAddress;

use crate::sync::SpinNoIrqLock;

mod device_tree;
pub mod bus;
pub mod net;
pub mod block;
mod gpu;
mod input;

pub enum DeviceType {
    Net,
    Gpu,
    Input,
    Block
}

pub trait Driver : Send + AsAny {
    // if interrupt belongs to this driver, handle it and return true
    // return false otherwise
    fn try_handle_interrupt(&mut self) -> bool;

    // return the correspondent device type, see DeviceType
    fn device_type(&self) -> DeviceType;
}

pub trait NetDriver: Driver {
    // get mac address for this device
    fn get_mac(&self) -> EthernetAddress;

    // get interface name for this device
    fn get_ifname(&self) -> String;
}

// little hack, see https://users.rust-lang.org/t/how-to-downcast-from-a-trait-any-to-a-struct/11219/3
pub trait AsAny {
    fn as_any(&self) -> &Any;
}

impl<T: Any> AsAny for T {
    fn as_any(&self) -> &Any { self }
}

lazy_static! {
    pub static ref DRIVERS: SpinNoIrqLock<Vec<Box<Driver>>> = SpinNoIrqLock::new(Vec::new());
}

lazy_static! {
    pub static ref NET_DRIVERS: SpinNoIrqLock<Vec<Box<NetDriver>>> = SpinNoIrqLock::new(Vec::new());
}

pub fn init(dtb: usize) {
    device_tree::init(dtb);
}