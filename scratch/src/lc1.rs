use std::collections::HashMap;
use rand::seq::SliceRandom;
use rand::thread_rng;
use rand::Rng;
use std::time::SystemTime;
use std::time::Instant;

fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> 
    {
        // [1] prepare numbers for binary search by 
        //     saving original indices and sorting
        let mut nums: Vec<(usize, i32)> = 
            nums.into_iter()
                .enumerate()
                .collect::<Vec<(usize, i32)>>();

        nums.sort_unstable_by_key(|&(_, n)| n);
        
        // [2] we perform linear scan for the first number
        //     and do binary search for the second number
        for (k, (i, ni)) in nums.iter().enumerate()
        {
            // [3] to prevent duplication, slices start from k+1
            match nums[k+1..].binary_search_by_key(&(target - *ni), |&(_, nj)| nj)
            {
                Ok(jj) => return vec![*i as i32, nums[jj+k+1].0 as i32],
                Err(_) => {}
            }
        }

        unreachable!("Error: this place should be unreachable");
        return vec![0,0];
    }



fn two_sum_me(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut seen: HashMap<i32, i32> = HashMap::new();
       for (index, val) in nums.iter().enumerate() {
        let x  = seen.entry((target-val));
        match x {
            std::collections::hash_map::Entry::Occupied(o) => {
                return vec![*o.get(), index as i32];
            },
            std::collections::hash_map::Entry::Vacant(v) => {
                v.insert(index as i32);
            }
        }

        //    if let Some(x) = seen.get(&(target-val)) {
        //        return vec![*x, index as i32];
        //    }
        //    seen.insert(val, index as i32);
       }
       return vec![];
    }



    
    fn generate_test_case(n: usize) -> Vec<i32> {
        let mut rng = thread_rng();
    
        let mut out:Vec<i32> = vec![];

        for i in 0..n {
            out.push(0);
        }
    
        // Randomly choose two indices
        let indices: Vec<usize> = (5..n).collect();
        let chosen_indices: Vec<usize> = indices.choose_multiple(&mut rng, 2).cloned().collect();
    
        // Ensure that the selected numbers at the chosen indices add up to the target
        let (index1, index2) = (chosen_indices[0], chosen_indices[1]);
        out[index1] = 2;
        out[index2] = 1;
    
        out
    }
    

    fn timeit<F: FnOnce() -> T, T>(f: F) -> T {
        let start = Instant::now();
        let result = f();
        let end = Instant::now();
        let duration = end.duration_since(start);
        println!("it took {}", duration.as_micros());
        result
      }

    fn main() {
        //target 3
        for i in 0..10 {
            let n = 1000000; 
            let vec = generate_test_case(n);
            let vec1  = vec.clone();
            let vec2 = vec.clone();
            print!("Reference Trial {}",i);
            let closure_with_move = || two_sum(vec1, 3);
            timeit(closure_with_move);
            print!("Me Trial {}",i);
            let closure_with_borrow = || two_sum(vec2.clone(), 3);
            timeit(closure_with_borrow);

        }
        
    }
