#[inline(never)]
pub fn test_function() -> (usize, u32, [u32; 2]) {
    let mut array = [3; 2];

    let mut index = 0;
    let mut result = 0;

    for x in [1, 2] {
        if x == 1 {
            array[1] = 99;
        }
        else {
            index = if x == 2 { 1 } else { 0 };

            // grabs value from array[0], not array[1]
            result = array[index];
        }
    }

    (index, result, array)
}
