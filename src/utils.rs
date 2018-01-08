pub fn min<T:PartialOrd>(a:T,b:T)->T { if a<b{a}else{b} }
pub fn max<T:PartialOrd>(a:T,b:T)->T { if a>b{a}else{b} }

pub fn split_vec_mut_around<T>(vec: &mut Vec<T>, index: usize) -> (&mut [T], &mut T, &mut [T]) {
	let (a, c) = vec.split_at_mut(index);
	let (b, c) = c.split_at_mut(1);
	(a, &mut b[0], c)
}

#[cfg(test)]
mod tests {
    use utils::split_vec_mut_around;

	#[test]
	fn test_1() {
        let mut v = vec![1, 2, 3];
        let (a, b, c) = split_vec_mut_around(&mut v, 0);
        assert_eq!(*b, 1);
	}

    #[test]
	fn test_2() {
        let mut v = vec![1, 2, 3];
        let (a, b, c) = split_vec_mut_around(&mut v, 1);
        assert_eq!(*b, 2);
	}

    #[test]
	fn test_3() {
        let mut v = vec![1, 2, 3];
        let (a, b, c) = split_vec_mut_around(&mut v, 2);
        assert_eq!(*b, 3);
	}
}