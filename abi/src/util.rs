// Copyright 2024 Democratized Data Foundation
//
// Use of this software is governed by the Business Source License
// included in the file licenses/BSL.txt.
//
// As of the Change Date specified in that file, in accordance with
// the Business Source License, use of this software will be governed
// by the Apache License, Version 2.0, included in the file
// licenses/APL.txt.

use libc::c_char;
use std::ffi::CStr;

// const_char_to_str converts a const char pointer to a string slice.
pub unsafe fn const_char_to_str<'a>(json_str: *const c_char) -> &'a str {
    let c_str: &CStr = CStr::from_ptr(json_str);
    let buf: &[u8] = c_str.to_bytes();
    std::str::from_utf8_unchecked(buf)
}

// StringBuffer is a simple struct that stores a string buffer and its capacity.
#[repr(C)]
pub struct StringBuffer {
    data: *mut c_char,
    cap: usize,
}

impl StringBuffer {
    const DEFAULT_CAP: usize = 256;

    // Fills the buffer with the given input string.
    // If the input string is longer than the buffer, the buffer is truncated.
    pub unsafe fn fill(&mut self, input: &str) {
        let input_bytes = input.as_bytes();
        let length_to_copy = std::cmp::min(self.cap - 1, input_bytes.len());
        std::ptr::copy_nonoverlapping(input_bytes.as_ptr(), self.data as *mut u8, length_to_copy);
        *self.data.add(length_to_copy) = 0; // Null-terminate
    }

    // Returns a new OutputString with the given capacity.
    pub fn new_with_cap(cap: usize) -> StringBuffer {
        let data = unsafe { libc::malloc(cap) as *mut c_char };
        unsafe { data.write_bytes(0, cap) };
        StringBuffer { cap, data }
    }

    // Returns a new OutputString with a default capacity of 256 bytes.
    pub fn new() -> StringBuffer {
        return Self::new_with_cap(Self::DEFAULT_CAP);
    }

    // Returns the capacity of the buffer.
    pub fn cap(&self) -> usize {
        self.cap
    }

    fn free(&mut self) {
        unsafe { libc::free(self.data as *mut libc::c_void) };
    }

    // Returns a pointer to the underlying buffer.
    pub fn data(&self) -> *const c_char {
        self.data
    }
}

impl Drop for StringBuffer {
    fn drop(&mut self) {
        self.free();
    }
}
