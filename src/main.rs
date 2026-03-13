#![allow(non_upper_case_globals)]

use std::env;

// ─── Linux _IOC bit layout ────────────────────────────────────────────────────
const IOC_NRSHIFT:   u64 = 0;
const IOC_TYPESHIFT: u64 = 8;
const IOC_SIZESHIFT: u64 = 16;
const IOC_DIRSHIFT:  u64 = 30;
const IOC_NONE:  u64 = 0;
const IOC_WRITE: u64 = 1;
const IOC_READ:  u64 = 2;

const fn ioc(dir: u64, ty: u64, nr: u64, size: u64) -> u64 {
    (dir  << IOC_DIRSHIFT)
  | (ty   << IOC_TYPESHIFT)
  | (nr   << IOC_NRSHIFT)
  | (size << IOC_SIZESHIFT)
}

const TOUCH_MAGIC: u64 = b'T' as u64;

const CMD_SET_CUR:  u64 = 0;
const CMD_GET_CUR:  u64 = 1;
const CMD_GET_DEF:  u64 = 2;
const CMD_GET_MIN:  u64 = 3;
const CMD_GET_MAX:  u64 = 4;
const CMD_GET_MODE: u64 = 5;
const CMD_RESET:    u64 = 6;
const CMD_SET_LONG: u64 = 7;
const BUF_BYTES: u64 = 1024; 
const fn req1024(nr: u64) -> u64 { ioc(IOC_READ | IOC_WRITE, TOUCH_MAGIC, nr, BUF_BYTES) }

const REQ_SET_CUR:  u64 = req1024(CMD_SET_CUR);
const REQ_GET_CUR:  u64 = req1024(CMD_GET_CUR);
const REQ_GET_DEF:  u64 = req1024(CMD_GET_DEF);
const REQ_GET_MIN:  u64 = req1024(CMD_GET_MIN);
const REQ_GET_MAX:  u64 = req1024(CMD_GET_MAX);
const REQ_GET_MODE: u64 = req1024(CMD_GET_MODE);
const REQ_RESET:    u64 = req1024(CMD_RESET);
const REQ_SET_LONG: u64 = req1024(CMD_SET_LONG);

const SZ_BUF1024: usize = 1024;

const V3_SELECT_TOUCH_ID: u64 = ioc(IOC_NONE,            TOUCH_MAGIC, 3,   0);
const V3_COMMON_DATA:     u64 = ioc(IOC_READ | IOC_WRITE, TOUCH_MAGIC, 0, 520);
const V3_HARDWARE_PARAM:  u64 = ioc(IOC_READ,             TOUCH_MAGIC, 1, 214);

const SZ_V3_BUF: usize = 520;

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u64)]
#[allow(dead_code)]
enum Cmd {
    SetCurValue  = CMD_SET_CUR,
    GetCurValue  = CMD_GET_CUR,
    GetDefValue  = CMD_GET_DEF,
    GetMinValue  = CMD_GET_MIN,
    GetMaxValue  = CMD_GET_MAX,
    GetModeValue = CMD_GET_MODE,
    ResetMode    = CMD_RESET,
    SetLongValue = CMD_SET_LONG,
}

impl Cmd {
    #[inline] fn nr(self) -> u64      { self as u64 }
    #[inline] fn req(self) -> u64     { req1024(self.nr()) }
}

fn parse_mode(s: &str) -> Result<u16, String> {
    if let Ok(n) = s.parse::<u16>() { return Ok(n); }
    match s.to_lowercase().replace('_', "-").as_str() {
        "game" | "game-mode"            => Ok(0),
        "active"                        => Ok(1),
        "up-threshold" | "upthreshold"  => Ok(2),
        "tolerance"                     => Ok(3),
        "aim" | "aim-sensitivity"       => Ok(4),
        "tap" | "tap-stability"         => Ok(5),
        "expert"                        => Ok(6),
        "edge" | "edge-filter"          => Ok(7),
        "orientation"                   => Ok(8),
        "rate" | "report-rate"          => Ok(9),
        "fod"                           => Ok(10),
        "aod"                           => Ok(11),
        "resist-rf"                     => Ok(12),
        "idle-time"                     => Ok(13),
        "doubletap" | "dt2w"            => Ok(14),
        "grip" | "grip-mode"            => Ok(15),
        "fod-icon"                      => Ok(16),
        "nonui"                         => Ok(17),
        "debug-level"                   => Ok(18),
        "power-status"                  => Ok(19),
        "pen"                           => Ok(20),
        _ => Err(format!(
            "unknown mode '{}' — use an integer (0-20) or: \
             game active up-threshold tolerance aim tap expert edge \
             orientation rate fod aod resist-rf idle-time doubletap \
             grip fod-icon nonui debug-level power-status pen", s
        )),
    }
}

