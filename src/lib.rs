pub mod darts {
    use std::{ffi::{c_void, CStr, CString}, ptr};

    use libdarts_sys as raw;

    pub struct DoubleArrayTrie {
        darts_t: raw::DartsT
    }

    pub struct Array {
        array: *const c_void,
    }

    impl Array {
        pub fn new() -> Array {
            Array { array: ptr::null_mut() }
        }
    }

    #[derive(Debug, PartialEq)]
    pub struct ResultPairType {
        pub value: i32,
        pub length: usize,
    }

    pub type Progress = dyn FnMut(usize, usize) -> i32;

    impl DoubleArrayTrie {
        pub fn new() -> DoubleArrayTrie {
            DoubleArrayTrie { darts_t: unsafe { raw::darts_new() } }
        }

        pub fn set_array(&mut self, array: &Array, size: usize) {
            unsafe { raw::darts_set_array(self.darts_t, array.array, size); }
        }

        pub fn array(&self) -> Array {
            Array { array: unsafe { raw::darts_array(self.darts_t) } }
        }

        pub fn clear(&self) {
            unsafe { raw::darts_clear(self.darts_t); }
        }

        pub fn unit_size(&self) -> usize {
            unsafe { raw::darts_unit_size(self.darts_t) }
        }

        pub fn size(&self) -> usize {
            unsafe { raw::darts_size(self.darts_t) }
        }

        pub fn total_size(&self) -> usize {
            unsafe { raw::darts_total_size(self.darts_t) }
        }

        pub fn nonzero_size(&self) -> usize {
            unsafe { raw::darts_nonzero_size(self.darts_t) }
        }

        pub fn build(&self, num_keys: usize, keys: &Vec<String>, lengths: Option<&[usize]>, values: Option<&[i32]>, progress_func: Option<Box<Progress>>) -> Result<(), &str> {
            let keys = keys.iter()
                .map(|key| CString::new(key.as_bytes()).unwrap())
                .collect::<Vec<_>>();

            let mut c_keys: Vec<*const std::os::raw::c_char> = Vec::with_capacity(num_keys + 1);
            for key in &keys {
                c_keys.push(key.as_ptr() as *const std::os::raw::c_char);
            }
            c_keys.push(ptr::null());

            let c_lengths = match lengths {
                Some(lengths) => &lengths[0],
                None => ptr::null()
            };

            let c_values = match values {
                Some(values) => &values[0],
                None => ptr::null()
            };

            static mut STORED_PROGRESS: Option<Box<Progress>> = None;

            unsafe {
                STORED_PROGRESS = progress_func;
                let retval = raw::darts_build(self.darts_t, num_keys, &c_keys[0], c_lengths, c_values, Some(progress_callback));
                if retval != 0 {
                    let err = CStr::from_ptr(raw::darts_error(self.darts_t));
                    return Err(err.to_str().unwrap());
                }
            }

            unsafe extern "C" fn progress_callback(current: usize, totols: usize) -> i32 {
                match STORED_PROGRESS {
                    Some(ref mut f) => f(current, totols),
                    None => 0
                }
            }

            Ok(())
        }

        pub fn open(&self, file_name: &str, mode: &str, offset: usize, size: usize) -> Result<(), &str> {
            let c_file_name = CString::new(file_name).unwrap();
            let c_mode = CString::new(mode).unwrap();
            unsafe { 
                let retval = raw::darts_open(self.darts_t, c_file_name.as_ptr(), c_mode.as_ptr(), offset, size);
                if retval != 0 {
                    let err = CStr::from_ptr(raw::darts_error(self.darts_t));
                    return Err(err.to_str().unwrap());
                }
                Ok(())
            }
        }

        pub fn save(&self, file_name: &str, mode: &str, offset: usize) -> Result<(), &str> {
            let c_file_name = CString::new(file_name).unwrap();
            let c_mode = CString::new(mode).unwrap();
            unsafe { 
                let retval = raw::darts_save(self.darts_t, c_file_name.as_ptr(), c_mode.as_ptr(), offset);
                if retval != 0 {
                    let err = CStr::from_ptr(raw::darts_error(self.darts_t));
                    return Err(err.to_str().unwrap());
                }
                Ok(())
            }
        }

        pub fn exact_match_search(&self, key: &str, length: usize, node_pos: usize) -> i32 {
            let c_key = CString::new(key).unwrap();
            unsafe { raw::darts_exact_match_search(self.darts_t, c_key.as_ptr(), length, node_pos) }
        }

        pub fn exact_match_search_pair(&self, key: &str, length: usize, node_pos: usize) -> ResultPairType {
            let c_key = CString::new(key).unwrap();
            unsafe { 
                let result = raw::darts_exact_match_search_pair(self.darts_t, c_key.as_ptr(), length, node_pos);
                ResultPairType {
                    value: result.value,
                    length: result.length,
                }
            }
        }

        pub fn common_prefix_search(&self, key: &str, max_num_results: usize, length: usize, node_pos: usize) -> Vec<ResultPairType> {
            let c_key = CString::new(key).unwrap();
            let mut raw_results = Vec::with_capacity(max_num_results);
            unsafe {
                let num = raw::darts_common_prefix_search(self.darts_t, c_key.as_ptr(), raw_results.as_mut_ptr(), max_num_results, length, node_pos);
                raw_results.set_len(num);
                let results = raw_results
                    .iter().map(|result| ResultPairType { value: result.value, length: result.length })
                    .collect();
                results
            }
        }

        pub fn traverse(&self, key: &str, node_pos: *mut usize, key_pos: *mut usize, length: usize) -> i32 {
            let c_key = CString::new(key).unwrap();
            unsafe { raw::darts_traverse(self.darts_t, c_key.as_ptr(), node_pos, key_pos, length) }
        }
    }
    
    impl Default for DoubleArrayTrie {
        fn default() -> Self {
            Self::new()
        }
    }

    impl Drop for DoubleArrayTrie {
        fn drop(&mut self) {
            unsafe {
                raw::darts_clear(self.darts_t);
                raw::darts_delete(self.darts_t);
            }
        }
    }
}

#[cfg(test)]
mod tests;
