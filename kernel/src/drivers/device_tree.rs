use core::slice;

use device_tree::{DeviceTree, Node};

use super::bus::virtio_mmio::virtio_probe;

const DEVICE_TREE_MAGIC: u32 = 0xd00dfeed;

fn walk_dt_node(dt: &Node) {
    if let Ok(compatible) = dt.prop_str("compatible") {
        // TODO: query this from table
        if compatible == "virtio,mmio" {
            virtio_probe(dt);
        }
    }
    for child in dt.children.iter() {
        walk_dt_node(child);
    }
}

struct DtbHeader {
    magic: u32,
    size: u32,
}

pub fn init(dtb: usize) {
    let header = unsafe {&*(dtb as *const DtbHeader)};
    let magic = u32::from_be(header.magic);
    if magic == DEVICE_TREE_MAGIC {
        let size = u32::from_be(header.size);
        let dtb_data = unsafe { slice::from_raw_parts(dtb as *const u8, size as usize) };
        if let Ok(dt) = DeviceTree::load(dtb_data) {
            walk_dt_node(&dt.root);
        }
    }
}
