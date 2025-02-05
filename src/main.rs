fn largest(list: &[i32]) -> &i32{
    let mut largest = &list[0];
    for num in list {
        if num > largest {
            largest = num;
        }
    }
    largest
}

fn main() {
    println!("Hello, world!");

    let nums = vec![1, 11, 11, 2, 111, 123, 2, 3, 4, 5];
    let nums_slice = &nums[1..3];
    let num = largest(&nums_slice);
    let num2 = largest(&nums);

    println!("largest num in {nums:?} is {num2}");
    println!("largest num in {nums_slice:?} is {num}");
}
