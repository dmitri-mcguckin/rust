use unique::Unique;

fn main() {
    let mut nums = vec![];
    let pred: fn(&&usize) -> bool = |&&n| n % 2 == 0;

    assert_eq!(None, nums.iter().unique(pred));

    nums.push(1);

    assert_eq!(None, nums.iter().unique(pred));

    nums.push(0);

    assert_eq!(Some(&0), nums.iter().unique(pred));

    nums.push(3);
    nums.push(2);
    nums.push(5);

    assert_eq!(None, nums.iter().unique(pred));
}
