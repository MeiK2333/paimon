extern crate regex;

use libc::c_void;
use std::fs;

struct Maps {
    pub r: bool,
    pub w: bool,
    pub x: bool,
    pub start: *mut c_void,
    pub end: *mut c_void,
    pub offset: i64,
    pub pathname: String,
}

pub fn hide_library(name: &str) {
    debug!("hide {}", name);
    let text = fs::read_to_string("/proc/self/maps").unwrap();
    let re = regex::Regex::new(r"(?m)^([0-9a-zA-Z]{8})-([0-9a-zA-Z]{8})\s([rwxps\-]{4})\s([0-9a-zA-Z]{8})\s[0-9a-zA-Z]{2}:[0-9a-zA-Z]{2}\s\d{1,10}\s+(.{0,120})$").unwrap();
    for line in text.lines() {
        let cap = match re.captures(&line) {
            Some(val) => val,
            None => {
                warn!("{}", line);
                continue;
            }
        };
        if cap[5].find(name) == None {
            continue;
        }
        let maps = Maps {
            r: cap[3].find("r") != None,
            w: cap[3].find("w") != None,
            x: cap[3].find("x") != None,
            start: usize::from_str_radix(&cap[1], 16).unwrap() as *mut c_void,
            end: usize::from_str_radix(&cap[2], 16).unwrap() as *mut c_void,
            offset: cap[4].parse().unwrap(),
            pathname: cap[5].to_string(),
        };
        debug!(
            "{}, {}, {}, {}, {}",
            &cap[1], &cap[2], &cap[3], &cap[4], &cap[5]
        );
        unsafe {
            do_hide(&maps);
        }
    }
}

unsafe fn do_hide(maps: &Maps) {
    let length = maps.end as usize - maps.start as usize;
    let backup_address = libc::mmap(
        std::ptr::null_mut(),
        length,
        libc::PROT_READ | libc::PROT_WRITE,
        libc::MAP_ANONYMOUS | libc::MAP_PRIVATE,
        -1,
        0,
    );
    if !maps.r {
        debug!("mprotect +r");
        libc::mprotect(maps.start, length, get_prot(&maps) | libc::PROT_READ);
    }
    debug!("memcpy -> backup");
    libc::memcpy(backup_address, maps.start, length);

    // munmap original
    debug!("munmap original");
    debug!("start: {:?}, end: {:?}, length: {}", maps.start, maps.end, length);
    libc::munmap(maps.start, length);

    // restore
    debug!("mmap original");
    libc::mmap(
        maps.start,
        length,
        get_prot(&maps),
        libc::MAP_ANONYMOUS | libc::MAP_PRIVATE,
        -1,
        0,
    );
    debug!("mprotect +w");
    libc::mprotect(maps.start, length, get_prot(&maps) | libc::PROT_WRITE);
    debug!("memcpy -> original");
    libc::memcpy(maps.start, backup_address, length);
    if !maps.w {
        debug!("mprotect -w");
        libc::mprotect(maps.start, length, get_prot(&maps));
    }
    debug!("hide success");
}

fn get_prot(maps: &Maps) -> i32 {
    let mut prot = 0;
    if maps.r {
        prot |= libc::PROT_READ;
    }
    if maps.w {
        prot |= libc::PROT_WRITE;
    }
    if maps.x {
        prot |= libc::PROT_EXEC;
    }
    return prot;
}