fn mode_name(n: u16) -> &'static str {
    match n {
        0  => "game-mode",       1  => "active",
        2  => "up-threshold",    3  => "tolerance",
        4  => "aim-sensitivity", 5  => "tap-stability",
        6  => "expert",          7  => "edge-filter",
        8  => "orientation",     9  => "report-rate",
        10 => "fod",             11 => "aod",
        12 => "resist-rf",       13 => "idle-time",
        14 => "doubletap",       15 => "grip-mode",
        16 => "fod-icon",        17 => "nonui",
        18 => "debug-level",     19 => "power-status",
        20 => "pen",             _  => "?",
    }
}

const ALL_MODES: &[(u16, &str)] = &[
    (0,  "game-mode"),      (1,  "active"),         (2,  "up-threshold"),
    (3,  "tolerance"),      (4,  "aim-sensitivity"), (5,  "tap-stability"),
    (6,  "expert"),         (7,  "edge-filter"),     (8,  "orientation"),
    (9,  "report-rate"),    (10, "fod"),             (11, "aod"),
    (12, "resist-rf"),      (13, "idle-time"),       (14, "doubletap"),
    (15, "grip-mode"),      (16, "fod-icon"),        (17, "nonui"),
    (18, "debug-level"),    (19, "power-status"),    (20, "pen"),
];

const DEV_NODE: &str = "/dev/xiaomi-touch\0";

#[allow(non_camel_case_types)]
mod sys {
    pub type c_int   = i32;
    pub type c_ulong = u64;
    extern "C" {
        pub fn open(path: *const u8, flags: c_int, ...) -> c_int;
        pub fn close(fd: c_int) -> c_int;
        pub fn ioctl(fd: c_int, req: c_ulong, ...) -> c_int;
        #[cfg(not(target_os = "android"))]
        pub fn __errno_location() -> *mut c_int;
        #[cfg(target_os = "android")]
        pub fn __errno() -> *mut c_int;
    }
}

#[inline]
fn errno() -> i32 {
    unsafe {
        #[cfg(not(target_os = "android"))]  { *sys::__errno_location() }
        #[cfg(target_os = "android")]       { *sys::__errno() }
    }
}

fn open_dev() -> Result<i32, String> {
    let fd = unsafe { sys::open(DEV_NODE.as_ptr(), 2 /* O_RDWR */) };
    if fd < 0 {
        Err(format!("open(/dev/xiaomi-touch) errno={} — need root? Xiaomi device?", errno()))
    } else {
        Ok(fd)
    }
}

#[inline] fn pu16(b: &mut [u8], o: usize, v: u16) { b[o] = v as u8; b[o+1] = (v>>8) as u8; }
#[inline] fn pi32(b: &mut [u8], o: usize, v: i32) { b[o..o+4].copy_from_slice(&v.to_le_bytes()); }
#[inline] fn gi32(b: &[u8],     o: usize) -> i32  { i32::from_le_bytes([b[o],b[o+1],b[o+2],b[o+3]]) }

fn v1_ioctl(fd: i32, req: u64, mode: u16, extra: &[i32]) -> Result<[u8; SZ_BUF1024], String> {
    let mut buf = [0u8; SZ_BUF1024];
    pi32(&mut buf, 0, mode as i32);
    for (i, &v) in extra.iter().take(255).enumerate() {
        pi32(&mut buf, 4 + i * 4, v);
    }
    let rc = unsafe { sys::ioctl(fd, req, buf.as_mut_ptr()) };
    if rc < 0 { Err(format!("V1 ioctl req=0x{:08x} mode={} errno={}", req, mode, errno())) }
    else       { Ok(buf) }
}

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

#[derive(Debug, Clone, Copy, PartialEq)]
enum Version { V1, V2, V3 }

impl Version {
    fn detect(touch_id: u32) -> Self {
        let fd = match open_dev() { Ok(fd) => fd, Err(_) => return Version::V2 };
        let rc = unsafe { sys::ioctl(fd, V3_SELECT_TOUCH_ID, touch_id as sys::c_ulong) };
        unsafe { sys::close(fd) };
        if rc == 0 { Version::V3 } else { Version::V2 }
    }
}

