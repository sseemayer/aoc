use std::{collections::HashMap, hash::Hash};

pub struct SummedAreaTable<I, V>
where
    I: num::PrimInt + Hash + Clone,
    V: std::ops::Add<V, Output = V> + std::ops::Sub<V, Output = V> + std::default::Default + Copy,
{
    sat: HashMap<[I; 2], V>,
}

impl<I, V> SummedAreaTable<I, V>
where
    I: num::PrimInt + Hash + Clone,
    V: std::ops::Add<V, Output = V> + std::ops::Sub<V, Output = V> + std::default::Default + Copy,
{
    pub fn new<F>(extent: ([I; 2], [I; 2]), value_fn: F) -> Self
    where
        F: Fn([I; 2]) -> V,
    {
        let ([imin, jmin], [imax, jmax]) = extent;

        let one = I::one();

        // build up summed-area table
        let mut sat = HashMap::new();
        for i in num::iter::range_inclusive(imin, imax) {
            for j in num::iter::range_inclusive(jmin, jmax) {
                let top: V = sat.get(&[i - one, j]).copied().unwrap_or_default();
                let left: V = sat.get(&[i, j - one]).copied().unwrap_or_default();
                let topleft: V = sat.get(&[i - one, j - one]).copied().unwrap_or_default();

                let val = value_fn([i, j]) + left + top - topleft;

                sat.insert([i, j], val);
            }
        }

        Self { sat }
    }

    pub fn get_range_sum(&self, extent: ([I; 2], [I; 2])) -> V {
        let ([imin, jmin], [imax, jmax]) = extent;
        let imin = imin - I::one();
        let jmin = jmin - I::one();

        //         jmin-1 jmax
        //         |      |
        //        aabbbbbbb
        // imin-1-aAbbbbbbB
        //        ccddddddd
        //        ccddddddd
        //   imax-cCddddddD
        //         |      |
        let a = self.sat.get(&[imin, jmin]).copied().unwrap_or_default();
        let b = self.sat.get(&[imin, jmax]).copied().unwrap_or_default();
        let c = self.sat.get(&[imax, jmin]).copied().unwrap_or_default();
        let d = self.sat.get(&[imax, jmax]).copied().unwrap_or_default();

        d + a - b - c
    }
}
