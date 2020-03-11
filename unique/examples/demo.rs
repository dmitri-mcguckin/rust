use unique::Unique;

fn main() {
    let mut nums = vec![];
    let pred: fn(&&usize) -> bool = |&&n| n % 2 == 0;

    let mut uniques = nums.iter().unique(pred);
    assert_eq!(None, uniques);
    println!("Test 1: Array: {:?}, Uniques: {:?}", nums, uniques);

    nums.push(1);

    uniques = nums.iter().unique(pred);
    assert_eq!(None, uniques);
    println!("Test 2: Array: {:?}, Uniques: {:?}", nums, uniques);

    nums.push(0);

    uniques = nums.iter().unique(pred);
    assert_eq!(Some(&0), uniques);
    println!("Test 3: Array: {:?}, Uniques: {:?}", nums, uniques);

    nums.push(3);
    nums.push(2);
    nums.push(5);

    uniques = nums.iter().unique(pred);
    assert_eq!(None, uniques);
    println!("Test 4: Array: {:?}, Uniques: {:?}", nums, uniques);
}
