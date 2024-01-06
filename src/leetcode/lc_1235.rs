pub struct Solution {}

impl Solution {
    fn schedule(i: usize, jobs: &Vec<Vec<i32>>, cache: &mut Vec<i32>) -> i32 {
        if i >= jobs.len() {
            0
        } else if cache[i] != -1 {
            cache[i]
        } else {
            let z = jobs.partition_point(|x| x[0] < jobs[i][1]);
            cache[i] = Solution::schedule(i+1, jobs, cache).max(jobs[i][2] + Solution::schedule(z, jobs, cache));
            cache[i]
        }
    }

    pub fn job_scheduling(start_time: Vec<i32>, end_time: Vec<i32>, profit: Vec<i32>) -> i32 {
        let mut cache = vec![-1; start_time.len()];
        let mut jobs = (0..start_time.len()).map(|i| vec![start_time[i], end_time[i], profit[i]]).collect::<Vec<Vec<_>>>();
        jobs.sort_by_key(|x| (x[0], x[1], -x[2]));
        Solution::schedule(0, &jobs, &mut cache)
    }
}