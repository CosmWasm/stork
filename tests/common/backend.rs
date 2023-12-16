use std::collections::{btree_map, BTreeMap};
use std::iter::Cloned;

use cosmwasm_std::testing::MockStorage;
use cosmwasm_std::Storage as _;

pub struct TestStorage(BTreeMap<Vec<u8>, Vec<u8>>);

impl TestStorage {
    pub fn new() -> Self {
        Self(BTreeMap::new())
    }
}

impl stork::StorageBackend for TestStorage {
    fn get(&self, key: &[u8]) -> Option<Vec<u8>> {
        self.0.get(key).map(|v| v.clone())
    }

    fn set(&mut self, key: &[u8], value: &[u8]) {
        self.0.insert(key.to_vec(), value.to_vec());
    }

    fn remove(&mut self, key: &[u8]) {
        self.0.remove(key);
    }
}

impl stork::StorageIterableBackend for TestStorage {
    type KeysIterator<'a> = Box<dyn Iterator<Item = Vec<u8>> + 'a>;
    type ValuesIterator<'a> = Box<dyn Iterator<Item = Vec<u8>> + 'a>;
    type PairsIterator<'a> = Box<dyn Iterator<Item = (Vec<u8>, Vec<u8>)> + 'a>;

    fn keys<'a>(
        &'a self,
        start: Option<&'a [u8]>,
        end: Option<&'a [u8]>,
    ) -> Self::KeysIterator<'a> {
        Box::new(
            self.0
                .keys()
                .filter(move |k| check_bounds(k, start, end))
                .cloned(),
        )
    }

    fn values<'a>(
        &'a self,
        start: Option<&'a [u8]>,
        end: Option<&'a [u8]>,
    ) -> Self::ValuesIterator<'a> {
        Box::new(
            self.0
                .iter()
                .filter(move |(k, _)| check_bounds(k, start, end))
                .map(|(_, v)| v.clone()),
        )
    }

    fn pairs<'a>(
        &'a self,
        start: Option<&'a [u8]>,
        end: Option<&'a [u8]>,
    ) -> Self::PairsIterator<'a> {
        Box::new(
            self.0
                .clone()
                .into_iter()
                .filter(move |(k, _)| check_bounds(k, start, end)),
        )
    }
}

fn check_bounds(v: &[u8], start: Option<&[u8]>, end: Option<&[u8]>) -> bool {
    if let Some(start) = start {
        if v < start {
            return false;
        }
    }
    if let Some(end) = end {
        if v >= end {
            return false;
        }
    }
    true
}