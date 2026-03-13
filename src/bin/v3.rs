use xiaomi_touch_core::{Cmd, TouchIoctlProtocol, run_cli, errno, sys, V3_SELECT_TOUCH_ID, V3_COMMON_DATA, V3_HARDWARE_PARAM, pu16, pi32, gi32, SZ_V3_BUF};

struct ProtocolV3;

fn v3_select(fd: i32, touch_id: u32) -> Result<(), String> {
    let rc = unsafe { sys::ioctl(fd, V3_SELECT_TOUCH_ID, touch_id as sys::c_ulong) };
    if rc < 0 { Err(format!("V3 SELECT_TOUCH_ID errno={}", errno())) }
    else       { Ok(()) }
}

fn v3_common_data(fd: i32, touch_id: u8, cmd: Cmd, mode: u16, values: &[i32]) -> Result<[u8; SZ_V3_BUF], String> {
    let mut buf = [0u8; SZ_V3_BUF];
    buf[0] = touch_id;
    buf[1] = cmd.nr() as u8;
    pu16(&mut buf, 2, mode);
    pu16(&mut buf, 4, values.len().min(128) as u16);
    for (i, &v) in values.iter().take(128).enumerate() { pi32(&mut buf, 8 + i*4, v); }
    let rc = unsafe { sys::ioctl(fd, V3_COMMON_DATA, buf.as_mut_ptr()) };
    if rc < 0 { Err(format!("V3 COMMON_DATA cmd={:?} mode={} errno={}", cmd, mode, errno())) }
    else       { Ok(buf) }
}

impl TouchIoctlProtocol for ProtocolV3 {
    fn set(&self, fd: i32, touch_id: u32, mode: u16, value: i32) -> Result<(), String> {
        v3_select(fd, touch_id)?;
        v3_common_data(fd, touch_id as u8, Cmd::SetCurValue, mode, &[value]).map(|_| ())
    }

    fn get(&self, fd: i32, touch_id: u32, mode: u16, cmd: Cmd) -> Result<i32, String> {
        v3_select(fd, touch_id)?;
        let buf = v3_common_data(fd, touch_id as u8, cmd, mode, &[0])?;
        Ok(gi32(&buf, 8))
    }

    fn get_mode_all(&self, fd: i32, touch_id: u32, mode: u16) -> Result<[i32; 6], String> {
        v3_select(fd, touch_id)?;
        let buf = v3_common_data(fd, touch_id as u8, Cmd::GetModeValue, mode, &[0])?;
        let mut out = [0i32; 6];
        for i in 0..6 { out[i] = gi32(&buf, 8 + i * 4); }
        Ok(out)
    }

    fn reset(&self, fd: i32, touch_id: u32, mode: u16) -> Result<(), String> {
        v3_select(fd, touch_id)?;
        v3_common_data(fd, touch_id as u8, Cmd::ResetMode, mode, &[0]).map(|_| ())
    }

    fn set_long(&self, fd: i32, touch_id: u32, mode: u16, values: &[i32]) -> Result<(), String> {
        v3_select(fd, touch_id)?;
        v3_common_data(fd, touch_id as u8, Cmd::SetLongValue, mode, values).map(|_| ())
    }

    fn print_specific_ioctl_codes(&self) {
        println!("=== V3 (peridot era — common_data_t + SELECT_TOUCH_ID) ===");
        println!("  SELECT_TOUCH_ID  0x{:016x}  _IOC(NONE,'T',3,  0)  [immediate ulong]", V3_SELECT_TOUCH_ID);
        println!("  COMMON_DATA      0x{:016x}  _IOC(RW,  'T',0,520)  [520-byte struct]", V3_COMMON_DATA);
        println!("  HARDWARE_PARAM   0x{:016x}  _IOC(R,   'T',1,214)", V3_HARDWARE_PARAM);
    }
}

fn main() {
    run_cli(ProtocolV3);
}
