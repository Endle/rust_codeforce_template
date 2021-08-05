// C-style n-dimension array
//currently only 2-D


pub struct Arr <T>
{
    _max_n:usize,
    _max_m:usize,
    _data: Vec<T>,
}

impl<T:Copy> Arr<T> {
    pub fn new(size:(usize,usize), default: T) -> Self{
        let dimensions = 2;
        let container_size = (size.0+1) * (size.1+1) + 1;
        let mut v = Vec::with_capacity(container_size);
        for _ in 0..container_size {
            v.push(default);
        }
        Arr {
            _max_n: size.0,
            _max_m: size.1,
            _data: v,
        }
    }

    fn _get_pos(&self, n:usize, m:usize) -> usize {
        assert!(n <= self._max_n);
        assert!(m <= self._max_m);
        n * self._max_m + m
    }

    pub fn get(&self, n:usize, m:usize) -> T {
        let p = self._get_pos(n, m);
        self._data[p]
    }
    pub fn set(&mut self, n:usize, m:usize, ans:T) -> T {
        let p = self._get_pos(n, m);
        self._data[p] = ans;
        ans
    }
}