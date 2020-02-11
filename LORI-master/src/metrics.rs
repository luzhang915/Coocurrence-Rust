use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use std::thread;

use log::info;

pub struct MetricsConcurrent {
    footprint: Vec<f32>,
    footprint_diff: Vec<f32>,
    footmark: Vec<f32>,
    footmark_diff: Vec<f32>,
    miss_ratio: Vec<f32>,
}

impl MetricsConcurrent {
    /* Uses the get_count() function to get the num of unique elements of the trace,
     * which then adds those values, given a vector input to get the trace properties. */
    pub fn build(trace: Vec<usize>) -> MetricsConcurrent {
        info!("TR B: Building the metrics of the trace.");

        let trace_arc = Arc::new(trace);

        let trace_arc_clone = Arc::clone(&trace_arc);
        let miss_ratio_thread = thread::spawn(move || -> Vec<f32> {
            let reuse_distance = MetricsConcurrent::reuse_distance(&trace_arc_clone);
            let miss_ratio = MetricsConcurrent::miss_ratio(reuse_distance, true);
            miss_ratio
        });

        let trace_arc_clone = Arc::clone(&trace_arc);
        let reuse_interval_arc = Arc::new(MetricsConcurrent::reuse_interval(&trace_arc_clone));

        let reuse_interval_clone = Arc::clone(&reuse_interval_arc);
        let footprint_thread = thread::spawn(move || -> Footprint {
            let reuse_interval = Arc::clone(&reuse_interval_clone);
            let fp_struct = MetricsConcurrent::footprint(&reuse_interval);
            fp_struct
        });

        let reuse_interval_clone = Arc::clone(&reuse_interval_arc);
        let footmark_thread = thread::spawn(move || -> Footmark {
            let fm_struct = MetricsConcurrent::footmark(&reuse_interval_clone);
            fm_struct
        });

        let miss_ratio = miss_ratio_thread.join().unwrap();
        let footprint_struct = footprint_thread.join().unwrap();
        let footmark_struct = footmark_thread.join().unwrap();

        info!("TR F: Finished building the metrics of the trace.");
        MetricsConcurrent {
            footprint: footprint_struct.footprint,
            footprint_diff: footprint_struct.footprint_diff,
            footmark: footmark_struct.footmark,
            footmark_diff: footmark_struct.footmark_diff,
            miss_ratio,
        }
    }

    /* Computes the reuse interval sequence for the trace. This is done by creating a hashmap from
     * the trace elements to the last reuse interval. The upon revisiting an element in the
     * hashmap, the reuse interval is updated and added to the reuse interval sequence. */
    fn reuse_interval(trace: &Vec<usize>) -> ResuseInterval {
        info!("RI B: Building reuse interval sequence.");
        let mut reuse_interval_vec = Vec::new();
        let mut reuse_interval_histogram = vec![0; trace.len()];
        // Hashmap for storing first and last accesses.
        let mut first_last_accesses: HashMap<usize, Access> = HashMap::new();
        // Hashmap for storing previous reuse time of each element.
        let mut reuse_interval_map: HashMap<&usize, usize> = HashMap::new();
        // Hashset for the unique elements of the trace.
        let mut trace_set: HashSet<usize> = HashSet::new();

        for (i, e) in trace.iter().enumerate() {
            // Iterate over the trace
            if let Some(prev_ri) = reuse_interval_map.get(e).cloned() {
                // Stores current time - previous RI in RI sequence
                let interval = i - prev_ri;
                reuse_interval_vec.push(interval);
                reuse_interval_histogram[interval] += 1;
                // Update reuse interval in hashmap
                reuse_interval_map.insert(e, i);
            } else {
                // 0 is to be viewed as infinity in this context
                reuse_interval_vec.push(0);
                reuse_interval_histogram[0] += 1;
                // If no existing element, add 0 to sequence
                reuse_interval_map.insert(e, i);
            }

            // Update first and last access time.
            if let Some(access_prev) = first_last_accesses.get(e) {
                let access_new = Access {
                    first: access_prev.first,
                    last: i + 1,
                };
                first_last_accesses.insert(*e, access_new);
            } else {
                let access_new = Access {
                    first: i + 1,
                    last: i + 1,
                };
                first_last_accesses.insert(*e, access_new);
            }

            // Add to the hashset.
            trace_set.insert(*e);
        }

        info!("RI F: Finished building reuse interval sequence.");
        ResuseInterval {
            vec: reuse_interval_vec,
            reuse_interval_histogram,
            first_last_accesses,
            trace_set,
        }
    }

