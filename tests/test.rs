#[cfg(test)]
#[path = "../src"]
mod test_multi_queue {
    mod multi_queue;
    #[test]
    fn str_usize_tests() {
        let mut mque = multi_queue::MultiQueue::<String, usize>::new();
        mque.push(String::from("a"), 5);
        mque.push(String::from("a"), 7);
        mque.push(String::from("b"), 5);

        let a = mque.pop(String::from("a"));
        assert_eq!(a.unwrap(), 5);
        let a = mque.pop(String::from("a"));
        assert_eq!(a.unwrap(), 7);
        let a = mque.pop(String::from("a"));
        assert!(a.is_none());
        let b = mque.pop(String::from("b"));
        assert_eq!(b.unwrap(), 5);
        let b = mque.pop(String::from("b"));
        assert!(b.is_none());
        let b = mque.pop(String::from("non_exist"));
        assert!(b.is_none());
    }
}



#[cfg(test)]
#[path = "../src"]
mod test_algo {
    use my_lib::algo::calculate_array_inversions;

    mod algo;
    #[test]
    fn test_calculate_array_inversions() {
        let v = &[1, 4, 6];
        assert_eq!(calculate_array_inversions(v), 0);
        let v = &[1, 4, 6, 2];
        assert_eq!(calculate_array_inversions(v), 2);
        let v = &[6, 5, 4, 3, 2, 1];
        assert_eq!(calculate_array_inversions(v), 15);
        let v = &[0, 25, 33, 6, 45, 8, 21, 16, 9, 10, 39, 22, 11, 18, 1, 12, 47, 32, 5, 20, 17, 28, 27, 26, 48, 7, 14, 38, 43, 30, 19, 36, 29, 42, 15, 23, 13, 34, 37, 40, 41, 44, 24, 46, 31, 4, 3, 2, 35, 49];
        assert_eq!(calculate_array_inversions(v), 482);

    }
}


#[cfg(test)]
#[path = "../src"]
mod test_nums {
    use my_lib::nums::{pow_mod, represent_from_bits};

    #[test]
    fn test_pow() {
        const MODULO:i32 = 998244353;
        let v = pow_mod(2, 10, MODULO);
        assert_eq!(v, 1024);

        let v = pow_mod(5, 6, MODULO);
        assert_eq!(v, 15625);

        let v = pow_mod(7, 92, MODULO);
        assert_eq!(v, 807568023);
    }


    use my_lib::nums::represent_into_bits;
    #[test]
    fn test_represent_into_bits() {
        let x = 0;
        let v = represent_into_bits(x);
        let ev = [0u8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        assert_eq!(v, ev);
        let s = represent_from_bits(&v);
        assert_eq!(x,s);


        let x = 7;
        let v = represent_into_bits(x);
        let ev = [1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        assert_eq!(v, ev);
        let s = represent_from_bits(&v);
        assert_eq!(x,s);

        let x = (1u64<<60) + 3;
        let v = represent_into_bits(x);
        let ev = [1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0];
        assert_eq!(v, ev);
        let s = represent_from_bits(&v);
        assert_eq!(x,s);
    }
}