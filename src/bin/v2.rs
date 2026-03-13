use xiaomi_touch_core::{Cmd, TouchIoctlProtocol, run_cli, errno, sys, REQ_SET_CUR, REQ_GET_MODE, REQ_RESET, REQ_SET_LONG, pi32, gi32, SZ_BUF1024};
use xiaomi_touch_core::{REQ_GET_CUR, REQ_GET_DEF, REQ_GET_MIN, REQ_GET_MAX};

struct ProtocolV2;

fn v2_ioctl(fd: i32, req: u64, touch_id: i32, mode: u16, extra: &[i32]) -> Result<[u8; SZ_BUF1024], String> {
    let mut buf = [0u8; SZ_BUF1024];
    pi32(&mut buf, 0, touch_id);
    pi32(&mut buf, 4, mode as i32);
    for (i, &v) in extra.iter().take(254).enumerate() {
        pi32(&mut buf, 8 + i * 4, v);
    }
    let rc = unsafe { sys::ioctl(fd, req, buf.as_mut_ptr()) };
    if rc < 0 { Err(format!("V2 ioctl req=0x{:08x} mode={} errno={}", req, mode, errno())) }
    else       { Ok(buf) }
}

impl TouchIoctlProtocol for ProtocolV2 {
    fn set(&self, fd: i32, touch_id: u32, mode: u16, value: i32) -> Result<(), String> {
        v2_ioctl(fd, REQ_SET_CUR, touch_id as i32, mode, &[value]).map(|_| ())
    }

    fn get(&self, fd: i32, touch_id: u32, mode: u16, cmd: Cmd) -> Result<i32, String> {
        let buf = v2_ioctl(fd, cmd.req(), touch_id as i32, mode, &[])?;
        Ok(gi32(&buf, 0))
    }

    fn get_mode_all(&self, fd: i32, touch_id: u32, mode: u16) -> Result<[i32; 6], String> {
        let buf = v2_ioctl(fd, REQ_GET_MODE, touch_id as i32, mode, &[])?;
        let mut out = [0i32; 6];
        for i in 0..6 { out[i] = gi32(&buf, i * 4); }
        Ok(out)
    }

    fn reset(&self, fd: i32, touch_id: u32, mode: u16) -> Result<(), String> {
        v2_ioctl(fd, REQ_RESET, touch_id as i32, mode, &[]).map(|_| ())
    }

    fn set_long(&self, fd: i32, touch_id: u32, mode: u16, values: &[i32]) -> Result<(), String> {
        let count = values.len().min(253);
        let mut extra = vec![count as i32];
        extra.extend_from_slice(&values[..count]);
        v2_ioctl(fd, REQ_SET_LONG, touch_id as i32, mode, &extra).map(|_| ())
    }

    fn print_specific_ioctl_codes(&self) {
        println!("=== V2 (marble era  — buffer layout: [touch_id | mode | value/count | ...]) ===");
        println!("  SET_CUR   (0)  0x{:016x}  _IOC(RW,'T',0,1024)", REQ_SET_CUR);
        println!("  GET_CUR   (1)  0x{:016x}  _IOC(RW,'T',1,1024)", REQ_GET_CUR);
        println!("  GET_DEF   (2)  0x{:016x}  _IOC(RW,'T',2,1024)", REQ_GET_DEF);
        println!("  GET_MIN   (3)  0x{:016x}  _IOC(RW,'T',3,1024)", REQ_GET_MIN);
        println!("  GET_MAX   (4)  0x{:016x}  _IOC(RW,'T',4,1024)", REQ_GET_MAX);
        println!("  GET_MODE  (5)  0x{:016x}  _IOC(RW,'T',5,1024)  [getModeAll]", REQ_GET_MODE);
        println!("  RESET     (6)  0x{:016x}  _IOC(RW,'T',6,1024)", REQ_RESET);
        println!("  SET_LONG  (7)  0x{:016x}  _IOC(RW,'T',7,1024)", REQ_SET_LONG);
    }
}

fn main() {
    run_cli(ProtocolV2);
}