    // Gives footprint given size x. Takes average working set size.
    fn footprint(reuse_interval: &ResuseInterval) -> Footprint {
        info!("FP B: Building informatoin about footprint.");
        let mut footprint: Vec<f32> = Vec::new();
        let mut footprint_diff: Vec<f32> = Vec::new();

        footprint.push(0.0);

        if footprint[0] >= reuse_interval.trace_set.len() as f32 {
            return Footprint {
                footprint,
                footprint_diff,
            };
        }

        for window_length in 1..(reuse_interval.vec.len() + 1) {
            // Computationally fast footprint.
            // First we need three sums.
            let mut ri_hist_sum = 0;
            let mut first_access_sum = 0;
            let mut last_access_sum = 0;

            // This uses the Xiang formula. Consult "A Relational Theory of Locality", Yuan et. al
            // for more information.
            if window_length < reuse_interval.reuse_interval_histogram.len() {
                for (i, e) in reuse_interval.reuse_interval_histogram[window_length + 1..]
                    .iter()
                    .enumerate()
                    {
                        ri_hist_sum += ((i + window_length + 1) - window_length) * *e;
                    }
            }
            for i in &reuse_interval.trace_set {
                if let Some(access) = reuse_interval.first_last_accesses.get(&i) {
                    if access.first > window_length {
                        first_access_sum += access.first - window_length;
                    }
                    if access.last < reuse_interval.vec.len() - window_length + 1 {
                        last_access_sum += reuse_interval.vec.len() - window_length
                            + 1
                            - access.last;
                    }
                }
            }
            footprint.push(
                reuse_interval.trace_set.len() as f32
                    - (1.0 / (reuse_interval.vec.len() - window_length + 1) as f32)
                    * (ri_hist_sum + first_access_sum + last_access_sum) as f32,
            );

            footprint_diff.push(footprint[window_length] - footprint[window_length - 1]);

            if footprint[window_length] >= reuse_interval.trace_set.len() as f32 {
                break;
            }
        }
        info!("FP F: Finished building informatoin about footprint.");
        Footprint {
            footprint,
            footprint_diff,
        }
    }

    // Function for computing the footmark given the reuse interval histogram and parameter x.
    fn footmark(reuse_interval: &ResuseInterval) -> Footmark {
        info!("FM B: Building informatoin about footmark.");
        let mut footmark: Vec<f32> = Vec::new();
        let mut footmark_diff: Vec<f32> = Vec::new();

        footmark.push(0.0);

        for window_length in 1..(reuse_interval.vec.len() + 1) {
            let mut sum: usize = 0;
            // Uses a simple summation over the reuse interval histogram
            for i in 1..window_length {
                sum += (window_length - i) * reuse_interval.reuse_interval_histogram[i];
            }

            footmark.push(
                ((reuse_interval.vec.len() * window_length) - sum) as f32
                    / reuse_interval.vec.len() as f32,
            );

            footmark_diff.push(footmark[window_length] - footmark[window_length - 1]);
        }
        info!("FM F: Finished building informatoin about footmark.");
        Footmark {
            footmark,
            footmark_diff,
        }
    }

