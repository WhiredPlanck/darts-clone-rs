use darts::{DoubleArrayTrie, ResultPairType};
use rand::{distributions::Alphanumeric, Rng};

use super::*;
use std::collections::BTreeSet;

const NUM_VALID_KEYS: usize = 1 << 16;
const NUM_INVALID_KEYS: usize = 1 << 17;
const MAX_NUM_RESULTS: usize = 16;

struct TestData {
    dic: DoubleArrayTrie,
    invalid_keys: BTreeSet<String>,
    keys: Vec<String>,
    lengths: Vec<usize>,
    values: Vec<i32>,
}

impl TestData {
    fn new() -> TestData {
        let valid_keys = generate_valid_keys(NUM_VALID_KEYS);
        let invalid_keys = generate_invalid_keys(NUM_INVALID_KEYS, &valid_keys);

        let keys = Vec::from_iter(valid_keys.iter().map(|s| s.to_owned()));
        let lengths = Vec::from_iter(valid_keys.iter().map(|key| key.len()));
        let values = Vec::from_iter(valid_keys.iter().enumerate().map(|(i, _)| i as i32));

        let dic = darts::DoubleArrayTrie::new();

        TestData { dic, invalid_keys, keys, lengths, values }
    }

    fn random_value(&self) -> Vec<i32> {
        let mut random = Vec::with_capacity(self.values.len());
        for _ in &self.values {
            let value = rand::thread_rng().gen_range(0..10);
            random.push(value);
        }
        random
    }
}



fn generate_valid_keys(num_keys: usize) -> BTreeSet<String> {
    let mut valid_keys = BTreeSet::new();
    
    while valid_keys.len() < num_keys {
        let range = rand::thread_rng().gen_range(1..=8);
        let key: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(range)
            .map(char::from)
            .collect();
        let key_str = format!("{}漢字", key);
        valid_keys.insert(key_str);
    }

    valid_keys
}

fn generate_invalid_keys(num_keys: usize, valid_keys: &BTreeSet<String>) -> BTreeSet<String> {
    let mut invalid_keys = BTreeSet::new();

    while invalid_keys.len() < num_keys {
        let range = rand::thread_rng().gen_range(1..=8);
        let key: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(range)
            .map(char::from)
            .collect();
        if !valid_keys.contains(&key) {
            invalid_keys.insert(key);
        }
    }

    invalid_keys
}

fn test_dic(dic: &DoubleArrayTrie, keys: &Vec<String>, lengths: &Vec<usize>, values: &Vec<i32>, invalid_keys: &BTreeSet<String>) {
    let mut value: i32;
    let mut result: ResultPairType;

    for (i, key) in keys.iter().enumerate() {
        value = dic.exact_match_search(key, 0, 0);
        assert_eq!(value, values[i]);

        result = dic.exact_match_search_pair(key, 0, 0);
        assert_eq!(result.value, values[i]);
        assert_eq!(result.length, lengths[i]);

        value = dic.exact_match_search(key, lengths[i], 0);
        assert_eq!(value, values[i]);

        result = dic.exact_match_search_pair(key, lengths[i], 0);
        assert_eq!(result.value, values[i]);
        assert_eq!(result.length, lengths[i]);
    }

    for key in invalid_keys {
        value = dic.exact_match_search(key, 0, 0);
        assert_eq!(value, -1);

        result = dic.exact_match_search_pair(key, 0, 0);
        assert_eq!(result.value, -1);

        value = dic.exact_match_search(key, key.len(), 0);
        assert_eq!(value, -1);

        result = dic.exact_match_search_pair(key, key.len(), 0);
        assert_eq!(result.value, -1);
    }
}

#[test]
fn build_with_keys() {
    let TestData { dic, invalid_keys, keys, lengths, values } = TestData::new();

    match dic.build(keys.len(), &keys, None, None, None) {
        Ok(_) => test_dic(&dic, &keys, &lengths, &values, &invalid_keys),
        Err(what) => panic!("{}", what),
    }
}

#[test]
fn build_with_keys_and_lengths() {
    let TestData { dic, invalid_keys, keys, lengths, values } = TestData::new();

    match dic.build(keys.len(), &keys, Some(&lengths), None, None) {
        Ok(_) => test_dic(&dic, &keys, &lengths, &values, &invalid_keys),
        Err(what) => panic!("{}", what),
    }
}

#[test]
fn build_with_keys_lengths_and_values() {
    let TestData { dic, invalid_keys, keys, lengths, values } = TestData::new();

    match dic.build(keys.len(), &keys, Some(&lengths), Some(&values), None) {
        Ok(_) => test_dic(&dic, &keys, &lengths, &values, &invalid_keys),
        Err(what) => panic!("{}", what),
    }
}

#[test]
fn build_with_keys_lengths_and_random_values() {
    let data = TestData::new();
    let TestData { dic, invalid_keys, keys, lengths, .. } = &data;
    let random = data.random_value();

    match dic.build(keys.len(), &keys, Some(&lengths), Some(&random), None) {
        Ok(_) => test_dic(&dic, &keys, &lengths, &random, &invalid_keys),
        Err(what) => panic!("{}", what),
    }
}

