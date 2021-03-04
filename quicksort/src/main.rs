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

    let len=array.len();
    
    quicksort(&mut array[..], 0,len-1);

    for elem in array{
        print!("{} ",elem);
    }
  
}


fn partition(array:&mut [isize],lo:usize,hi:usize)-> usize{

    let pivot=array[(lo+hi)/2];
    let mut left: isize = lo as isize-1;
    let mut right: usize = hi+1;

    loop {
        left+=1;
        while array[left as usize] < pivot {
            left += 1;
        }
        right -= 1;
        while array[right] > pivot {
            right -= 1;
        }
        if left as usize >= right{
            return right;
        }
        array.swap(left as usize, right);
        
        
    }
}

fn quicksort(array: &mut [isize],lo:usize,hi:usize) {
       if lo<hi{
        let pivot=partition(array, lo, hi);
        quicksort(array, lo, pivot);
        quicksort(array, pivot+1, hi)
       }
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
            quicksort(&mut array[..],0,len-1);
            
            println!("sorted ?? {:?}",array);
            
            for i in 1 .. array.len() {
                println!("{} | {}",array[i-1],array[i]);
                assert!(array[i-1] <= array[i])
            }
        }
    }
}