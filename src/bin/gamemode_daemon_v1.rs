//! gamemode-daemon-v1 — watches /data/oofcontrol/gamemode.txt and applies
//! touch settings via V1 ioctl protocol (sm8250-era, no touch_id in buffer).

use xiaomi_touch_core::{
    Cmd, TouchIoctlProtocol, errno, sys,
    REQ_SET_CUR, REQ_RESET, REQ_SET_LONG, REQ_GET_MODE,
    pi32, gi32, SZ_BUF1024,
    GAMEMODE_CONFIG_PATH, run_gamemode_watcher, parse_daemon_args,
};

struct ProtocolV1;

fn v1_ioctl(fd: i32, req: u64, mode: u16, extra: &[i32]) -> Result<[u8; SZ_BUF1024], String> {
    let mut buf = [0u8; SZ_BUF1024];
    pi32(&mut buf, 0, mode as i32);
    for (i, &v) in extra.iter().take(255).enumerate() { pi32(&mut buf, 4 + i * 4, v); }
    let rc = unsafe { sys::ioctl(fd, req, buf.as_mut_ptr()) };
    if rc < 0 { Err(format!("V1 ioctl req=0x{:08x} mode={} errno={}", req, mode, errno())) }
    else       { Ok(buf) }
}

impl TouchIoctlProtocol for ProtocolV1 {
    fn set(&self, fd: i32, _tid: u32, mode: u16, value: i32) -> Result<(), String> {
        v1_ioctl(fd, REQ_SET_CUR, mode, &[value]).map(|_| ())
    }
    fn get(&self, fd: i32, _tid: u32, mode: u16, cmd: Cmd) -> Result<i32, String> {
        let buf = v1_ioctl(fd, cmd.req(), mode, &[])?; Ok(gi32(&buf, 0))
    }
    fn get_mode_all(&self, fd: i32, _tid: u32, mode: u16) -> Result<[i32; 6], String> {
        let buf = v1_ioctl(fd, REQ_GET_MODE, mode, &[])?;
        let mut out = [0i32; 6]; for i in 0..6 { out[i] = gi32(&buf, i * 4); } Ok(out)
    }
    fn reset(&self, fd: i32, _tid: u32, mode: u16) -> Result<(), String> {
        v1_ioctl(fd, REQ_RESET, mode, &[]).map(|_| ())
    }
    fn set_long(&self, fd: i32, _tid: u32, mode: u16, values: &[i32]) -> Result<(), String> {
        let count = values.len().min(254);
        let mut extra = vec![count as i32];
        extra.extend_from_slice(&values[..count]);
        v1_ioctl(fd, REQ_SET_LONG, mode, &extra).map(|_| ())
    }
    fn print_specific_ioctl_codes(&self) {}
}

fn main() {
    let (touch_id, config_path, poll_ms) = parse_daemon_args(GAMEMODE_CONFIG_PATH);
    run_gamemode_watcher(&ProtocolV1, touch_id, &config_path, poll_ms);
}