struct Device { fd: i32, touch_id: u32, version: Version }

impl Device {
    fn open(touch_id: u32, version: Version) -> Result<Self, String> {
        Ok(Device { fd: open_dev()?, touch_id, version })
    }

    fn set(&self, mode: u16, value: i32) -> Result<(), String> {
        match self.version {
            Version::V1 => { v1_ioctl(self.fd, REQ_SET_CUR, mode, &[value])?; }
            Version::V2 => { v2_ioctl(self.fd, REQ_SET_CUR, self.touch_id as i32, mode, &[value])?; }
            Version::V3 => {
                v3_select(self.fd, self.touch_id)?;
                v3_common_data(self.fd, self.touch_id as u8, Cmd::SetCurValue, mode, &[value])?;
            }
        }
        Ok(())
    }

    fn get(&self, mode: u16, cmd: Cmd) -> Result<i32, String> {
        match self.version {
            Version::V1 => {
                let buf = v1_ioctl(self.fd, cmd.req(), mode, &[])?;
                Ok(gi32(&buf, 0))
            }
            Version::V2 => {
                let buf = v2_ioctl(self.fd, cmd.req(), self.touch_id as i32, mode, &[])?;
                Ok(gi32(&buf, 0))
            }
            Version::V3 => {
                v3_select(self.fd, self.touch_id)?;
                let buf = v3_common_data(self.fd, self.touch_id as u8, cmd, mode, &[0])?;
                Ok(gi32(&buf, 8))
            }
        }
    }

    fn get_mode_all(&self, mode: u16) -> Result<[i32; 6], String> {
        let mut out = [0i32; 6];
        match self.version {
            Version::V1 => {
                let buf = v1_ioctl(self.fd, REQ_GET_MODE, mode, &[])?;
                for i in 0..6 { out[i] = gi32(&buf, i * 4); }
            }
            Version::V2 => {
                let buf = v2_ioctl(self.fd, REQ_GET_MODE, self.touch_id as i32, mode, &[])?;
                for i in 0..6 { out[i] = gi32(&buf, i * 4); }
            }
            Version::V3 => {
                v3_select(self.fd, self.touch_id)?;
                let buf = v3_common_data(self.fd, self.touch_id as u8, Cmd::GetModeValue, mode, &[0])?;
                for i in 0..6 { out[i] = gi32(&buf, 8 + i * 4); }
            }
        }
        Ok(out)
    }

    fn reset(&self, mode: u16) -> Result<(), String> {
        match self.version {
            Version::V1 => { v1_ioctl(self.fd, REQ_RESET, mode, &[])?; }
            Version::V2 => { v2_ioctl(self.fd, REQ_RESET, self.touch_id as i32, mode, &[])?; }
            Version::V3 => {
                v3_select(self.fd, self.touch_id)?;
                v3_common_data(self.fd, self.touch_id as u8, Cmd::ResetMode, mode, &[0])?;
            }
        }
        Ok(())
    }

    fn set_long(&self, mode: u16, values: &[i32]) -> Result<(), String> {
        match self.version {
            Version::V1 => {
                let count = values.len().min(254);
                let mut extra = vec![count as i32];
                extra.extend_from_slice(&values[..count]);
                v1_ioctl(self.fd, REQ_SET_LONG, mode, &extra)?;
            }
            Version::V2 => {
                let count = values.len().min(253);
                let mut extra = vec![count as i32];
                extra.extend_from_slice(&values[..count]);
                v2_ioctl(self.fd, REQ_SET_LONG, self.touch_id as i32, mode, &extra)?;
            }
            Version::V3 => {
                v3_select(self.fd, self.touch_id)?;
                v3_common_data(self.fd, self.touch_id as u8, Cmd::SetLongValue, mode, values)?;
            }
        }
        Ok(())
    }
}

impl Drop for Device {
    fn drop(&mut self) { unsafe { sys::close(self.fd) }; }
}

// ─── CLI ──────────────────────────────────────────────────────────────────────

