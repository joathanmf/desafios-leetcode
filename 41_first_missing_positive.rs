// 41. First Missing Positive
// https://leetcode.com/problems/first-missing-positive/

fn first_missing_positive_v1(mut nums: Vec<i32>) -> i32 {
    let mut i = 0;
    let mut n = 1;

    nums.sort();

    while i < nums.len() {
        if nums[i] > 0 {
            if nums[i] != n {
                return n;
            }

            n += 1;
        }

        i += 1;
    }

    return n;
}

// Leetcode
fn first_missing_positive_v2(mut nums: Vec<i32>) -> i32 {
    let mut i = 0;
    let len = nums.len();
    
    while i < len {
        let cur = nums[i];

        if cur > 0 && cur <= len as i32 {
            if nums[(cur - 1) as usize] != cur {
                let aux = cur;
                nums[i] = nums[(cur - 1) as usize];
                nums[(cur - 1) as usize] = aux;
    
                continue;
            }
        }

        i += 1;
    }

    for i in 0..len {
        if nums[i] != (i + 1) as i32 {
            return (i + 1) as i32;
        }
    }

    return (len + 1) as i32;
}

fn main() {
    let nums = vec![3, 4, -1, 1, 2, 5, 6, -2, 7];

    println!("Primeiro positivo faltando (v1): {}", first_missing_positive_v1(nums.clone()));
    println!("Primeiro positivo faltando (v2): {}", first_missing_positive_v2(nums));
}
