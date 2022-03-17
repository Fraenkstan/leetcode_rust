

pub fn find_lu_slength(a: String, b: String) -> i32 {
    if a.len() == b.len() {
        -1i32
    }
    a.len().max(b.len()) as i32
}