fn usage() {
    eprintln!(
r#"xiaomi-touch — /dev/xiaomi-touch ioctl controller

Usage:
  xiaomi-touch [OPTIONS] <subcommand> [args...]

Options:
  --v1              Force V1 (sm8250 era — buf[0]=mode, no touch_id)
  --v2              Force V2 (marble era  — buf[0]=touch_id)
  --v3              Force V3 (peridot era — common_data_t + SELECT_TOUCH_ID)
  --touch-id N      Panel index (default: 0; not used in V1)
  --detect          Detect version + print ioctl codes, then exit
  -h / --help       Show this help

  Note: V1 and V2 share identical ioctl codes. Auto-detection always
  returns V2 or V3. Use --v1 explicitly on sm8250 devices (Mi 10, Poco X3...).

Subcommands:
  set      <mode> <value>        Set current value
  get      <mode>                Get current value
  def      <mode>                Get default value
  min      <mode>                Get minimum value
  max      <mode>                Get maximum value
  mode-val <mode>                Get full mode array (getModeAll)
  reset    <mode>                Reset to default
  long     <mode> <v0> [v1...]   Set multiple values (SET_LONG_VALUE)
  dump                           Read cur/def/min/max for all 21 modes
  ioctl-codes                    Print computed ioctl request numbers

Named modes (or raw integer 0-20):
  game  active  up-threshold  tolerance  aim  tap  expert  edge
  orientation  rate  fod  aod  resist-rf  idle-time  doubletap
  grip  fod-icon  nonui  debug-level  power-status  pen

Examples:
  xiaomi-touch set game 1
  xiaomi-touch --v1 set game 1        # sm8250 (no touch_id in buffer)
  xiaomi-touch --v3 --touch-id 1 set rate 1
  xiaomi-touch get doubletap
  xiaomi-touch mode-val game          # prints: cur= def= min= max= ext0= ext1=
  xiaomi-touch dump
  xiaomi-touch --detect"#
    );
}

fn print_ioctl_codes() {
    println!("=== V1 & V2  (identical IOC codes — buffer layout differs) ===");
    println!("  SET_CUR   (0)  0x{:016x}  _IOC(RW,'T',0,1024)", REQ_SET_CUR);
    println!("  GET_CUR   (1)  0x{:016x}  _IOC(RW,'T',1,1024)", REQ_GET_CUR);
    println!("  GET_DEF   (2)  0x{:016x}  _IOC(RW,'T',2,1024)", REQ_GET_DEF);
    println!("  GET_MIN   (3)  0x{:016x}  _IOC(RW,'T',3,1024)", REQ_GET_MIN);
    println!("  GET_MAX   (4)  0x{:016x}  _IOC(RW,'T',4,1024)", REQ_GET_MAX);
    println!("  GET_MODE  (5)  0x{:016x}  _IOC(RW,'T',5,1024)  [getModeAll]", REQ_GET_MODE);
    println!("  RESET     (6)  0x{:016x}  _IOC(RW,'T',6,1024)", REQ_RESET);
    println!("  SET_LONG  (7)  0x{:016x}  _IOC(RW,'T',7,1024)", REQ_SET_LONG);
    println!();
    println!("  V1 layout: [mode | value/count | values...]       (no touch_id)");
    println!("  V2 layout: [touch_id | mode | value/count | ...]");
    println!();
    println!("=== V3  (distinct codes) ===");
    println!("  SELECT_TOUCH_ID  0x{:016x}  _IOC(NONE,'T',3,  0)  [immediate ulong]", V3_SELECT_TOUCH_ID);
    println!("  COMMON_DATA      0x{:016x}  _IOC(RW,  'T',0,520)  [520-byte struct]", V3_COMMON_DATA);
    println!("  HARDWARE_PARAM   0x{:016x}  _IOC(R,   'T',1,214)", V3_HARDWARE_PARAM);
}

fn die(e: String) -> ! { eprintln!("error: {}", e); std::process::exit(1); }

fn req_mode(args: &[String], idx: usize) -> u16 {
    match args.get(idx) {
        Some(s) => parse_mode(s).unwrap_or_else(|e| die(e)),
        None    => die("missing <mode> argument".into()),
    }
}

