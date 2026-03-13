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
pub const BUF_BYTES: u64 = 1024; 
pub const fn req1024(nr: u64) -> u64 { ioc(IOC_READ | IOC_WRITE, TOUCH_MAGIC, nr, BUF_BYTES) }

pub const REQ_SET_CUR:  u64 = req1024(CMD_SET_CUR);
pub const REQ_GET_CUR:  u64 = req1024(CMD_GET_CUR);
pub const REQ_GET_DEF:  u64 = req1024(CMD_GET_DEF);
pub const REQ_GET_MIN:  u64 = req1024(CMD_GET_MIN);
pub const REQ_GET_MAX:  u64 = req1024(CMD_GET_MAX);
pub const REQ_GET_MODE: u64 = req1024(CMD_GET_MODE);
pub const REQ_RESET:    u64 = req1024(CMD_RESET);
pub const REQ_SET_LONG: u64 = req1024(CMD_SET_LONG);

pub const SZ_BUF1024: usize = 1024;

pub const V3_SELECT_TOUCH_ID: u64 = ioc(IOC_NONE,            TOUCH_MAGIC, 3,   0);
pub const V3_COMMON_DATA:     u64 = ioc(IOC_READ | IOC_WRITE, TOUCH_MAGIC, 0, 520);
pub const V3_HARDWARE_PARAM:  u64 = ioc(IOC_READ,             TOUCH_MAGIC, 1, 214);

