//! Module for bindings to Darts-clone.

use std::{
    ffi::{c_void, CStr, CString},
    ptr,
};

use libdarts_sys as raw;

/// Type of double array trie instance.
pub struct DoubleArrayTrie {
    darts_t: raw::DartsT,
}

/// Type of array of units.
pub struct Array {
    array: *const c_void,
}

impl Array {
    pub fn new() -> Array {
        Array {
            array: ptr::null_mut(),
        }
    }
}

/// Enables applications to get the lengths of the
/// matched keys in addition to the values.
#[derive(Debug, PartialEq)]
pub struct ResultPairType {
    pub value: i32,
    pub length: usize,
}

/// Type of callback functions for reporting the progress of building a dictionary.
pub type Progress = dyn FnMut(usize, usize) -> i32;

impl DoubleArrayTrie {
    /// Constructs an instance of double array trie.
    pub fn new() -> DoubleArrayTrie {
        DoubleArrayTrie {
            darts_t: unsafe { raw::darts_new() },
        }
    }

    /// Calls [`DoubleArrayTrie::clear`] in order to free memory allocated to the
    /// old array and then sets a new array. This function is useful to set a memory-
    /// mapped array.
    ///
    /// It can also set the size of the new array but the size is not
    /// used in search methods. So it works well even if the size is 0 or omitted.
    /// Remember that [`DoubleArrayTrie::size`] and [`DoubleArrayTrie::total_size`]
    /// returns 0 in such a case.
    pub fn set_array(&mut self, array: &Array, size: usize) {
        unsafe {
            raw::darts_set_array(self.darts_t, array.array, size);
        }
    }

    /// Returns a instance to the array of units.
    pub fn array(&self) -> Array {
        Array {
            array: unsafe { raw::darts_array(self.darts_t) },
        }
    }

    /// Frees memory allocated to units.
    pub fn clear(&self) {
        unsafe {
            raw::darts_clear(self.darts_t);
        }
    }

    /// Returns the size of each unit.
    pub fn unit_size(&self) -> usize {
        unsafe { raw::darts_unit_size(self.darts_t) }
    }

    /// Returns the number of units. It can be 0 if [`DoubleArrayTrie::set_array`] is used.
    pub fn size(&self) -> usize {
        unsafe { raw::darts_size(self.darts_t) }
    }

    /// Returns the number of bytes allocated to the array of units.
    /// It can be 0 if [`DoubleArrayTrie::set_array`] is used.
    pub fn total_size(&self) -> usize {
        unsafe { raw::darts_total_size(self.darts_t) }
    }

    /// Exists for compatibility. It always returns the number of
    /// units because it takes long time to count the number of non-zero units.
    pub fn nonzero_size(&self) -> usize {
        unsafe { raw::darts_nonzero_size(self.darts_t) }
    }

