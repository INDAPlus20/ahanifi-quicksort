use std::{cmp, io::{self, Read}, mem::{self, swap}};

fn main() {
    

    let mut line = String::with_capacity(500_000);
    io::stdin().lock().read_to_string(&mut line);

    // thank u viola
    let mut array: Vec<isize> = line
        .split_whitespace()
        .skip(1) // <-- SKIP LENGTH PARAM
        .map(|_value| _value.parse::<isize>().unwrap())
        .collect();


    
    quicksort(&mut array[..]);

    for elem in array{
        print!("{} ",elem);
    }
  
}

fn quicksort(array: &mut [isize]) {
        let len: usize = array.len();
        if len <= 1 {
            return;
        }
    
        let pivot: usize = 0;
        array.swap(pivot, len / 2);
    
        let mut left: usize = 1;
        let mut right: usize = len - 1;
    
        loop {
            while left < len && array[left] <= array[pivot] {
                left += 1
            }
            while right > 0 && array[right] >= array[pivot] {
                right -= 1
            }
            if left >= right {
                break;
            }
            array.swap(left, right);
            left += 1;
            right -= 1;
        }
        array.swap(pivot, right);

        
        let left_partition= &mut array[..cmp::min(left - 1, right)];
        quicksort(left_partition);
        let right_partition=&mut array[cmp::max(left, right + 1)..];
        quicksort(right_partition);
}

fn pause() {
    let mut _t = String::new();
    std::io::stdin().read_line(&mut _t).unwrap();
}

#[cfg(test)]
mod tests {
    use std::borrow::Borrow;

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use rand::{self, Rng};
    use rand::distributions::{Standard};
    #[test]
    fn test_random_sort() {
        let mut rng = rand::thread_rng();

        for _ in 0 .. 10_00 {
            let len: usize=rng.gen_range(2..1000);
            let mut array:Vec<isize>=vec![0isize;len];
            rng.fill(&mut array[..]);
            quicksort(&mut array[..]);
            
            println!("sorted ?? {:?}",array);
            
            for i in 1 .. array.len() {
                println!("{} | {}",array[i-1],array[i]);
                assert!(array[i-1] <= array[i])
            }
        }
    }
}