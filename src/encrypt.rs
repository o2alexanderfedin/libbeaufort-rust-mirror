use super::*;
use crate::tableau::beaufort_tableau;

pub(crate) extern "C" fn ssize(str: *const i8) -> u64 {
    let mut size: u64 = 0 as u64;
    while '\u{0}' as i32 != unsafe { *str.add(size as usize) } as i32 {
        {
            let __p = &mut size;
            let __t = *__p;
            *__p = (*__p).wrapping_add(1);
            __t
        };
    }
    return size;
}

#[allow(unused_doc_comments)]
pub(crate) extern "C" fn beaufort_encrypt(src: *const i8, key: *const i8,
    mut mat: *mut *mut i8) -> *mut i8 {
    let mut enc: *mut i8 = 0 as *mut () as *mut i8;
    let mut ch: i8 = 0 as i8;
    let mut k: i8 = 0 as i8;
    let mut ksize: u64 = 0 as u64;
    let mut size: u64 = 0 as u64;
    let mut len: u64 = 0 as u64;
    let mut rsize: u64 = 0 as u64;
    let mut i: i32 = 0;
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut j: i32 = 0;
    let mut needed: i32 = 1;
    if 0 as *mut () == mat as *mut () {
        mat =
            beaufort_tableau(c"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz".as_ptr()
                        as *mut i8 as *const i8);
        if 0 as *mut () == mat as *mut () { return 0 as *mut () as *mut i8; }
    }
    ksize = ssize(key);
    len = ssize(src);
    rsize = ssize(unsafe { *mat.offset(0 as isize) } as *const i8);
    enc =
        unsafe {
                malloc((core::mem::size_of::<i8>() as
                                u64).wrapping_mul(len).wrapping_add(1 as u64))
            } as *mut i8;
    if 0 as *mut () == enc as *mut () { return 0 as *mut () as *mut i8; }
    {
        '__b5: loop {
            if !({ ch = unsafe { *src.offset(i as isize) } as i8; ch } != 0) {
                break '__b5;
            }
            '__c5: loop {

                /// find column with char
                (needed = 1);
                {
                    { x = 0; y = 0 };
                    '__b6: loop {
                        if !((x as u64) < rsize) { break '__b6; }
                        '__c6: loop {
                            if ch as i32 ==
                                    unsafe {
                                            *unsafe { (*mat.offset(y as isize)).offset(x as isize) }
                                        } as i32 {
                                needed = 1;
                                break '__b6;
                            } else { needed = 0; }
                            break '__c6;
                        }
                        { let __p = &mut x; *__p += 1; *__p };
                    }
                }
                if 0 == needed {
                    unsafe {
                        *enc.add({
                                            let __p = &mut size;
                                            let __t = *__p;
                                            *__p = (*__p).wrapping_add(1);
                                            __t
                                        } as usize) = ch
                    };
                    break '__c5;
                }
                k =
                    unsafe {
                            *key.add(({
                                                    let __p = &mut j;
                                                    let __t = *__p;
                                                    *__p += 1;
                                                    __t
                                                } as u64 % ksize) as usize)
                        } as i8;
                {
                    y = 0;
                    '__b7: loop {
                        if !((y as u64) < rsize) { break '__b7; }
                        '__c7: loop {
                            if k as i32 ==
                                    unsafe {
                                            *unsafe { (*mat.offset(y as isize)).offset(x as isize) }
                                        } as i32 {
                                needed = 1;
                                break '__b7;
                            } else { needed = 0; }
                            break '__c7;
                        }
                        { let __p = &mut y; *__p += 1; *__p };
                    }
                }
                if 0 == needed {
                    unsafe {
                        *enc.add({
                                            let __p = &mut size;
                                            let __t = *__p;
                                            *__p = (*__p).wrapping_add(1);
                                            __t
                                        } as usize) = ch
                    };
                    { let __p = &mut j; let __t = *__p; *__p -= 1; __t };
                    break '__c5;
                }
                unsafe {
                    *enc.add({
                                        let __p = &mut size;
                                        let __t = *__p;
                                        *__p = (*__p).wrapping_add(1);
                                        __t
                                    } as usize) =
                        unsafe {
                            *unsafe { (*mat.offset(y as isize)).offset(0 as isize) }
                        }
                };
                break '__c5;
            }
            { let __p = &mut i; *__p += 1; *__p };
        }
    }
    unsafe { *enc.add(size as usize) = '\u{0}' as i32 as i8 };
    return enc;
}
