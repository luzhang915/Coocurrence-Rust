use std::collections::HashMap;
use std::path::Path;
use std::vec::IntoIter;

use csv::Writer;

use super::bucket::Bucket;

pub struct Hist {
    cnt: u128,
    hist: HashMap<u128, Bucket>,
    sublog_bits: Option<u32>,
}

impl Hist {
    pub fn new_empty() -> Hist {
        Hist {
            cnt: 0,
            hist: HashMap::new(),
            sublog_bits: Some(0),
        }
    }

    pub fn with_capacity_empty(capacity: usize) -> Hist {
        Hist {
            cnt: 0,
            hist: HashMap::with_capacity(capacity),
            sublog_bits: Some(0),
        }
    }

    pub fn new_empty_sublog_bits(sublog_bits: u32) -> Hist {
        Hist {
            cnt: 0,
            hist: HashMap::new(),
            sublog_bits: Some(sublog_bits),
        }
    }

    pub fn with_capacity_empty_sublog_bits(capacity: usize, sublog_bits: u32) -> Hist {
        Hist {
            cnt: 0,
            hist: HashMap::with_capacity(capacity),
            sublog_bits: Some(sublog_bits),
        }
    }

    pub fn build(cnt: u128, hist: HashMap<u128, Bucket>) -> Hist {
        Hist {
            cnt,
            hist,
            sublog_bits: None,
        }
    }

    pub fn build_from_file(path: &Path) -> Hist {
        println!("Implement 'build_from_file({:?}: Path", path.to_str());

        let cnt: u128 = 0;
        let hist: HashMap<u128, Bucket> = HashMap::new();

        Hist {
            cnt,
            hist,
            sublog_bits: None,
        }
    }

    pub fn increment(&mut self, value: u128) {
        self.cnt += value;

        let bucket_min_max: BucketMinMax = self.value_to_bucket_min_max(value);

        match self.hist.get_mut(&bucket_min_max.min) {
            Some(bucket) => bucket.increment(value),
            None => {
                let bucket: Bucket = Bucket::build(bucket_min_max.min, bucket_min_max.max, 1, value);

                self.hist.insert(bucket_min_max.min, bucket);
            },
        }
    }

    fn value_to_bucket_min_max(&self, value: u128) -> BucketMinMax {
        if value < (1 << self.sublog_bits.unwrap() as u128) {
            return BucketMinMax::build(value, value);
        }

        let leading_bit: u32 = 127 - value.leading_zeros();
        let sub_range_width: u128 = 1 << (leading_bit - self.sublog_bits.unwrap()) as u128;
        let lower_bound: u128 = sub_range_width << (self.sublog_bits.unwrap() as u128);
        let sub_range = (value - lower_bound) / sub_range_width;

        let bucket_min: u128 = lower_bound + sub_range;
        let bucket_max: u128 = lower_bound + ((sub_range + 1) * sub_range_width) - 1;

        BucketMinMax::build(bucket_min, bucket_max)
    }

    pub fn bucket_iter_sorted(&self) -> IntoIter<&Bucket> {
        let mut buckets: Vec<&Bucket> = self.hist.values().collect();
        buckets.sort_unstable();
        buckets.into_iter()
    }

    pub fn bucket_iter_sorted_mut(&mut self) -> IntoIter<&Bucket> {
        let mut buckets: Vec<&Bucket> = self.hist.values().collect();
        buckets.sort_unstable();
        buckets.into_iter()
    }

    pub fn bucket_iter(&self) -> IntoIter<&Bucket> {
        let buckets: Vec<&Bucket> = self.hist.values().collect();
        buckets.into_iter()
    }

    pub fn bucket_iter_mut(&mut self) -> IntoIter<&Bucket> {
        let buckets: Vec<&Bucket> = self.hist.values().collect();
        buckets.into_iter()
    }

    pub fn get_count(&self) -> u128 {
        self.cnt
    }

    pub fn write_to_csv(&self, path: &Path) -> Result<(), csv::Error> {
        let mut writer = Writer::from_path(path)?;

        for bucket in self.bucket_iter_sorted() {
            writer.serialize(bucket)?;
        }

        Ok(())
    }

}

struct BucketMinMax {
    min: u128,
    max: u128,
}

impl BucketMinMax {
    pub fn build(min: u128, max: u128) -> BucketMinMax {
        BucketMinMax {
            min,
            max,
        }
    }
}