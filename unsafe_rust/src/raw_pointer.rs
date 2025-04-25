use std::slice;
pub fn raw_pointer_example() {
    let mut num = 5;

    let r1 = &raw const num;
    let r2 = &raw mut num;
    unsafe {
        let new_num = *r2+*r1;
        println!("Sum of raw pointers: {new_num}");
    }
}

pub fn raw_pointer_to_random_direction() {
    let address = 0x012345usize;
    let _r = address as *const i32;
}

pub fn unsafe_block(){
    let mut num = 5;

    let r1 = &raw const num;
    let r2 = &raw mut num;

    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }
}

pub fn common_split_mut(){
    let mut v = vec![1, 2, 3, 4, 5, 6];

    let r = &mut v[..];

    let (a, b) = r.split_at_mut(3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);
}
pub fn use_split_at_mut(){
    let mut vector = vec![1, 2, 3, 4, 5, 6];
    let (left, right) = split_at_mut(&mut vector, 3);
    println!("{:?} {:?}",left,right);
}
fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    let ptr = values.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

pub fn random_memory_slice(){
    use std::slice;

    let address = 0x01234usize;
    let r = address as *mut i32;

    let values: &[i32] = unsafe { slice::from_raw_parts_mut(r, 10000) };

    println!("{:?}",values);
}