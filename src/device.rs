const IF_NAME_SIZE: usize = 20;
const NET_DEVICE_ADDR_LEN: usize = 16;

#[rustfmt::skip]
#[derive(Debug, Clone)]
#[repr(u16)]
pub enum NetDeviceType {
    Dummy    = 0x0000,
    Loopback = 0x0001,
    Ethernet = 0x0002,
}

#[rustfmt::skip]
#[derive(Debug, Clone)]
#[repr(u16)]
pub enum NetDeviceFlag {
    Up        = 0x0001,
    Loopback  = 0x0010,
    Broadcast = 0x0020,
    P2P       = 0x0040,
    NeedArp   = 0x0100,
}

impl NetDeviceFlag {
    fn is_up(flag: &u16) -> bool {
        *flag & (Self::Up as u16) == 1
    }
}

#[derive(Debug, Clone)]
#[repr(C)]
pub struct NetDevice {
    name: [u8; IF_NAME_SIZE],
    ty: NetDeviceType,
    mtu: u16,
    flags: u16,
    hlen: u16,
    alen: u16,
    addr: [u8; NET_DEVICE_ADDR_LEN],
    broadcast: [u8; NET_DEVICE_ADDR_LEN],
}

#[derive(Debug, Clone)]
#[repr(C)]
pub struct NetDeviceArray {
    devices: Vec<NetDevice>,
}

impl NetDeviceArray {
    pub fn new() -> Self {
        Self {
            devices: Vec::new(),
        }
    }
    pub fn register(&mut self, dev: &NetDevice) {
        let _dev = dev.with_name(format!("net{}", self.devices.len()));
        self.devices.push(_dev);
    }
    pub fn run(&mut self) -> Result<(), String> {
        if self.devices.is_empty() {
            return Err("Not registered device".to_string());
        }
        for dev in self.devices.iter_mut() {
            match dev.open() {
                Ok(_) => {
                    println!("Open '{}'", std::str::from_utf8(&dev.name).unwrap());
                }
                Err(e) => {
                    return Err(format!(
                        "Failed to open '{}': {}",
                        std::str::from_utf8(&dev.name).unwrap(),
                        e
                    ));
                }
            }
        }
        println!("Success to open {} devices.", self.devices.len());
        Ok(())
    }
    pub fn shutdown(&mut self) -> Result<(), String> {
        if self.devices.is_empty() {
            return Err("Not registered device".to_string());
        }
        for dev in self.devices.iter_mut() {
            match dev.close() {
                Ok(_) => {
                    println!("Close '{}'", std::str::from_utf8(&dev.name).unwrap());
                }
                Err(e) => {
                    return Err(format!(
                        "Failed to close '{}': {}",
                        std::str::from_utf8(&dev.name).unwrap(),
                        e
                    ));
                }
            }
        }
        println!("Success to close {} devices.", self.devices.len());
        Ok(())
    }
    pub fn output(
        &self,
        index: usize,
        ty: NetDeviceType,
        data: &[u8],
        dst: &[u8],
    ) -> Result<(), String> {
        self.devices[index].output(ty, data, dst)
    }
}

impl NetDevice {
    pub fn new(
        ty: NetDeviceType,
        mtu: u16,
        flags: u16,
        hlen: u16,
        alen: u16,
        addr: &[u8],
        broadcast: &[u8],
    ) -> Self {
        let mut addr_tmp = [0u8; NET_DEVICE_ADDR_LEN];
        addr_tmp[..addr.len()].copy_from_slice(addr);
        let mut broadcast_tmp = [0u8; NET_DEVICE_ADDR_LEN];
        broadcast_tmp[..broadcast.len()].copy_from_slice(broadcast);
        Self {
            name: [0u8; IF_NAME_SIZE],
            ty,
            mtu,
            flags,
            hlen,
            alen,
            addr: addr_tmp,
            broadcast: broadcast_tmp,
        }
    }
    fn with_name(&self, name: impl AsRef<str>) -> Self {
        let mut name_tmp = [0u8; IF_NAME_SIZE];
        name_tmp[..name.as_ref().len()].copy_from_slice(name.as_ref().as_bytes());
        Self {
            name: name_tmp,
            ty: self.ty.clone(),
            mtu: self.mtu,
            flags: self.flags,
            hlen: self.hlen,
            alen: self.alen,
            addr: self.addr,
            broadcast: self.broadcast,
        }
    }
    pub fn open(&mut self) -> Result<(), String> {
        if NetDeviceFlag::is_up(&self.flags) {
            Err(format!(
                "already opened: devive={}",
                std::str::from_utf8(&self.name).unwrap()
            ))
        } else {
            self.flags |= NetDeviceFlag::Up as u16;
            Ok(())
        }
    }
    pub fn close(&mut self) -> Result<(), String> {
        if !NetDeviceFlag::is_up(&self.flags) {
            Err(format!(
                "not opened: devive={}",
                std::str::from_utf8(&self.name).unwrap()
            ))
        } else {
            self.flags &= !(NetDeviceFlag::Up as u16);
            Ok(())
        }
    }
    pub fn output(&self, ty: NetDeviceType, data: &[u8], dst: &[u8]) -> Result<(), String> {
        println!("OutputData: {:?}", data);
        if !NetDeviceFlag::is_up(&self.flags) {
            return Err(format!(
                "not opened: dev={}",
                str::from_utf8(&self.name).unwrap()
            ));
        };
        if self.mtu < data.len() as u16 {
            return Err(format!(
                "too long: dev={} mtu={} len={}",
                str::from_utf8(&self.name).unwrap(),
                self.mtu,
                data.len(),
            ));
        }
        Ok(())
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     fn dummy_init() -> NetDevice {
//         NetDevice::new(NetDeviceType::Dummy, 128, 0, 0, 0, &[], &[])
//     }
//     #[test]
//     fn test_output() {
//         let mut devices = NetDeviceArray::new();
//         let dev = dummy_init();
//         devices.register(&dev);
//         let _ = devices.run();
//         dev.output(NetDeviceType::Dummy, data, dst)
//     }
// }
