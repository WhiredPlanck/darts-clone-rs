use libc::{c_void, c_char, c_int, size_t};

#[doc = "Type of double array instance."]
pub type DartsT = *mut c_void;
#[doc = "Type of double array trie key."]
pub type DartsKeyType = c_char;
#[doc = "Type of double array trie value."]
pub type DartsValueType = c_int;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DartsResultPairType {
    pub value: DartsValueType,
    pub length: size_t,
}

#[link(name = "darts")]
extern "C" {
    #[doc = "darts_new() constructs an instance of double array trie."]
    pub fn darts_new() -> DartsT;

    #[doc = "darts_new() frees an allocated double array trie."]
    pub fn darts_delete(darts: DartsT);

    #[doc = "darts_error() returns the internal error. It could be NULL if there is\nno error."]
    pub fn darts_error(darts: DartsT) -> *const c_char;

    #[doc = "Returns version string of Darts."]
    pub fn darts_version() -> *const c_char;

    #[doc = "darts_set_array() calls darts_clear() in order to free memory allocated to the\nold array and then sets a new array. This function is useful to set a memory-\nmapped array.\n\nNote that the array set by darts_set_array() is not freed in\ndarts_clear() and darts_delete().\n\ndarts_set_array() can also set the size of the new array but the size is not\nused in search methods. So it works well even if the size is 0 or omitted.\nRemember that darts_size() and darts_total_size() returns 0 in such a case."]
    pub fn darts_set_array(darts: DartsT, ptr: *const c_void, size: size_t);

    #[doc = "darts_array() returns a pointer to the array of units."]
    pub fn darts_array(darts: DartsT) -> *const c_void;

    #[doc = "darts_clear() frees memory allocated to units and then initializes member\nvariables with 0 and NULLs. Note that darts_clear() does not free memory if\nthe array of units was set by darts_clear(). In such a case, `array_' is not\nNULL and `buf_' is NULL."]
    pub fn darts_clear(darts: DartsT);

    #[doc = "unit_size() returns the size of each unit. The size must be 4 bytes."]
    pub fn darts_unit_size(darts: DartsT) -> size_t;

    #[doc = "size() returns the number of units. It can be 0 if set_array() is used."]
    pub fn darts_size(darts: DartsT) -> size_t;

    #[doc = "total_size() returns the number of bytes allocated to the array of units.\nIt can be 0 if set_array() is used."]
    pub fn darts_total_size(darts: DartsT) -> size_t;

    #[doc = "nonzero_size() exists for compatibility. It always returns the number of\nunits because it takes long time to count the number of non-zero units."]
    pub fn darts_nonzero_size(darts: DartsT) -> size_t;

    #[doc = "darts_build() constructs a dictionary from given key-value pairs. If `lengths'\nis NULL, `keys' is handled as an array of zero-terminated strings. If\n`values' is NULL, the index in `keys' is associated with each key, i.e.\nthe ith key has (i - 1) as its value.\n\nNote that the key-value pairs must be arranged in key order and the values\nmust not be negative. Also, if there are duplicate keys, only the first\npair will be stored in the resultant dictionary.\n\n`progress_func' is a pointer to a callback function. If it is not NULL,\nit will be called in darts_build() so that the caller can check the progress of\ndictionary construction.\n\nThe return value of darts_build() is 0, and it indicates the success of the\noperation. Otherwise, get error message from darts_error().\n\ndarts_build() uses another construction algorithm if `values' is not NULL. In\nthis case, Darts-clone uses a Directed Acyclic Word Graph (DAWG) instead\nof a trie because a DAWG is likely to be more compact than a trie."]
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

    #[doc = "darts_open() reads an array of units from the specified file. And if it goes\nwell, the old array will be freed and replaced with the new array read\nfrom the file. `offset' specifies the number of bytes to be skipped before\nreading an array. `size' specifies the number of bytes to be read from the\nfile. If the `size' is 0, the whole file will be read.\n\ndarts_open() returns 0 iff the operation succeeds. Otherwise,\nget error message from darts_error()."]
    pub fn darts_open(
        darts: DartsT,
        file_name: *const c_char,
        mode: *const c_char,
        offset: size_t,
        size: size_t,
    ) -> c_int;

    #[doc = "darts_save() writes the array of units into the specified file. `offset'\nspecifies the number of bytes to be skipped before writing the array.\ndarts_save() returns 0 iff the operation succeeds. Otherwise, it returns a\nnon-zero value."]
    pub fn darts_save(
        darts: DartsT,
        file_name: *const c_char,
        mode: *const c_char,
        offset: size_t,
    ) -> c_int;

    #[doc = "darts_exact_match_search() tests whether the given key exists or not, and\nif it exists, its value and length are returned. Otherwise, the\nvalue and the length of return value are set to -1 and 0 respectively.\n\nNote that if `length' is 0, `key' is handled as a zero-terminated string.\n`node_pos' specifies the start position of matching. This argument enables\nthe combination of exactMatchSearch() and traverse(). For example, if you\nwant to test \"xyzA\", \"xyzBC\", and \"xyzDE\", you can use traverse() to get\nthe node position corresponding to \"xyz\" and then you can use\nexactMatchSearch() to test \"A\", \"BC\", and \"DE\" from that position.\n\nNote that the length of `result' indicates the length from the `node_pos'.\nIn the above example, the lengths are { 1, 2, 2 }, not { 4, 5, 5 }."]
    pub fn darts_exact_match_search(
        darts: DartsT,
        key: *const DartsKeyType,
        length: size_t,
        node_pos: size_t,
    ) -> DartsValueType;

    #[doc = "darts_exact_match_search_pair() returns a darts_result_pair instead."]
    pub fn darts_exact_match_search_pair(
        darts: DartsT,
        key: *const DartsKeyType,
        length: size_t,
        node_pos: size_t,
    ) -> DartsResultPairType;

    #[doc = "darts_common_prefix_search() searches for keys which match a prefix of the\ngiven string. If `length' is 0, `key' is handled as a zero-terminated string.\nThe values and the lengths of at most `max_num_results' matched keys are\nstored in `results'. commonPrefixSearch() returns the number of matched\nkeys. Note that the return value can be larger than `max_num_results' if\nthere are more than `max_num_results' matches. If you want to get all the\nresults, allocate more spaces and call commonPrefixSearch() again.\n`node_pos' works as well as in exactMatchSearch()."]
    pub fn darts_common_prefix_search(
        darts: DartsT,
        key: *const DartsKeyType,
        results: *mut DartsResultPairType,
        max_num_results: size_t,
        length: size_t,
        node_pos: size_t,
    ) -> size_t;

    #[doc = "In Darts-clone, a dictionary is a deterministic finite-state automaton\n(DFA) and traverse() tests transitions on the DFA. The initial state is\n`node_pos' and traverse() chooses transitions labeled key[key_pos],\nkey[key_pos + 1], ... in order. If there is not a transition labeled\nkey[key_pos + i], traverse() terminates the transitions at that state and\nreturns -2. Otherwise, traverse() ends without a termination and returns\n-1 or a nonnegative value, -1 indicates that the final state was not an\naccept state. When a nonnegative value is returned, it is the value\nassociated with the final accept state. That is, traverse() returns the\nvalue associated with the given key if it exists. Note that traverse()\nupdates `node_pos' and `key_pos' after each transition."]
    pub fn darts_traverse(
        darts: DartsT,
        key: *const DartsKeyType,
        node_pos: *mut size_t,
        key_pos: *mut size_t,
        length: size_t,
    ) -> DartsValueType;
}
