//! gamemode-daemon-v3 — watches /data/oofcontrol/gamemode.txt and applies
//! touch settings via V3 ioctl protocol (sm8650 / peridot: SELECT_TOUCH_ID + common_data_t).

use xiaomi_touch_core::{
    Cmd, TouchIoctlProtocol, errno, sys,
    V3_SELECT_TOUCH_ID, V3_COMMON_DATA,
    pu16, pi32, gi32, SZ_V3_BUF,
    GAMEMODE_CONFIG_PATH, run_gamemode_watcher, parse_daemon_args,
};

struct ProtocolV3;

fn v3_select(fd: i32, tid: u32) -> Result<(), String> {
    let rc = unsafe { sys::ioctl(fd, V3_SELECT_TOUCH_ID, tid as sys::c_ulong) };
    if rc < 0 { Err(format!("V3 SELECT_TOUCH_ID errno={}", errno())) } else { Ok(()) }
}

fn v3_common(fd: i32, tid: u8, cmd: Cmd, mode: u16, values: &[i32]) -> Result<[u8; SZ_V3_BUF], String> {
    let mut buf = [0u8; SZ_V3_BUF];
    buf[0] = tid;
    buf[1] = cmd.nr() as u8;
    pu16(&mut buf, 2, mode);
    pu16(&mut buf, 4, values.len().min(128) as u16);
    for (i, &v) in values.iter().take(128).enumerate() { pi32(&mut buf, 8 + i * 4, v); }
    let rc = unsafe { sys::ioctl(fd, V3_COMMON_DATA, buf.as_mut_ptr()) };
    if rc < 0 { Err(format!("V3 COMMON_DATA cmd={:?} mode={} errno={}", cmd, mode, errno())) }
    else       { Ok(buf) }
}

impl TouchIoctlProtocol for ProtocolV3 {
    fn set(&self, fd: i32, tid: u32, mode: u16, value: i32) -> Result<(), String> {
        v3_select(fd, tid)?;
        v3_common(fd, tid as u8, Cmd::SetCurValue, mode, &[value]).map(|_| ())
    }
    fn get(&self, fd: i32, tid: u32, mode: u16, cmd: Cmd) -> Result<i32, String> {
        v3_select(fd, tid)?;
        let buf = v3_common(fd, tid as u8, cmd, mode, &[0])?; Ok(gi32(&buf, 8))
    }
    fn get_mode_all(&self, fd: i32, tid: u32, mode: u16) -> Result<[i32; 6], String> {
        v3_select(fd, tid)?;
        let buf = v3_common(fd, tid as u8, Cmd::GetModeValue, mode, &[0])?;
        let mut out = [0i32; 6]; for i in 0..6 { out[i] = gi32(&buf, 8 + i * 4); } Ok(out)
    }
    fn reset(&self, fd: i32, tid: u32, mode: u16) -> Result<(), String> {
        v3_select(fd, tid)?;
        v3_common(fd, tid as u8, Cmd::ResetMode, mode, &[0]).map(|_| ())
    }
    fn set_long(&self, fd: i32, tid: u32, mode: u16, values: &[i32]) -> Result<(), String> {
        v3_select(fd, tid)?;
        v3_common(fd, tid as u8, Cmd::SetLongValue, mode, values).map(|_| ())
    }
    fn print_specific_ioctl_codes(&self) {}
}

fn main() {
    let (touch_id, config_path, poll_ms) = parse_daemon_args(GAMEMODE_CONFIG_PATH);
    run_gamemode_watcher(&ProtocolV3, touch_id, &config_path, poll_ms);
}
