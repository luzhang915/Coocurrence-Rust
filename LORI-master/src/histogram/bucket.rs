use std::cmp::Ordering;

use serde::Serialize;

/// A data structure to describe the buckets of a histogram.
#[derive(Debug, PartialEq, Eq, Serialize)]
pub struct Bucket {
    min: u128,
    max: u128,
    cnt: u128,
    sum: u128,
}

impl Bucket {
    /// Builds a bucket based off of the bucket's
    /// 'min' the minimum value that can be counted in this bucket,
    /// 'max' the maximum value that can be counted in this bucket,
    /// 'cnt' the count of how many elements there are in this bucket, and
    /// 'sum' the sum of the values of the elements in this bucket.
    pub fn build(min: u128, max: u128, cnt: u128, sum: u128) -> Bucket {
        Bucket {
            min,
            max,
            cnt,
            sum,
        }
    }

    pub fn increment(&mut self, value: u128) {
        self.sum += value;
        self.cnt += 1;
    }

    pub fn get_min(&self) -> &u128 {
        &self.min
    }

    pub fn get_max(&self) -> &u128 {
        &self.max
    }

    pub fn get_cnt(&self) -> &u128 {
        &self.cnt
    }

    pub fn get_sum(&self) -> &u128 {
        &self.sum
    }

    pub fn range(&self) -> u128 {
        &self.max - &self.min
    }

}

impl Ord for Bucket {
    fn cmp(&self, other: &Self) -> Ordering {
        self.min.cmp(&other.min)
    }
}

impl PartialOrd for Bucket {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bucket_1() {
        let min: u128 = 10;
        let max: u128 = 19;
        let cnt: u128 = 7;
        let sum: u128 = 133;
        let range: u128 = max - min;

        let bucket: Bucket = Bucket::build(min.clone(), max.clone(), cnt.clone(), sum.clone());

        assert_eq!(&min, bucket.get_min());
        assert_eq!(&max, bucket.get_max());
        assert_eq!(&cnt, bucket.get_cnt());
        assert_eq!(&sum, bucket.get_sum());
        assert_eq!(range, bucket.range());
    }
}