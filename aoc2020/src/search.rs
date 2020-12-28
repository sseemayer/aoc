use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub trait Neighbors
where
    Self: Sized,
{
    fn get_neighbors(&self) -> Vec<(usize, Self)>;
}

#[derive(Debug)]
struct Neighbor<T> {
    distance: usize,
    previous: Vec<T>,
    data: T,
}

impl<T> std::cmp::PartialEq for Neighbor<T> {
    fn eq(&self, other: &Self) -> bool {
        self.distance.eq(&other.distance)
    }
}

impl<T> std::cmp::Eq for Neighbor<T> {}

impl<T> std::cmp::PartialOrd for Neighbor<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.distance.partial_cmp(&other.distance)
    }
}

impl<T> std::cmp::Ord for Neighbor<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.distance.cmp(&other.distance)
    }
}

#[derive(Debug)]
pub struct BFS<T: Eq> {
    queue: BinaryHeap<Reverse<Neighbor<T>>>,
}

impl<T: Eq> BFS<T> {
    pub fn new(start: T) -> Self {
        let mut queue = BinaryHeap::new();
        queue.push(Reverse(Neighbor {
            distance: 0,
            previous: Vec::new(),
            data: start,
        }));

        BFS { queue }
    }
}

impl<T> std::iter::Iterator for BFS<T>
where
    T: Neighbors + Eq + Clone,
{
    type Item = (usize, Vec<T>, T);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(i) = self.queue.pop() {
            let item = i.0;

            let mut previous = item.previous.clone();
            previous.push(item.data.clone());

            for (distance, data) in item.data.get_neighbors() {
                if previous.contains(&data) {
                    continue;
                }

                self.queue.push(Reverse(Neighbor {
                    distance: item.distance + distance,
                    previous: previous.clone(),
                    data,
                }))
            }

            Some((item.distance, item.previous, item.data))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, PartialEq, Eq, Clone)]
    struct Word(String);

    impl Neighbors for Word {
        fn get_neighbors(&self) -> Vec<(usize, Self)> {
            if self.0.len() > 4 {
                Vec::new()
            } else {
                vec![
                    (1, Word(format!("{}a", self.0))),
                    (2, Word(format!("{}bb", self.0))),
                    (3, Word(format!("{}ccc", self.0))),
                ]
            }
        }
    }

    #[test]
    fn test_it_works() {
        let bfs = BFS::new(Word("x".to_string()));
        let out: Vec<_> = bfs.collect();
        /*
         * 1    2     3      4       5        6          7
         * x -> xa -> xaa -> xaaa -D xaaaa
         * |    |     |      |--------------D xaaabb
         * |    |     |      |-------------------------D xaaaccc
         * |    |     |------------D xaabb
         * |    |     |---------------------D xaaccc
         * |    |----------> xabb -D xabba
         * |    |            |--------------D xabbbb
         * |    |            |-------------------------D xabbccc
         * |    |------------------D xaccc
         * |--------> xbb -> xbba -D xbbaa
         * |          |      |--------------D xbbabb
         * |          |      |-------------------------D xbbaccc
         * |          |------------D xbbbb
         * |          |---------------------D xbbccc
         * |---------------> xccc -D xccca
         *                   |--------------D xcccbb
         *                   |-------------------------D xcccccc
         *
         */
        let expected = vec![
            (0, "x"),
            (1, "xa"),
            (2, "xbb"),
            (2, "xaa"),
            (3, "xccc"),
            (3, "xaaa"),
            (3, "xbba"),
            (3, "xabb"),
            (4, "xaccc"),
            (4, "xbbbb"),
            (4, "xabba"),
            (4, "xbbaa"),
            (4, "xaaaa"),
            (4, "xccca"),
            (4, "xaabb"),
            (5, "xaaccc"),
            (5, "xbbabb"),
            (5, "xaaabb"),
            (5, "xabbbb"),
            (5, "xbbccc"),
            (5, "xcccbb"),
            (6, "xaaaccc"),
            (6, "xbbaccc"),
            (6, "xabbccc"),
            (6, "xcccccc"),
        ];

        assert_eq!(out.len(), expected.len());
        for (i, ((da, _, wa), (db, wb))) in out.iter().zip(expected.iter()).enumerate() {
            dbg!(i, da, wa, db, wb);
            assert_eq!(da, db);
            assert_eq!(wa.0, *wb);
        }
    }
}
