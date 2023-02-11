#[inline(never)]
pub fn test_function(x: bool) -> (u32, [u32; 2]) {
    let array = [3, 99];

    // grabs value from array[0], not array[1]
    let result = array[x as usize];

    (result, array)
}