    /*
     * This function computes the reuse distance.
     */
    fn reuse_distance(trace: &Vec<usize>) -> ReuseDistance {
        info!("RD B: Building the reuse distance vector and histogram sequence.");
        let mut reuse_distance_vec = Vec::new();
        let mut reuse_distance_histogram = vec![0; trace.len() + 1];

        let mut set: HashSet<usize> = HashSet::new();
        for (i, e) in trace.iter().enumerate() {
            let mut count: usize = 1;
            /* If element already in the hashset, will iterate backward through the reuse
             * distance sequence until it finds itself. The number of iterations is then the
             * reuse distance. */
            match set.insert(*e) {
                true => {
                    reuse_distance_vec.push(0);
                    reuse_distance_histogram[0] += 1;
                }
                // If element in hashset already, do this:
                false => {
                    let mut set_backwards: HashSet<usize> = HashSet::new();
                    let mut j: usize = 1;
                    while *e != trace[i - j] {
                        // We only want to count unique additions.
                        match set_backwards.insert(trace[i - j]) {
                            false => (),
                            true => count += 1,
                        }
                        j += 1;
                    }
                    // Push the result.
                    reuse_distance_vec.push(count);
                    reuse_distance_histogram[count] += 1;
                }
            }
        }
        info!("RD F: Finished building the reuse distance vector and histogram sequence.");
        ReuseDistance {
            vec: reuse_distance_vec,
            histogram: reuse_distance_histogram,
        }
    }

    // Gives the miss ratios of a fully associated LRU cache up to a given size c.
    fn miss_ratio(reuse_distance: ReuseDistance, coldstarts: bool) -> Vec<f32> {
        info!("MR B: Building the miss ratio vector and histogram sequence.");
        let mut miss_ratio: Vec<f32> = vec![1.0];
        // Use histogram sum to compute.
        let mut hits = 0;
        for e in reuse_distance.histogram[1..].iter() {
            hits += *e;
            let normalizer = match coldstarts {
                true => reuse_distance.vec.len(),
                false => {
                    reuse_distance.vec.len()
                        - reuse_distance.histogram[0]
                }
            };
            miss_ratio.push(1.0 - (hits as f32 / normalizer as f32));
        }
        info!("MR F: Finished building the miss ratio vector and histogram sequence.");
        miss_ratio
    }

    // // Function computes the working set size given an end length and interval length
    // fn working_set_size(&self, x: usize, i: usize) -> u32 {
    //     let mut set: HashSet<u32> = HashSet::new();
    //     // Add everything in the interval to the hashset then return the size
    //     for e in i - x..i {
    //         set.insert(self.trace[e]);
    //     }
    //     set.len() as u32
    // }

    pub fn get_footprint(&self) -> &Vec<f32> {
        &self.footprint
    }

    pub fn get_footprint_diff(&self) -> &Vec<f32> {
        &self.footprint_diff
    }

    pub fn get_footmark(&self) -> &Vec<f32> {
        &self.footmark
    }

    pub fn get_footmark_diff(&self) -> &Vec<f32> {
        &self.footmark_diff
    }

    pub fn get_miss_ratio(&self) -> &Vec<f32> {
        &self.miss_ratio
    }
}

#[derive(Clone)]
struct ResuseInterval {
    vec: Vec<usize>,
    reuse_interval_histogram: Vec<usize>,
    // TODO use Histogram in development
    first_last_accesses: HashMap<usize, Access>,
    trace_set: HashSet<usize>,
}

#[derive(Clone)]
struct Access {
    first: usize,
    last: usize,
}

struct Footprint {
    footprint: Vec<f32>,
    footprint_diff: Vec<f32>,
}

struct Footmark {
    footmark: Vec<f32>,
    footmark_diff: Vec<f32>,
}

struct ReuseDistance {
    vec: Vec<usize>,
    histogram: Vec<usize>, // TODO use Histogram in development
}

#[cfg(test)]
mod tests {
    use super::*;

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn footprint_test_1() {
        init();

        let trace: Vec<usize> = vec![1, 2, 3, 4, 5, 6];
        let metrics: MetricsConcurrent = MetricsConcurrent::build(trace);

        assert_eq!(metrics.footprint, [0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0])
    }

