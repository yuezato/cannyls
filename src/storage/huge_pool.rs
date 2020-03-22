use libc::c_void;
use std::ptr;

#[derive(Debug)]
pub struct HugePool {
    pool: *mut libc::c_void,
    aligned: *mut libc::c_void,
    len: usize,
}

unsafe impl Sync for HugePool {}
unsafe impl Send for HugePool {}

impl Drop for HugePool {
    fn drop(&mut self) {
        unsafe { libc::munmap(self.pool, self.len) };
    }
}

impl HugePool {
    pub fn to_slice(&self, len: usize) -> &[u8] {
        unsafe { std::slice::from_raw_parts(self.aligned as *const u8, len) }
    }

    pub fn as_mutref(&self) -> *mut u8 {
        self.aligned as *mut u8
    }

    pub fn new(len: usize) -> Option<HugePool> {
        if len % 4096 != 0 {
            return None;
        }
        let len = len + 4096;

        unsafe {
            let ptr = libc::mmap(
                ptr::null_mut(),
                len,
                libc::PROT_READ | libc::PROT_WRITE,
                libc::MAP_PRIVATE | libc::MAP_HUGETLB | libc::MAP_ANONYMOUS,
                0,
                0,
            );

            if ptr == libc::MAP_FAILED {
                None
            } else if ptr as usize % 4096 == 0 {
                Some(HugePool {
                    pool: ptr,
                    aligned: ptr,
                    len,
                })
            } else {
                Some(HugePool {
                    pool: ptr,
                    aligned: (4096 * ((ptr as usize + 4095) / 4096)) as *mut c_void,
                    len,
                })
            }
        }
    }
}
