use std::{
    cmp::min,
    sync::{Arc, Mutex},
    time::Instant,
    vec,
};

struct ParallelIterator<T> {
    iter: Vec<T>,
}

// acaquire current number of cpus
static NUM_THREADS: usize = 32;

impl<T> ParallelIterator<T>
where
    T: Send + Sync + Copy + PartialEq + std::fmt::Debug,
{
    fn find(&self, search_for: T) -> Option<usize> {
        let chunk_size = (self.iter.len() / NUM_THREADS) + 1;
        let result = Arc::new(Mutex::new(None));

        std::thread::scope(|scope| {
            for (index, chunk) in self.iter.chunks(chunk_size).enumerate() {
                let result = Arc::clone(&result);
                let chunk = chunk.to_vec();

                scope.spawn(move || {
                    let pos = chunk.iter().position(|i| *i == search_for);
                    if let Some(pos) = pos {
                        let mut res = result.lock().unwrap();
                        *res = Some(match *res {
                            Some(prev_index) => min(prev_index, index * chunk_size + pos),
                            None => index * chunk_size + pos,
                        });
                    }
                });
            }
        });
        Arc::try_unwrap(result)
            .expect("Lock still has multiple owners")
            .into_inner()
            .unwrap()
    }

    fn find_all(&self, search_for: T) -> Vec<usize> {
        let chunk_size = (self.iter.len() / NUM_THREADS) + 1;
        let result = Arc::new(Mutex::new(Vec::new()));

        std::thread::scope(|scope| {
            for (index, chunk) in self.iter.chunks(chunk_size).enumerate() {
                let result = Arc::clone(&result);
                let chunk = chunk.to_vec();

                scope.spawn(move || {
                    let mut res = Vec::new();
                    for (i, &num) in chunk.iter().enumerate() {
                        if num == search_for {
                            res.push(index * chunk_size + i);
                        }
                    }
                    let mut result = result.lock().unwrap();
                    result.extend(res);
                });
            }
        });
        Arc::try_unwrap(result)
            .expect("Lock still has multiple owners")
            .into_inner()
            .unwrap()
    }

    fn map<U, F>(&self, f: F) -> Vec<U>
    where
        U: Send + Sync + Copy + PartialEq + std::fmt::Debug,
        F: Fn(T) -> U + Send + Sync + Copy,
    {
        let chunk_size = (self.iter.len() / NUM_THREADS) + 1;
        let result = Arc::new(Mutex::new(vec![None; NUM_THREADS]));

        std::thread::scope(|scope| {
            for (i, chunk) in self.iter.chunks(chunk_size).enumerate() {
                let result = Arc::clone(&result);
                let chunk = chunk.to_vec();

                scope.spawn(move || {
                    let res: Vec<U> = chunk.into_iter().map(|i| f(i)).collect();
                    let mut result = result.lock().unwrap();
                    result[i] = Some(res);
                });
            }
        });

        let mut final_result = Vec::new();
        let result = Arc::try_unwrap(result)
            .expect("Lock still has multiple owners")
            .into_inner()
            .unwrap();
        for chunk in result {
            if let Some(mut res) = chunk {
                final_result.append(&mut res);
            }
        }
        final_result
    }
}

trait IntoParIter<T> {
    fn into_par_iter(self) -> ParallelIterator<T>;
}

impl<T> IntoParIter<T> for Vec<T> {
    fn into_par_iter(self) -> ParallelIterator<T> {
        ParallelIterator { iter: self }
    }
}

fn time<T>(test_name: &str, data: T, f: impl Fn(T) -> ()) {
    let before = Instant::now();
    f(data);
    println!("Test {test_name}: {:.2?}", before.elapsed());
}

fn main() {
    let test_length = 1000000;
    let find_index = 777722;

    let mut nums: Vec<i32> = vec![0, 1].repeat(test_length / 2);
    nums[find_index] = 2;
    nums[find_index + 100] = 2;
    nums[find_index + 1000] = 2;

    time("find_normal", nums.clone(), |nums| {
        let mut iter = nums.into_iter();
        assert_eq!(iter.position(|i| i == 2), Some(find_index));
    });

    time("find_parallel", nums.clone(), |nums| {
        let iter = nums.into_par_iter();
        assert_eq!(iter.find(2), Some(find_index));
    });

    let data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let my_struct = ParallelIterator { iter: data.clone() };

    let result = my_struct.map(|x| x * 2);
    println!("{:?}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_large() {
        let test_length = 10_000_000;
        let find_index = 7_777_222;

        let mut nums: Vec<i32> = vec![0, 1].repeat(test_length / 2);
        nums[find_index] = 2;
        nums[find_index + 100] = 2;
        nums[find_index + 1000] = 2;

        let iter = ParallelIterator { iter: nums.clone() };
        assert_eq!(iter.find(2), Some(find_index));
    }

    #[test]
    fn test_find_all_large() {
        let test_length = 10_000_000;
        let find_indices = vec![7_777_222, 7_777_322, 7_778_222];

        let mut nums: Vec<i32> = vec![0, 1].repeat(test_length / 2);
        for &index in &find_indices {
            nums[index] = 2;
        }

        let iter = ParallelIterator { iter: nums.clone() };
        assert_eq!(iter.find_all(2), find_indices);
    }

    #[test]
    fn test_performance_find() {
        let test_length = 10_000_000;
        let find_index = 7_777_222;

        let mut nums: Vec<i32> = vec![0, 1].repeat(test_length / 2);
        nums[find_index] = 2;

        time("test_performance_find", nums.clone(), |nums| {
            let iter = ParallelIterator { iter: nums };
            assert_eq!(iter.find(2), Some(find_index));
        });
    }

    #[test]
    fn test_performance_find_all() {
        let test_length = 10_000_000;
        let find_indices = vec![7_777_222, 7_777_322, 7_778_222];

        let mut nums: Vec<i32> = vec![0, 1].repeat(test_length / 2);
        for &index in &find_indices {
            nums[index] = 2;
        }

        time("test_performance_find_all", nums.clone(), |nums| {
            let iter = ParallelIterator { iter: nums };
            assert_eq!(iter.find_all(2), find_indices);
        });
    }

    fn double(x: i32) -> i32 {
        x * 2
    }

    #[test]
    fn test_map() {
        let data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let my_struct = ParallelIterator { iter: data.clone() };

        let result = my_struct.map(double);

        // The result should be each element doubled
        let expected: Vec<i32> = data.into_iter().map(double).collect();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_map_large_dataset() {
        let data_size = 10000000; // 1 million elements
        let data: Vec<i32> = (0..data_size).collect();
        let my_struct = ParallelIterator { iter: data.clone() };

        let result = my_struct.map(double);

        // The result should be each element doubled
        let expected: Vec<i32> = data.into_iter().map(double).collect();
        assert_eq!(result, expected);
    }
}
