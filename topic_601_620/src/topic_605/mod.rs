
pub fn can_place_flowers(flowerbed: Vec<i32>, n: i32) -> bool {
    let mut flowerbed = flowerbed;
    let mut count = 0;
    if flowerbed[0] == 0 {
        count += 1;
        flowerbed[0] = 1;
    }
    for i in 1..flowerbed.len() {
        if flowerbed[i - 1] == 1 && flowerbed[i] == 1 {
            count -= 1;
            flowerbed[i - 1] = 0;
        }
        if flowerbed[i - 1] == 0 && flowerbed[i] == 0 {
            count += 1;
            flowerbed[i] = 1;
        }
    }
    count >= n
}