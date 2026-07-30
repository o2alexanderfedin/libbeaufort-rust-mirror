use super::*;
use crate::tableau::beaufort_tableau;

pub(crate) extern "C" fn ssize(str: *const i8) -> u64 {
    let mut size: u64 = 0 as u64;
    while '\u{0}' as i32 != unsafe { *str.add(size as usize) } as i32 {
        {
            let __old = size;
            size = size.wrapping_add(1);
            __old
        };
    }
    return size;
}

#[allow(unused_doc_comments)]
pub(crate) extern "C" fn beaufort_decrypt(
    src: *const i8,
    key: *const i8,
    mut mat: *mut *mut i8,
) -> *mut i8 {
    let mut dec: *mut i8 = 0 as *mut () as *mut i8;
    let mut ch: i8 = 0 as i8;
    let mut k: i8 = 0 as i8;
    let mut ksize: u64 = 0 as u64;
    let mut size: u64 = 0 as u64;
    let mut rsize: u64 = 0 as u64;
    let mut len: u64 = 0 as u64;
    let mut i: i32 = 0;
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut j: i32 = 0;
    let mut needed: i32 = 1;
    if 0 as *mut () == mat as *mut () {
        mat = beaufort_tableau(
            c"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz".as_ptr() as *mut i8
                as *const i8,
        );
        if 0 as *mut () == mat as *mut () {
            return 0 as *mut () as *mut i8;
        }
    }
    ksize = ssize(key);
    len = ssize(src);
    rsize = ssize(unsafe { *mat.offset(0 as isize) } as *const i8);
    dec = unsafe {
        malloc(
            (core::mem::size_of::<i8>() as u64)
                .wrapping_mul(len)
                .wrapping_add(1 as u64),
        )
    } as *mut i8;
    if 0 as *mut () == dec as *mut () {
        return 0 as *mut () as *mut i8;
    }
    {
        '__b1: loop {
            if !({
                ch = unsafe { *src.offset(i as isize) } as i8;
                ch
            } != 0)
            {
                break '__b1;
            }
            '__c1: loop {
                needed = 1;
                {
                    y = 0;
                    '__b2: loop {
                        if !((y as u64) < rsize) {
                            break '__b2;
                        }
                        '__c2: loop {
                            if ch as i32
                                == unsafe {
                                    *unsafe { (*mat.offset(y as isize)).offset(0 as isize) }
                                } as i32
                            {
                                needed = 1;
                                break '__b2;
                            } else {
                                needed = 0;
                            }
                            break '__c2;
                        }
                        y += 1;
                    }
                }
                if 0 == needed {
                    unsafe {
                        *dec.add({
                            let __old = size;
                            size = size.wrapping_add(1);
                            __old
                        } as usize) = ch
                    };
                    break '__c1;
                }

                /// determine char in `key'
                (k = unsafe {
                    *key.add(
                        ({
                            let __old = j;
                            j += 1;
                            __old
                        } as u64
                            % ksize) as usize,
                    )
                } as i8);
                {
                    x = 0;
                    '__b3: loop {
                        if !((x as u64) < rsize) {
                            break '__b3;
                        }
                        '__c3: loop {
                            if k as i32
                                == unsafe {
                                    *unsafe { (*mat.offset(y as isize)).offset(x as isize) }
                                } as i32
                            {
                                needed = 1;
                                break '__b3;
                            } else {
                                needed = 0;
                            }
                            break '__c3;
                        }
                        x += 1;
                    }
                }
                if 0 == needed {
                    unsafe {
                        *dec.add({
                            let __old = size;
                            size = size.wrapping_add(1);
                            __old
                        } as usize) = ch
                    };
                    j -= 1;
                    break '__c1;
                }
                unsafe {
                    *dec.add({
                        let __old = size;
                        size = size.wrapping_add(1);
                        __old
                    } as usize) =
                        unsafe { *unsafe { (*mat.offset(0 as isize)).offset(x as isize) } }
                };
                break '__c1;
            }
            i += 1;
        }
    }
    unsafe { *dec.add(size as usize) = '\u{0}' as i32 as i8 };
    return dec;
}
