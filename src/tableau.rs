use super::*;

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

pub(crate) extern "C" fn beaufort_tableau(alpha: *const i8) -> *mut *mut i8 {
    let size: u64 = ssize(alpha);
    let mut mat: *mut *mut i8 = 0 as *mut () as *mut *mut i8;
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut j: i32 = 0;
    mat = unsafe {
        calloc(
            size.wrapping_add(1 as u64),
            core::mem::size_of::<*mut i8>() as u64,
        )
    } as *mut *mut i8;
    if 0 as *mut () == mat as *mut () {
        return 0 as *mut () as *mut *mut i8;
    }
    {
        '__b9: loop {
            if !((y as u64) < size) {
                break '__b9;
            }
            '__c9: loop {
                unsafe {
                    *mat.offset(y as isize) =
                        unsafe { calloc(size, core::mem::size_of::<i8>() as u64) } as *mut i8
                };
                if 0 as *mut () == unsafe { *mat.offset(y as isize) } as *mut () {
                    return 0 as *mut () as *mut *mut i8;
                }
                {
                    {
                        x = 0;
                        j = size as i32
                    };
                    '__b10: loop {
                        if !((x as u64) < size) {
                            break '__b10;
                        }
                        '__c10: loop {
                            unsafe {
                                *unsafe { (*mat.offset(y as isize)).offset(x as isize) } =
                                    unsafe { *alpha.add(((j + y) as u64 % size) as usize) } as i8
                            };
                            break '__c10;
                        }
                        {
                            {
                                let __p = &mut x;
                                *__p += 1;
                                *__p
                            };
                            {
                                let __p = &mut j;
                                *__p -= 1;
                                *__p
                            }
                        };
                    }
                }
                unsafe {
                    *unsafe { (*mat.offset(y as isize)).offset(x as isize) } = '\u{0}' as i32 as i8
                };
                break '__c9;
            }
            {
                let __p = &mut y;
                *__p += 1;
                *__p
            };
        }
    }
    unsafe { *mat.offset(y as isize) = 0 as *mut () as *mut i8 };
    return mat;
}
