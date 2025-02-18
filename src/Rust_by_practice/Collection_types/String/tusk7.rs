use std::mem;

#[test]
pub fn main() {
    let story = String::from("Rust By Practice");

    let mut story = mem::ManuallyDrop::new(story);

    let ptr = story.as_ptr();
    let len = story.len();
    let capacity = story.capacity();

    assert_eq!(16, len);

    let s = unsafe { String::from_raw_parts(ptr as *mut u8, len, capacity) };

    assert_eq!(*story, s);

    println!("Success!");
}