pub const SZ_V3_BUF: usize = 520;

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u64)]
#[allow(dead_code)]
pub enum Cmd {
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
    #[inline] pub fn nr(self) -> u64      { self as u64 }
    #[inline] pub fn req(self) -> u64     { req1024(self.nr()) }
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
        "mode-num"                      => Ok(21),
        _ => Err(format!(
            "unknown mode '{}' — use an integer (0-21) or: \
             game active up-threshold tolerance aim tap expert edge \
             orientation rate fod aod resist-rf idle-time doubletap \
             grip fod-icon nonui debug-level power-status pen mode-num", s
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
        20 => "pen",             21 => "mode-num",
        _  => "?",
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
    (21, "mode-num"),
];

const DEV_NODE: &str = "/dev/xiaomi-touch\0";

#[allow(non_camel_case_types)]
pub mod sys {
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
pub fn errno() -> i32 {
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

#[inline] pub fn pu16(b: &mut [u8], o: usize, v: u16) { b[o] = v as u8; b[o+1] = (v>>8) as u8; }
#[inline] pub fn pi32(b: &mut [u8], o: usize, v: i32) { b[o..o+4].copy_from_slice(&v.to_le_bytes()); }
#[inline] pub fn gi32(b: &[u8],     o: usize) -> i32  { i32::from_le_bytes([b[o],b[o+1],b[o+2],b[o+3]]) }

pub trait TouchIoctlProtocol {
    fn set(&self, fd: i32, touch_id: u32, mode: u16, value: i32) -> Result<(), String>;
    fn get(&self, fd: i32, touch_id: u32, mode: u16, cmd: Cmd) -> Result<i32, String>;
    fn get_mode_all(&self, fd: i32, touch_id: u32, mode: u16) -> Result<[i32; 6], String>;
    fn reset(&self, fd: i32, touch_id: u32, mode: u16) -> Result<(), String>;
    fn set_long(&self, fd: i32, touch_id: u32, mode: u16, values: &[i32]) -> Result<(), String>;
    fn print_specific_ioctl_codes(&self);
}

struct Device<'a, P: TouchIoctlProtocol> { fd: i32, touch_id: u32, protocol: &'a P }

impl<'a, P: TouchIoctlProtocol> Device<'a, P> {
    fn open(touch_id: u32, protocol: &'a P) -> Result<Self, String> {
        Ok(Device { fd: open_dev()?, touch_id, protocol })
    }
    
    fn set(&self, mode: u16, value: i32) -> Result<(), String> {
        self.protocol.set(self.fd, self.touch_id, mode, value)
    }

    fn get(&self, mode: u16, cmd: Cmd) -> Result<i32, String> {
        self.protocol.get(self.fd, self.touch_id, mode, cmd)
    }

    fn get_mode_all(&self, mode: u16) -> Result<[i32; 6], String> {
        self.protocol.get_mode_all(self.fd, self.touch_id, mode)
    }

    fn reset(&self, mode: u16) -> Result<(), String> {
        self.protocol.reset(self.fd, self.touch_id, mode)
    }

    fn set_long(&self, mode: u16, values: &[i32]) -> Result<(), String> {
        self.protocol.set_long(self.fd, self.touch_id, mode, values)
    }
}

impl<'a, P: TouchIoctlProtocol> Drop for Device<'a, P> {
    fn drop(&mut self) { unsafe { sys::close(self.fd) }; }
}

// ─── Added Features (sysfs, daemon, status) ───────────────────────────────────

const SYSFS_NODES: &[&str] = &[
    "/proc/touchpanel/double_tap",
    "/sys/class/touch/touch_dev/gesture_control",
    "/sys/devices/platform/soc/soc:touch/gesture_control",
];

const SETTING_KEYS: &[&str] = &[
    "secure oplus_customize_gesture_double_touch",
    "secure double_tap_to_wake",
    "system double_touch",
    "system double_tap_to_wake",
];

fn sysfs_write(value: &str) {
    for &node in SYSFS_NODES {
        if let Ok(mut fd) = std::fs::File::options().write(true).open(node) {
            use std::io::Write;
            let _ = fd.write_all(value.as_bytes());
        }
    }
}

fn apply_with_sysfs<P: TouchIoctlProtocol>(dev: &Device<P>, mode: u16, value: i32) -> Result<(), String> {
    dev.set(mode, value)?;
    if mode == 14 {
        sysfs_write(if value > 0 { "1" } else { "0" });
    }
    Ok(())
}

fn get_setting() -> i32 {
    for &key in SETTING_KEYS {
        let cmd = format!("settings get {}", key);
        if let Ok(out) = std::process::Command::new("sh").arg("-c").arg(&cmd).output() {
            let s = String::from_utf8_lossy(&out.stdout);
            if s.contains("1") { return 1; }
            if s.contains("0") { return 0; }
        }
    }
    -1
}

fn resolve_value(mode: u16, raw: i32) -> i32 {
    if raw == 1 && mode >= 2 && mode <= 5 {
        5
    } else {
        raw
    }
}

fn run_daemon<P: TouchIoctlProtocol>(dev: &Device<P>, mode: u16) {
    println!("daemon active: mode {}", mode);
    let mut last = -1;
    loop {
        let cur = get_setting();
        if cur != -1 && cur != last {
            println!("sync: {}", cur);
            let val = resolve_value(mode, cur);
            if let Err(e) = apply_with_sysfs(dev, mode, val) {
                eprintln!("sync error: {}", e);
            }
            last = cur;
        }
        std::thread::sleep(std::time::Duration::from_secs(2));
    }
}

fn print_status() {
    println!("hw support:");
    let dev_ok = std::path::Path::new("/dev/xiaomi-touch").exists();
    println!("  /dev/xiaomi-touch: {}", if dev_ok { "ok" } else { "not found" });
    for &node in SYSFS_NODES {
        if std::path::Path::new(node).exists() {
            println!("  node: {}", node);
        }
    }
}

// ─── CLI ──────────────────────────────────────────────────────────────────────

fn usage(detect_binary: &str) {
    eprintln!(
r#"{bin} — /dev/xiaomi-touch ioctl controller

Usage:
  {bin} [OPTIONS] <subcommand> [args...]

Options:
  --touch-id N      Panel index (default: 0)
  -h / --help       Show this help

Subcommands:
  set      <mode> <value>        Set current value
  get      <mode>                Get current value
  def      <mode>                Get default value
  min      <mode>                Get minimum value
  max      <mode>                Get maximum value
  mode-val <mode>                Get full mode array (getModeAll)
  reset    <mode>                Reset to default
  long     <mode> <v0> [v1...]   Set multiple values (SET_LONG_VALUE)
  dump                           Read cur/def/min/max for all 22 modes
  status                         Check hardware availability
  daemon   <mode>                Sync with settings (polling)
  --e      <mode>                Enable/set value (uses resolve_value)
  --d      <mode>                Disable/reset
  ioctl-codes                    Print computed ioctl request numbers

Named modes (or raw integer 0-21):
  game  active  up-threshold  tolerance  aim  tap  expert  edge
  orientation  rate  fod  aod  resist-rf  idle-time  doubletap
  grip  fod-icon  nonui  debug-level  power-status  pen  mode-num

Examples:
  {bin} set game 1
  {bin} get doubletap
  {bin} mode-val game          # prints: cur= def= min= max= ext0= ext1=
  {bin} dump"#,
      bin=detect_binary
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

pub fn run_cli(protocol: impl TouchIoctlProtocol) {
    let args: Vec<String> = env::args().collect();
    let bin_name = args.get(0).map(|s| s.as_str()).unwrap_or("xiaomi-touch");
    let mut touch_id: u32 = 0;
    let mut i = 1usize;

    while i < args.len() {
        match args[i].as_str() {
            "--touch-id"    => {
                i += 1;
                touch_id = args.get(i)
                    .and_then(|s| s.parse().ok())
                    .unwrap_or_else(|| die("--touch-id needs a non-negative integer".into()));
                i += 1;
            }
            "--help" | "-h" => { usage(bin_name); return; }
            _               => break,
        }
    }

    let sub = match args.get(i) {
        Some(s) => { let s = s.clone(); i += 1; s }
        None    => { usage(bin_name); std::process::exit(1); }
    };

    match sub.as_str() {
        "ioctl-codes" => { protocol.print_specific_ioctl_codes(); return; }
        "status" => { print_status(); return; }
        _ => {}
    }

    eprintln!("driver={}  touch_id={}", bin_name, touch_id);

    let dev = Device::open(touch_id, &protocol).unwrap_or_else(|e| die(e));

    match sub.as_str() {
        "set" => {
            let mode  = req_mode(&args, i);
            let value = req_i32(&args, i+1, "value");
            apply_with_sysfs(&dev, mode, value).unwrap_or_else(|e| die(e));
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
        "daemon" => {
            let mode = if args.len() > i { req_mode(&args, i) } else { 14 };
            run_daemon(&dev, mode);
        }
        "--e" => {
            let mode = req_mode(&args, i);
            let val = resolve_value(mode, 1);
            apply_with_sysfs(&dev, mode, val).unwrap_or_else(|e| die(e));
            println!("set {}({}) -> {}", mode_name(mode), mode, val);
        }
        "--d" => {
            let mode = req_mode(&args, i);
            let val = 0;
            apply_with_sysfs(&dev, mode, val).unwrap_or_else(|e| die(e));
            println!("set {}({}) -> {}", mode_name(mode), mode, val);
        }
        other => {
            eprintln!("unknown subcommand '{}'\n", other);
            usage(bin_name);
            std::process::exit(1);
        }
    }
}

// ─── Gamemode Daemon (shared) ─────────────────────────────────────────────────

pub const GAMEMODE_CONFIG_PATH: &str = "/data/oofcontrol/gamemode.txt";
pub const GAMEMODE_DEFAULT_POLL_MS: u64 = 500;
pub const MODE_GRIP_LONG_ID: u16 = 15;
pub const MODE_GAME_MODE_ID: u16 = 0;

/// Parsed representation of /data/oofcontrol/gamemode.txt
#[derive(Debug, Clone)]
pub struct GamemodeConfig {
    /// (mode_id, value) pairs for SET_CUR_VALUE
    pub modes: Vec<(u16, i32)>,
    /// Optional 96-element grip array for SET_LONG_VALUE on mode 15
    pub grip: Option<Vec<i32>>,
}

pub fn parse_gamemode_config(content: &str) -> Result<GamemodeConfig, String> {
    let mut modes = Vec::new();
    let mut grip: Option<Vec<i32>> = None;

    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') { continue; }

        let mut parts = line.splitn(2, ' ');
        let mode_str = parts.next().unwrap_or("").trim();
        let val_str  = parts.next().unwrap_or("").trim();

        let mode: u16 = mode_str.parse()
            .map_err(|_| format!("bad mode id '{}'", mode_str))?;

        if mode == MODE_GRIP_LONG_ID {
            let vals: Result<Vec<i32>, _> = val_str.split(',')
                .map(|s| s.trim().parse::<i32>())
                .collect();
            grip = Some(vals.map_err(|_| format!("bad grip values: '{}'", val_str))?);
        } else {
            let value: i32 = val_str.parse()
                .map_err(|_| format!("bad value '{}' for mode {}", val_str, mode))?;
            modes.push((mode, value));
        }
    }

    Ok(GamemodeConfig { modes, grip })
}

/// Apply a parsed config to the device via the given protocol.
pub fn apply_gamemode_config<P: TouchIoctlProtocol>(
    protocol: &P,
    fd: i32,
    touch_id: u32,
    cfg: &GamemodeConfig,
) {
    // Game-mode OFF: just reset
    if let Some(&(_, val)) = cfg.modes.iter().find(|&&(m, _)| m == MODE_GAME_MODE_ID) {
        if val == 0 {
            eprintln!("[gamemode-daemon] game-mode OFF -> resetting");
            if let Err(e) = protocol.reset(fd, touch_id, MODE_GAME_MODE_ID) {
                eprintln!("[gamemode-daemon] reset error: {}", e);
            }
            return;
        }
    }

    // Apply grip zone before enabling game mode (kernel requires this order)
    if let Some(ref g) = cfg.grip {
        eprintln!("[gamemode-daemon] applying grip zone ({} values)", g.len());
        if let Err(e) = protocol.set_long(fd, touch_id, MODE_GRIP_LONG_ID, g) {
            eprintln!("[gamemode-daemon] grip error: {}", e);
        }
    }

    // Apply all mode params in order
    for &(mode, value) in &cfg.modes {
        eprintln!("[gamemode-daemon] set mode={} value={}", mode, value);
        if let Err(e) = protocol.set(fd, touch_id, mode, value) {
            eprintln!("[gamemode-daemon] set error (mode={}): {}", mode, e);
        }
    }
}

/// Main polling loop — call from each variant's main().
pub fn run_gamemode_watcher<P: TouchIoctlProtocol>(
    protocol: &P,
    touch_id: u32,
    config_path: &str,
    poll_ms: u64,
) {
    use std::time::Duration;
    use std::thread;
    use std::fs;

    let poll_dur = Duration::from_millis(poll_ms);
    let mut last_mtime: Option<std::time::SystemTime> = None;
    let mut last_content: Option<String> = None;

    eprintln!("[gamemode-daemon] watching '{}' every {}ms (touch_id={})", config_path, poll_ms, touch_id);

    loop {
        let current_mtime = fs::metadata(config_path).ok().and_then(|m| m.modified().ok());

        let changed = match (current_mtime, last_mtime) {
            (Some(c), Some(l)) => c != l,
            (Some(_), None)    => true,
            _                  => false,
        };

        if changed {
            last_mtime = current_mtime;

            match fs::read_to_string(config_path) {
                Err(e) => eprintln!("[gamemode-daemon] read error: {}", e),
                Ok(content) => {
                    if last_content.as_deref() == Some(&content) {
                        thread::sleep(poll_dur);
                        continue;
                    }
                    last_content = Some(content.clone());
                    eprintln!("[gamemode-daemon] config changed, parsing...");

                    match parse_gamemode_config(&content) {
                        Err(e) => eprintln!("[gamemode-daemon] parse error: {}", e),
                        Ok(cfg) => {
                            let dev_node = "/dev/xiaomi-touch\0";
                            let fd = unsafe { sys::open(dev_node.as_ptr(), 2 /* O_RDWR */) };
                            if fd < 0 {
                                eprintln!("[gamemode-daemon] open error errno={}", errno());
                            } else {
                                apply_gamemode_config(protocol, fd, touch_id, &cfg);
                                unsafe { sys::close(fd); }
                                eprintln!("[gamemode-daemon] applied OK");
                            }
                        }
                    }
                }
            }
        }

        thread::sleep(poll_dur);
    }
}

/// Parse common daemon CLI flags. Returns (touch_id, config_path, poll_ms).
pub fn parse_daemon_args(default_config: &str) -> (u32, String, u64) {
    let args: Vec<String> = std::env::args().collect();
    let (bin, default_cfg) = (args[0].clone(), default_config.to_string());
    let mut touch_id: u32  = 0;
    let mut config_path    = default_cfg.clone();
    let mut poll_ms: u64   = GAMEMODE_DEFAULT_POLL_MS;
    let mut i = 1;

    while i < args.len() {
        match args[i].as_str() {
            "--touch-id" => {
                i += 1;
                touch_id = args.get(i).and_then(|s| s.parse().ok())
                    .unwrap_or_else(|| { eprintln!("--touch-id needs an integer"); std::process::exit(1) });
            }
            "--file" => {
                i += 1;
                config_path = args.get(i).cloned()
                    .unwrap_or_else(|| { eprintln!("--file needs a path"); std::process::exit(1) });
            }
            "--poll-ms" => {
                i += 1;
                poll_ms = args.get(i).and_then(|s| s.parse().ok())
                    .unwrap_or_else(|| { eprintln!("--poll-ms needs an integer"); std::process::exit(1) });
            }
            "--help" | "-h" => {
                eprintln!("{bin} -- OOFControl gamemode file watcher\n\nOptions:\n  --touch-id N   Panel index (default: 0)\n  --file PATH    Config file (default: {default_cfg})\n  --poll-ms N    Poll interval ms (default: {GAMEMODE_DEFAULT_POLL_MS})\n  -h/--help      Show this help");
                std::process::exit(0);
            }
            other => { eprintln!("unknown argument '{other}'"); std::process::exit(1); }
        }
        i += 1;
    }
    (touch_id, config_path, poll_ms)
}
