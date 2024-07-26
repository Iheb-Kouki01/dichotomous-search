mod search;
#[no_mangle]

pub extern "C" fn searchc(a:u32,ptr :*const u32,len:usize)-> usize {
    
    use std::vec;
    use search::search;
    if ptr.is_null() {
        return 0;
    };

    let slice = unsafe {
        std::slice::from_raw_parts(ptr, len)
    };

    let mut v:Vec<u32>=vec![];

    for &i in slice.iter() {
        v.push(i);
    }

    return search(a, v) ;
}