use crate::*;
use std;

pub unsafe fn stbiw__writefv(mut s: *mut stbi__write_context, fmt: &str, args: &[i32]) {
    let mut argIndex = 0;

    for c in fmt.chars() {
        if c == ' ' {
        } else if c == '1' {
            let mut x: u8 = (args[argIndex] & 0xff) as u8;
            ((*s).func)((*s).context, ((&mut x) as *mut u8), 1);
            argIndex += 1;
        } else if c == '2' {
            let mut x: i32 = args[argIndex];
            let mut b: [u8; 2] = [0; 2];
            b[(0) as usize] = (((x) & 0xff) as u8);
            b[(1) as usize] = (((x >> 8) & 0xff) as u8);
            ((*s).func)((*s).context, ((b.as_mut_ptr()) as *mut u8), 2);
            argIndex += 1;
        } else if c == '4' {
            let mut x: u32 = args[argIndex] as u32;
            let mut b: [u8; 4] = [0; 4];
            b[(0) as usize] = (((x) & ((0xff) as u32)) as u8);
            b[(1) as usize] = (((x >> 8) & ((0xff) as u32)) as u8);
            b[(2) as usize] = (((x >> 16) & ((0xff) as u32)) as u8);
            b[(3) as usize] = (((x >> 24) & ((0xff) as u32)) as u8);
            ((*s).func)((*s).context, ((b.as_mut_ptr()) as *mut u8), 4);
            argIndex += 1;
        } else {
            return;
        }
    }
}

pub unsafe fn stbiw__writef(mut s: *mut stbi__write_context, mut fmt: &str, args: &[i32]) {
    stbiw__writefv(s, fmt, args);
}

pub unsafe fn stbiw__outfile(
    mut s: *mut stbi__write_context,
    mut rgb_dir: i32,
    mut vdir: i32,
    mut x: i32,
    mut y: i32,
    mut comp: i32,
    mut expand_mono: i32,
    mut data: *mut u8,
    mut alpha: i32,
    mut pad: i32,
    mut fmt: &str,
    args: &[i32],
) -> i32 {
    if y < 0 || x < 0 {
        return ((0) as i32);
    } else {
        stbiw__writefv(s, fmt, args);
        stbiw__write_pixels(s, rgb_dir, vdir, x, y, comp, data, alpha, pad, expand_mono);
        return ((1) as i32);
    }
}
