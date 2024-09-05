use futures_lite::future::block_on;
use nusb::transfer::{ControlIn, ControlOut, ControlType, Recipient};

fn main() {
    env_logger::init();
    let di = nusb::list_devices()
        .unwrap()
        .find(|d| d.vendor_id() == 0x59e3 && d.product_id() == 0x0a23)
        .expect("device should be connected");

    println!("Device info: {di:?}");

    let device = di.open().unwrap();

    // Linux can make control transfers without claiming an interface
    #[cfg(any(target_os = "linux", target_os = "macos", target_os = "android"))]
    {
        let result = block_on(device.control_out(ControlOut {
            control_type: ControlType::Vendor,
            recipient: Recipient::Device,
            request: 0x81,
            value: 0x9999,
            index: 0x9999,
            data: &[1, 2, 3, 4],
        }));
        println!("{result:?}");

        let result = block_on(device.control_in(ControlIn {
            control_type: ControlType::Vendor,
            recipient: Recipient::Device,
            request: 0x81,
            value: 0x9999,
            index: 0x9999,
            length: 256,
        }));
        println!("{result:?}");
    }

    // but we also provide an API on the `Interface` to support Windows
    let interface = device.claim_interface(0).unwrap();

    let result = block_on(interface.control_out(ControlOut {
        control_type: ControlType::Vendor,
        recipient: Recipient::Device,
        request: 0x81,
        value: 0x9999,
        index: 0x9999,
        data: &[1, 2, 3, 4],
    }));
    println!("{result:?}");

    let result = block_on(interface.control_in(ControlIn {
        control_type: ControlType::Vendor,
        recipient: Recipient::Device,
        request: 0x81,
        value: 0x9999,
        index: 0x9999,
        length: 256,
    }));
    println!("{result:?}");
}
