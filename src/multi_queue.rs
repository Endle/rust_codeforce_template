//HashMap Value is Queue


use std::collections::HashMap;
use std::collections::VecDeque;
pub struct MultiQueue<KeyT, ValT> where
    KeyT: Eq + std::hash::Hash
{
    _data: HashMap<KeyT, VecDeque<ValT> >,
}
impl <KeyT:Eq  + std::hash::Hash,  ValT> MultiQueue<KeyT, ValT> {
    pub fn new() -> Self {
        MultiQueue {
            _data : HashMap::new()
        }
    }

    pub fn push(&mut self, key: KeyT, value: ValT){
        // if !self._data.contains_key(&key) {
        //     self._data.insert(
        //         key,
        //         VecDeque::new()
        //     );
        // };
        let mut entry = self._data.entry(key).or_insert(
            VecDeque::new()
        );
        entry.push_back(value);
        // self._data.get(&key).expect("Assert exist").push_back(value)
    }

    /// Return None if 1. key not found 2. queue consumed
    pub fn pop(&mut self, key: KeyT) -> Option<ValT> {
        let mut queue = self._data.get_mut(&key);
        if queue.is_none() {
            return None;
        }
        let mut queue = queue.expect("Get Queue");
        // if queue.is_empty() {
        //     return None;
        // }
        queue.pop_front()

    }


    pub fn is_empty(&self, key: KeyT) -> bool{
        let mut queue = self._data.get(&key);
        if queue.is_none() {
            return true;
        }
        let queue = queue.expect("Get Queue");
        // if queue.is_empty() {
        //     return None;
        // }
        queue.is_empty()
    }

}

impl <KeyT:Eq  + std::hash::Hash + std::fmt::Display+Copy,
    ValT: std::fmt::Display+Copy>
std::fmt::Debug for MultiQueue<KeyT, ValT> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Queues count {}\n", self._data.len());
        // let mut keys = self._data.keys().cl
        for key in self._data.keys() {
            write!(f, "    {}  [", key);
            // write!(f, "{:?}", self._data.get(key).unwrap());
            for v in self._data.get(key).unwrap() {
                write!(f, "{}", v);
                write!(f, ",");
                // if j + 1 < self._max_m {

                // }
            }
            write!(f, "]\n");
        }
        write!(f,"")
    }
}