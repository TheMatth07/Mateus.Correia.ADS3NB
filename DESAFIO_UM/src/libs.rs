pub unsafe fn sum_array(ptr: *const i32, len: usize) -> i32 {
    let mut product = 1;
    //sempre começa no 0 e não 1
    for i in 0..len {
    unsafe {
        product *= *ptr.offset(i as isize);
        }
    }
    //retorna o valor de product
    return product;
    
}

#[cfg(test)]
mod tests {
}
//remoção do "use super::*;""
    
    fn main() {
        let arr = [1, 2, 3, 4];
        let product = unsafe { sum_array(arr.as_ptr(), arr.len()) };
        assert_eq!(product, 24);

    }