    /// Constructs a dictionary from given key-value pairs. If `lengths`
    /// is [`None`], `keys` is handled as an array of strings. If
    /// `values` is None, the index in `keys` is associated with each key, i.e.
    /// the ith key has (i - 1) as its value.
    ///
    /// Note that the key-value pairs must be arranged in key order and the values
    /// must not be negative. Also, if there are duplicate keys, only the first
    /// pair will be stored in the resultant dictionary.
    ///
    /// `progress_func` is a optional callback function. If it is not None,
    /// it will be called when building so that the caller can check the progress of
    /// dictionary construction.
    ///
    /// It uses another construction algorithm if `values` is not [`None`]. In
    /// this case, Darts-clone uses a Directed Acyclic Word Graph (DAWG) instead
    /// of a trie because a DAWG is likely to be more compact than a trie.
    pub fn build(
        &self,
        num_keys: usize,
        keys: &Vec<String>,
        lengths: Option<&[usize]>,
        values: Option<&[i32]>,
        progress_func: Option<Box<Progress>>,
    ) -> Result<(), &str> {
        let keys = keys
            .iter()
            .map(|key| CString::new(key.as_bytes()).unwrap())
            .collect::<Vec<_>>();

        let mut c_keys: Vec<*const std::os::raw::c_char> = Vec::with_capacity(num_keys + 1);
        for key in &keys {
            c_keys.push(key.as_ptr() as *const std::os::raw::c_char);
        }
        c_keys.push(ptr::null());

        let c_lengths = match lengths {
            Some(lengths) => &lengths[0],
            None => ptr::null(),
        };

        let c_values = match values {
            Some(values) => &values[0],
            None => ptr::null(),
        };

        static mut STORED_PROGRESS: Option<Box<Progress>> = None;

        unsafe {
            STORED_PROGRESS = progress_func;
            let retval = raw::darts_build(
                self.darts_t,
                num_keys,
                &c_keys[0],
                c_lengths,
                c_values,
                Some(progress_callback),
            );
            if retval != 0 {
                let err = CStr::from_ptr(raw::darts_error(self.darts_t));
                return Err(err.to_str().unwrap());
            }
        }

        unsafe extern "C" fn progress_callback(current: usize, totols: usize) -> i32 {
            match STORED_PROGRESS {
                Some(ref mut f) => f(current, totols),
                None => 0,
            }
        }

        Ok(())
    }

    /// Reads an array of units from the specified file. And if it goes
    /// well, the old array will be freed and replaced with the new array read
    /// from the file. `offset` specifies the number of bytes to be skipped before
    /// reading an array. `size` specifies the number of bytes to be read from the
    /// file. If the `size` is 0, the whole file will be read.
    pub fn open(
        &self,
        file_name: &str,
        mode: &str,
        offset: usize,
        size: usize,
    ) -> Result<(), &str> {
        let c_file_name = CString::new(file_name).unwrap();
        let c_mode = CString::new(mode).unwrap();
        unsafe {
            let retval = raw::darts_open(
                self.darts_t,
                c_file_name.as_ptr(),
                c_mode.as_ptr(),
                offset,
                size,
            );
            if retval != 0 {
                let err = CStr::from_ptr(raw::darts_error(self.darts_t));
                return Err(err.to_str().unwrap());
            }
            Ok(())
        }
    }

    /// Writes the array of units into the specified file. `offset`
    /// specifies the number of bytes to be skipped before writing the array.
    pub fn save(&self, file_name: &str, mode: &str, offset: usize) -> Result<(), &str> {
        let c_file_name = CString::new(file_name).unwrap();
        let c_mode = CString::new(mode).unwrap();
        unsafe {
            let retval =
                raw::darts_save(self.darts_t, c_file_name.as_ptr(), c_mode.as_ptr(), offset);
            if retval != 0 {
                let err = CStr::from_ptr(raw::darts_error(self.darts_t));
                return Err(err.to_str().unwrap());
            }
            Ok(())
        }
    }

    /// Tests whether the given key exists or not, and if it exists,
    /// its value and length are returned. Otherwise, the value and 
    /// the length of return value are set to -1 and 0 respectively.
    ///
    /// Note that if `length` is 0, `key` is handled as a string. `node_pos` 
    /// specifies the start position of matching. This argument enable the 
    /// combination of [`DoubleArrayTrie::exact_match_search`] and
    /// [`DoubleArrayTrie::traverse`]. For example, if you want to test "xyzA",
    /// "xyzBC", and "xyzDE", you can use [`DoubleArrayTrie::traverse`] to 
    /// get the node position corresponding to "xyz" and then you can
    /// use [`DoubleArrayTrie::exact_match_search`] to test "A", "BC",
    /// and "DE" from that position.
    ///
    /// Note that the length of `result` indicates the length from the `node_pos`.
    /// In the above example, the lengths are { 1, 2, 2 }, not { 4, 5, 5 }.
    pub fn exact_match_search(&self, key: &str, length: usize, node_pos: usize) -> i32 {
        let c_key = CString::new(key).unwrap();
        unsafe { raw::darts_exact_match_search(self.darts_t, c_key.as_ptr(), length, node_pos) }
    }