#[test]
fn save_and_open() {
    let data = TestData::new();
    let TestData { dic, invalid_keys, keys, lengths, .. } = &data;
    let random = data.random_value();

    let dic_copy = DoubleArrayTrie::new();
    match dic.build(keys.len(), &keys, Some(&lengths), Some(&random), None) {
        Ok(_) => {
            assert_eq!(Ok(()), dic.save("test-darts.dic", "wb", 0));
            assert_eq!(Ok(()), dic_copy.open("test-darts.dic", "rb", 0, 0));
            assert_eq!(dic.size(), dic_copy.size());
    
            test_dic(&dic_copy, &keys, &lengths, &random, &invalid_keys);
        },
        Err(what) => panic!("{}", what),
    }
}

#[test]
fn set_array_with_array() {
    let data = TestData::new();
    let TestData { dic, invalid_keys, keys, lengths, .. } = &data;
    let random = data.random_value();

    let mut dic_copy = DoubleArrayTrie::new();
    match dic.build(keys.len(), &keys, Some(&lengths), Some(&random), None) {
        Ok(_) => {
            dic.save("test-darts.dic", "wb", 0).unwrap();
            dic_copy.open("test-darts.dic", "rb", 0, 0).unwrap();
            dic_copy.set_array(&dic.array(), 0);
            assert_eq!(dic_copy.size(), 0);
    
            test_dic(&dic_copy, &keys, &lengths, &random, &invalid_keys);
        },
        Err(what) => panic!("{}", what),
    }
}

#[test]
fn set_array_with_array_and_size() {
    let data = TestData::new();
    let TestData { dic, invalid_keys, keys, lengths, .. } = &data;
    let random = data.random_value();

    let mut dic_copy = DoubleArrayTrie::new();
    match dic.build(keys.len(), &keys, Some(&lengths), Some(&random), None) {
        Ok(_) => {
            dic.save("test-darts.dic", "wb", 0).unwrap();
            dic_copy.open("test-darts.dic", "rb", 0, 0).unwrap();
            dic_copy.set_array(&dic.array(), dic.size());
            assert_eq!(dic_copy.size(), dic.size());
    
            test_dic(&dic_copy, &keys, &lengths, &random, &invalid_keys);
        },
        Err(what) => panic!("{}", what),
    }
}

#[test]
fn common_prefix_search() {
    let data = TestData::new();
    let TestData { dic, invalid_keys, keys, lengths, .. } = &data;
    let random = data.random_value();

    match dic.build(keys.len(), &keys, Some(&lengths), Some(&random), None) {
        Ok(_) => {
            for (i, key) in keys.iter().enumerate() {
                let results = dic.common_prefix_search(key, MAX_NUM_RESULTS, 0, 0);
                let num_results = results.len();
        
                assert!(num_results >= 1);
                assert!(num_results < 10);
        
                assert_eq!(results[num_results - 1].value, random[i]);
                assert_eq!(results[num_results - 1].length, lengths[i]);
        
                let results_with_length = dic.common_prefix_search(key, MAX_NUM_RESULTS, lengths[i], 0);
                let num_results_with_length = results_with_length.len();
        
                assert_eq!(num_results, num_results_with_length);
                assert_eq!(results, results_with_length);
            }

            for key in invalid_keys {
                let results = dic.common_prefix_search(key, MAX_NUM_RESULTS, 0, 0);
                let num_results = results.len();

                assert!(num_results < 10);

                if num_results > 0 {
                    assert_ne!(results[num_results - 1].value, -1);
                    assert!(results[num_results - 1].length < key.len());
                }

                let results_with_length = dic.common_prefix_search(key, MAX_NUM_RESULTS, key.len(), 0);
                assert_eq!(results_with_length.len(), num_results);
                assert_eq!(results, results_with_length);
            }
        },
        Err(what) => panic!("{}", what),
    }
}

#[test]
fn tarverse() {
    let data = TestData::new();
    let TestData { dic, invalid_keys, keys, lengths, .. } = &data;
    let random = data.random_value();

    match dic.build(keys.len(), &keys, Some(&lengths), Some(&random), None) {
        Ok(_) => {
            for (i, key) in keys.iter().enumerate() {
                let mut id = 0usize;
                let mut key_pos = 0usize;
                let mut result = 0;
                for i in 0..key.len() {
                    result = dic.traverse(key, &mut id, &mut key_pos, i + 1);
                    assert_ne!(result, -2);
                }
                assert_eq!(result, random[i]);
            }

            for key in invalid_keys {
                let mut id = 0usize;
                let mut key_pos = 0usize;
                let mut result = 0;
                for i in 0..key.len() {
                    result = dic.traverse(key, &mut id, &mut key_pos, i + 1);
                    if result == -2 {
                        break;
                    }
                }
                assert!(result < 0);
            }
        },
        Err(what) => panic!("{}", what),
    }
}