fn req_i32(args: &[String], idx: usize, label: &str) -> i32 {
    match args.get(idx) {
        Some(s) => s.parse::<i32>().unwrap_or_else(|_| die(format!("<{}> must be integer, got '{}'", label, s))),
        None    => die(format!("missing <{}> argument", label)),
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut version_override: Option<Version> = None;
    let mut touch_id: u32 = 0;
    let mut detect_only = false;
    let mut i = 1usize;

    while i < args.len() {
        match args[i].as_str() {
            "--v1"          => { version_override = Some(Version::V1); i += 1; }
            "--v2"          => { version_override = Some(Version::V2); i += 1; }
            "--v3"          => { version_override = Some(Version::V3); i += 1; }
            "--touch-id"    => {
                i += 1;
                touch_id = args.get(i)
                    .and_then(|s| s.parse().ok())
                    .unwrap_or_else(|| die("--touch-id needs a non-negative integer".into()));
                i += 1;
            }
            "--detect"      => { detect_only = true; i += 1; }
            "--help" | "-h" => { usage(); return; }
            _               => break,
        }
    }

    if detect_only {
        let v = Version::detect(touch_id);
        println!("Detected: {:?}", v);
        println!("(V1 vs V2 cannot be auto-detected — use --v1 on sm8250 devices)");
        print_ioctl_codes();
        return;
    }

    let sub = match args.get(i) {
        Some(s) => { let s = s.clone(); i += 1; s }
        None    => { usage(); std::process::exit(1); }
    };

    match sub.as_str() {
        "ioctl-codes" => { print_ioctl_codes(); return; }
        "detect" => {
            let v = Version::detect(touch_id);
            println!("Detected: {:?}", v);
            println!("(V1 vs V2 cannot be auto-detected — use --v1 on sm8250 devices)");
            print_ioctl_codes();
            return;
        }
        _ => {}
    }

    let version = version_override.unwrap_or_else(|| Version::detect(touch_id));
    eprintln!("driver={:?}  touch_id={}", version, touch_id);

    let dev = Device::open(touch_id, version).unwrap_or_else(|e| die(e));

    match sub.as_str() {
        "set" => {
            let mode  = req_mode(&args, i);
            let value = req_i32(&args, i+1, "value");
            dev.set(mode, value).unwrap_or_else(|e| die(e));
            println!("OK  mode={}({})  value={}", mode, mode_name(mode), value);
        }
        "get"  => { println!("{}", dev.get(req_mode(&args,i), Cmd::GetCurValue).unwrap_or_else(|e| die(e))); }
        "def"  => { println!("{}", dev.get(req_mode(&args,i), Cmd::GetDefValue).unwrap_or_else(|e| die(e))); }
        "min"  => { println!("{}", dev.get(req_mode(&args,i), Cmd::GetMinValue).unwrap_or_else(|e| die(e))); }
        "max"  => { println!("{}", dev.get(req_mode(&args,i), Cmd::GetMaxValue).unwrap_or_else(|e| die(e))); }
        "mode-val" => {
            let mode = req_mode(&args, i);
            let v = dev.get_mode_all(mode).unwrap_or_else(|e| die(e));
            println!("cur={}  def={}  min={}  max={}  ext0={}  ext1={}",
                     v[0], v[1], v[2], v[3], v[4], v[5]);
        }
        "reset" => {
            let mode = req_mode(&args, i);
            dev.reset(mode).unwrap_or_else(|e| die(e));
            println!("OK  reset mode={}({})", mode, mode_name(mode));
        }
        "long" => {
            let mode = req_mode(&args, i);
            i += 1;
            let vals: Vec<i32> = args[i..].iter()
                .map(|s| s.parse::<i32>().unwrap_or_else(|_| die(format!("expected integer, got '{}'", s))))
                .collect();
            if vals.is_empty() { die("'long' requires at least one value".into()); }
            dev.set_long(mode, &vals).unwrap_or_else(|e| die(e));
            println!("OK  set-long mode={}({})  count={}", mode, mode_name(mode), vals.len());
        }
        "dump" => {
            println!("{:<18} {:>4}  {:>8}  {:>8}  {:>8}  {:>8}",
                     "mode", "id", "current", "default", "min", "max");
            println!("{}", "─".repeat(62));
            for &(m, name) in ALL_MODES {
                let cur = dev.get(m, Cmd::GetCurValue).map(|v| v.to_string()).unwrap_or("-".into());
                let def = dev.get(m, Cmd::GetDefValue).map(|v| v.to_string()).unwrap_or("-".into());
                let mn  = dev.get(m, Cmd::GetMinValue).map(|v| v.to_string()).unwrap_or("-".into());
                let mx  = dev.get(m, Cmd::GetMaxValue).map(|v| v.to_string()).unwrap_or("-".into());
                println!("{:<18} {:>4}  {:>8}  {:>8}  {:>8}  {:>8}", name, m, cur, def, mn, mx);
            }
        }
        other => {
            eprintln!("unknown subcommand '{}'\n", other);
            usage();
            std::process::exit(1);
        }
    }
}
