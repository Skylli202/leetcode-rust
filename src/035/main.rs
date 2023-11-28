pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
    let mut insertion_index: usize = nums.len();
    for (i, num) in nums.into_iter().enumerate() {
        if num == target {
            return i as i32;
        }

        if num >= target && i < insertion_index {
            insertion_index = i;
        }
    }
    insertion_index as i32
}

#[test]
fn example_001() {
    let nums: Vec<i32> = vec![1, 3, 5, 6];
    let target: i32 = 5;
    let expected_result: i32 = 2;
    assert_eq!(search_insert(nums, target), expected_result);
}

#[test]
fn example_002() {
    let nums: Vec<i32> = vec![1, 3, 5, 6];
    let target: i32 = 2;
    let expected_result: i32 = 1;
    assert_eq!(search_insert(nums, target), expected_result);
}

#[test]
fn example_003() {
    let nums: Vec<i32> = vec![1, 3, 5, 6];
    let target: i32 = 7;
    let expected_result: i32 = 4;
    assert_eq!(search_insert(nums, target), expected_result);
}

fn main() {}