    /// [`DoubleArrayTrie::exact_match_search`] but returns a [`ResultPairType`] instead.
    pub fn exact_match_search_pair(
        &self,
        key: &str,
        length: usize,
        node_pos: usize,
    ) -> ResultPairType {
        let c_key = CString::new(key).unwrap();
        unsafe {
            let result =
                raw::darts_exact_match_search_pair(self.darts_t, c_key.as_ptr(), length, node_pos);
            ResultPairType {
                value: result.value,
                length: result.length,
            }
        }
    }

    /// Searches for keys which match a prefix of the given string.
    /// If `length` is 0, `key` is handled as a string.
    /// The values and the lengths of at most `max_num_results` matched keys are
    /// stored and will be returned.
    /// Note that the length of return value can be larger than `max_num_results` if
    /// there are more than `max_num_results` matches. If you want to get all the
    /// results, allocate more spaces and call this function again.
    /// `node_pos` works as well as in [`DoubleArrayTrie::exact_match_search`].
    pub fn common_prefix_search(
        &self,
        key: &str,
        max_num_results: usize,
        length: usize,
        node_pos: usize,
    ) -> Vec<ResultPairType> {
        let c_key = CString::new(key).unwrap();
        let mut raw_results = Vec::with_capacity(max_num_results);
        unsafe {
            let num = raw::darts_common_prefix_search(
                self.darts_t,
                c_key.as_ptr(),
                raw_results.as_mut_ptr(),
                max_num_results,
                length,
                node_pos,
            );
            raw_results.set_len(num);
            let results = raw_results
                .iter()
                .map(|result| ResultPairType {
                    value: result.value,
                    length: result.length,
                })
                .collect();
            results
        }
    }

    /// Searches for the longest key which matches a prefix of the given string,
    /// and if it exists, its value and length are set to `result`. Otherwise, 
    /// the value and the length of `result` are set to -1 and 0 respectively.
    /// Note that if `length` is 0, `key` is handled as a zero-terminated string.
    /// `node_pos` works as well as in [`DoubleArrayTrie::exact_match_search`].
    pub fn common_longest_prefix_search(
        &self,
        key: &str,
        length: usize,
        node_pos: usize,
    ) -> i32 {
        let c_key = CString::new(key).unwrap();
        unsafe {
            raw::darts_common_longest_prefix_search(
                self.darts_t,
                c_key.as_ptr(),
                length,
                node_pos,
            )
        }
    }

    /// [`DoubleArrayTrie::common_longest_prefix_search`] but returns a [`ResultPairType`] instead.
    pub fn common_longest_prefix_search_pair(
        &self,
        key: &str,
        length: usize,
        node_pos: usize,
    ) -> ResultPairType {
        let c_key = CString::new(key).unwrap();
        unsafe {
            let result = raw::darts_common_longest_prefix_search_pair(
                self.darts_t,
                c_key.as_ptr(),
                length,
                node_pos,
            );
            ResultPairType {
                value: result.value,
                length: result.length,
            }
        }
    }

    /// In Darts-clone, a dictionary is a deterministic finite-state automaton
    /// (DFA) and this function tests transitions on the DFA. The initial state is
    /// `node_pos` and this chooses transitions labeled `key[key_pos]`,
    /// `key[key_pos + 1]`, ... in order. If there is not a transition labeled
    /// `key[key_pos + i]`, this function terminates the transitions at that state and
    /// returns -2. Otherwise, it ends without a termination and returns
    /// -1 or a nonnegative value, -1 indicates that the final state was not an
    /// accept state. When a nonnegative value is returned, it is the value
    /// associated with the final accept state. That is, this function returns the
    /// value associated with the given key if it exists. Note that this function
    /// updates `node_pos` and `key_pos` after each transition.
    pub fn traverse(
        &self,
        key: &str,
        node_pos: *mut usize,
        key_pos: *mut usize,
        length: usize,
    ) -> i32 {
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
