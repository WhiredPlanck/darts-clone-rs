use libc::{c_void, c_char, c_int, size_t};

/// Type of double array instance.
pub type DartsT = *mut c_void;
/// Type of double array trie key.
pub type DartsKeyType = c_char;
/// Type of double array trie value.
pub type DartsValueType = c_int;

/// Enables applications to get the lengths of the
/// matched keys in addition to the values.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DartsResultPairType {
    pub value: DartsValueType,
    pub length: size_t,
}

#[link(name = "darts")]
extern "C" {
    /// Constructs an instance of double array trie.
    pub fn darts_new() -> DartsT;

    /// Frees an allocated double array trie.
    pub fn darts_delete(darts: DartsT);

    /// Returns the internal error. It could be NULL if there is no error.
    pub fn darts_error(darts: DartsT) -> *const c_char;

    /// Returns version string of Darts.
    pub fn darts_version() -> *const c_char;

    /// Calls [`darts_clear`] in order to free memory allocated to the
    /// old array and then sets a new array. This function is useful to set a memory-
    /// mapped array.
    /// 
    /// Note that the array set by this function is not freed in
    /// [`darts_clear`] and [`darts_delete`].
    /// 
    /// darts_set_array() can also set the size of the new array but the size is not
    /// used in search methods. So it works well even if the size is 0 or omitted.
    /// Remember that [`darts_size`] and [darts_total_size] returns 0 in such a case.
    pub fn darts_set_array(darts: DartsT, ptr: *const c_void, size: size_t);

    /// Returns a pointer to the array of units.
    pub fn darts_array(darts: DartsT) -> *const c_void;

    /// Frees memory allocated to units and then initializes member
    /// variables with 0 and NULLs. Note that this does not free memory if
    /// the array of units was set by [`darts_set_array`].
    pub fn darts_clear(darts: DartsT);

    /// Returns the size of each unit.
    pub fn darts_unit_size(darts: DartsT) -> size_t;

    /// Returns the number of units. It can be 0 if [`darts_set_array`] is used.
    pub fn darts_size(darts: DartsT) -> size_t;

    /// Returns the number of bytes allocated to the array of units.
    /// It can be 0 if [`darts_set_array`] is used.
    pub fn darts_total_size(darts: DartsT) -> size_t;

    /// Exists for compatibility. It always returns the number of
    /// units because it takes long time to count the number of non-zero units.
    pub fn darts_nonzero_size(darts: DartsT) -> size_t;

    /// Constructs a dictionary from given key-value pairs. If `lengths`
    /// is NULL, `keys` is handled as an array of zero-terminated strings. If
    /// `values` is NULL, the index in `keys` is associated with each key, i.e.
    /// the ith key has (i - 1) as its value.
    /// 
    /// Note that the key-value pairs must be arranged in key order and the values
    /// must not be negative. Also, if there are duplicate keys, only the first
    /// pair will be stored in the resultant dictionary.
    /// 
    /// `progress_func` is a pointer to a callback function. If it is not NULL,
    /// it will be called in this function so that the caller can check the progress of
    /// dictionary construction.
    /// 
    /// The return value is 0, and it indicates the success of the
    /// operation. Otherwise, get error message from darts_error().
    /// 
    /// Uses another construction algorithm if `values` is not NULL. In
    /// this case, Darts-clone uses a Directed Acyclic Word Graph (DAWG) instead
    /// of a trie because a DAWG is likely to be more compact than a trie.
    pub fn darts_build(
        darts: DartsT,
        num_keys: size_t,
        keys: *const *const DartsKeyType,
        lengths: *const size_t,
        values: *const DartsValueType,
        progress_func: ::std::option::Option<
            unsafe extern "C" fn(arg1: size_t, arg2: size_t) -> c_int,
        >,
    ) -> c_int;

    /// Reads an array of units from the specified file. And if it goes
    /// well, the old array will be freed and replaced with the new array read
    /// from the file. `offset` specifies the number of bytes to be skipped before
    /// reading an array. `size` specifies the number of bytes to be read from the
    /// file. If the `size' is 0, the whole file will be read.
    /// 
    /// Returns 0 iff the operation succeeds. Otherwise,
    /// get error message from [`darts_error`].
    pub fn darts_open(
        darts: DartsT,
        file_name: *const c_char,
        mode: *const c_char,
        offset: size_t,
        size: size_t,
    ) -> c_int;

    /// Writes the array of units into the specified file. `offset'
    /// specifies the number of bytes to be skipped before writing the array.
    /// Returns 0 iff the operation succeeds. Otherwise, returns a non-zero value.
    pub fn darts_save(
        darts: DartsT,
        file_name: *const c_char,
        mode: *const c_char,
        offset: size_t,
    ) -> c_int;

    /// Tests whether the given key exists or not, and
    /// if it exists, its value and length are returned. Otherwise, the
    /// value and the length of return value are set to -1 and 0 respectively.
    /// 
    /// Note that if `length` is 0, `key` is handled as a zero-terminated string.
    /// `node_pos` specifies the start position of matching. This argument enables
    /// the combination of darts_exact_match_search and [`darts_traverse`]. For example, if you
    /// want to test "xyzA", "xyzBC", and "xyzDE", you can use [`darts_traverse`] to get
    /// the node position corresponding to "xyz" and then you can use
    /// darts_exact_match_search to test "A", "BC", and "DE" from that position.
    /// 
    /// Note that the length of `result` indicates the length from the `node_pos`.
    /// In the above example, the lengths are { 1, 2, 2 }, not { 4, 5, 5 }.
    pub fn darts_exact_match_search(
        darts: DartsT,
        key: *const DartsKeyType,
        length: size_t,
        node_pos: size_t,
    ) -> DartsValueType;

    /// Returns a [`DartsResultPairType`] instead.
    pub fn darts_exact_match_search_pair(
        darts: DartsT,
        key: *const DartsKeyType,
        length: size_t,
        node_pos: size_t,
    ) -> DartsResultPairType;

    /// Searches for keys which match a prefix of the
    /// given string. If `length` is 0, `key` is handled as a zero-terminated string.
    /// The values and the lengths of at most `max_num_results` matched keys are
    /// stored in `results`. Returns the number of matched
    /// keys. Note that the return value can be larger than `max_num_results` if
    /// there are more than `max_num_results` matches. If you want to get all the
    /// results, allocate more spaces and call this again.
    /// `node_pos` works as well as in [`darts_exact_match_search`].
    pub fn darts_common_prefix_search(
        darts: DartsT,
        key: *const DartsKeyType,
        results: *mut DartsResultPairType,
        max_num_results: size_t,
        length: size_t,
        node_pos: size_t,
    ) -> size_t;

    /// In Darts-clone, a dictionary is a deterministic finite-state automaton
    /// (DFA) and this tests transitions on the DFA. The initial state is
    /// `node_pos` and the function chooses transitions labeled `key[key_pos]`,
    /// `key[key_pos + 1]`, ... in order. If there is not a transition labeled
    /// `key[key_pos + i]`, the function terminates the transitions at that state and
    /// returns -2. Otherwise, the function ends without a termination and returns
    /// -1 or a nonnegative value, -1 indicates that the final state was not an
    /// accept state. When a nonnegative value is returned, it is the value
    /// associated with the final accept state. That is, the function returns the
    /// value associated with the given key if it exists. Note that the function
    /// updates `node_pos` and `key_pos` after each transition.
    pub fn darts_traverse(
        darts: DartsT,
        key: *const DartsKeyType,
        node_pos: *mut size_t,
        key_pos: *mut size_t,
        length: size_t,
    ) -> DartsValueType;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_works() {
        let dic = unsafe { darts_new() };
        assert_ne!(dic, std::ptr::null_mut());
        unsafe { darts_delete(dic); }
    }
}
