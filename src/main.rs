pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32>
{
    if numbers.len() < 2
    {
        return vec![];
    }

    let mut left: usize = 0;
    let mut right: usize = numbers.len() - 1;

    while left < right
    {
        let sum = numbers[left] + numbers[right];
        if sum == target
        {
            return vec![left as i32 + 1, right as i32 + 1];
        }
        else if sum < target
        {
            left += 1;
        }
        else
        {
            right -= 1;
        }
    }

    vec![] 
}

fn main() {
    let numbers = vec![2, 7, 11, 15];
    let target = 9;
    let result = two_sum(numbers, target);
    println!("{:?}", result);

    let numbers = vec![2, 3, 4];
    let target = 6;
    let result = two_sum(numbers, target);
    println!("{:?}", result);

    let numbers: Vec<i32> = vec![1, 3, 6, 11, 23, 56, 78, 99];
    let target = 34;
    let result = two_sum(numbers, target);
    println!("{:?}", result);
}