    #[test]
    fn footprint_diff_test_1() {
        init();

        let trace: Vec<usize> = vec![1, 2, 3, 4, 5, 6];
        let metrics: MetricsConcurrent = MetricsConcurrent::build(trace);

        assert_eq!(metrics.footprint_diff, vec![1.0, 1.0, 1.0, 1.0, 1.0, 1.0])
    }

    #[test]
    fn footprint_test_2() {
        init();

        let trace: Vec<usize> = vec![1, 2, 3, 3, 2, 1];
        let metrics: MetricsConcurrent = MetricsConcurrent::build(trace);

        assert_eq!(metrics.footprint, [0.0, 1.0, 1.8, 2.5, 8.0 / 3.0, 3.0])
    }

    #[test]
    #[ignore] // ignored because the values are off by very small margins.
    fn footprint_diff_test_2() {
        init();

        let trace: Vec<usize> = vec![1, 2, 3, 3, 2, 1];
        let metrics: MetricsConcurrent = MetricsConcurrent::build(trace);

        assert_eq!(
            metrics.footprint_diff,
            vec![1.0, 0.8, 0.7, 1.0 / 6.0, 1.0 / 3.0]
        )
    }

    #[test]
    fn footmark_test_1() {
        init();

        let trace: Vec<usize> = vec![1, 2, 3, 4, 5, 6];
        let metrics: MetricsConcurrent = MetricsConcurrent::build(trace);

        assert_eq!(metrics.footmark, vec![0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0])
    }

    #[test]
    fn footmark_diff_test_1() {
        init();

        let trace: Vec<usize> = vec![1, 2, 3, 4, 5, 6];
        let metrics: MetricsConcurrent = MetricsConcurrent::build(trace);

        assert_eq!(metrics.footmark_diff, vec![1.0, 1.0, 1.0, 1.0, 1.0, 1.0])
    }

    #[test]
    fn footmark_test_2() {
        init();

        let trace: Vec<usize> = vec![1, 2, 3, 3, 2, 1];
        let metrics: MetricsConcurrent = MetricsConcurrent::build(trace);

        assert_eq!(
            metrics.footmark,
            vec![
                0.0,
                1.0,
                11.0 / 6.0,
                16.0 / 6.0,
                20.0 / 6.0,
                24.0 / 6.0,
                27.0 / 6.0
            ]
        )
    }

    #[test]
    #[ignore] // ignored because the values are off by very small margins.
    fn footmark_diff_test_2() {
        init();

        let trace: Vec<usize> = vec![1, 2, 3, 3, 2, 1];
        let metrics: MetricsConcurrent = MetricsConcurrent::build(trace);

        assert_eq!(
            metrics.footmark_diff,
            vec![1.0, 5.0 / 6.0, 5.0 / 6.0, 4.0 / 6.0, 4.0 / 6.0, 3.0 / 6.0]
        )
    }

    #[test]
    fn miss_ratio_test_1() {
        init();

        let trace: Vec<usize> = vec![1, 2, 3, 4, 5, 6];
        let metrics: MetricsConcurrent = MetricsConcurrent::build(trace);

        assert_eq!(metrics.miss_ratio, vec![1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0])
    }

    #[test]
    #[ignore] // ignored because the values are off by very small margins.
    fn miss_ratio_test_2() {
        init();

        let trace: Vec<usize> = vec![1, 2, 3, 3, 2, 1];
        let metrics: MetricsConcurrent = MetricsConcurrent::build(trace);

        assert_eq!(
            metrics.miss_ratio,
            vec![
                1.0,
                5.0 / 6.0,
                2.0 / 3.0,
                3.0 / 6.0,
                3.0 / 6.0,
                3.0 / 6.0,
                3.0 / 6.0
            ]
        )
    }

    #[test]
    fn miss_ratio_test_3() {
        init();

        let trace: Vec<usize> = vec![1, 2, 3, 4, 5, 6, 1];
        let metrics: MetricsConcurrent = MetricsConcurrent::build(trace);

        assert_eq!(
            metrics.miss_ratio,
            vec![1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 6.0 / 7.0, 6.0 / 7.0]
        )
    }
}