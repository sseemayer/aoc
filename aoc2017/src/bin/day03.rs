use anyhow::Result;
use aoc::map::Map;

//
// 37  36  35  34  33  32  31
// 38  17  16  15  14  13  30
// 39  18   5   4   3  12  29
// 40  19   6   1   2  11  28
// 41  20   7   8   9  10  27
// 42  21  22  23  24  25  26
// 43  44  45  46  47  48  49
//
//  B BAB B
//   \| |/      every ring expansion
//  B-BAB-B     brings an additional
//  A AAA A     circumference of
//  B-BAB-B     8 tiles
//   /| |\
//  B BAB B
//
//  ring   size   max_idx
//  0      0      1
//  1      8      9
//  2      16     25
//  3      24     49
//

fn coords_for(n: i64) -> (i64, i64) {
    let mut ring = 0;
    let mut ring_size = 0;
    let mut max_index_in_ring = 1;
    while n > max_index_in_ring {
        ring += 1;
        ring_size += 8;
        max_index_in_ring += ring_size;
    }

    if ring == 0 {
        return (0, 0);
    }

    let pos_in_ring = ring_size - (max_index_in_ring - n) - 1;
    let quarter = ring_size / 4;
    let pos_mod = pos_in_ring % quarter;

    if pos_in_ring < quarter {
        // east
        (pos_mod - ring + 1, ring)
    } else if pos_in_ring >= quarter && pos_in_ring < 2 * quarter {
        // north
        (ring, ring - pos_mod - 1)
    } else if pos_in_ring >= 2 * quarter && pos_in_ring < 3 * quarter {
        // west
        (ring - pos_mod - 1, -ring)
    } else {
        // south
        (-ring, pos_mod - ring + 1)
    }
}

fn main() -> Result<()> {
    let input = 312051;

    let (i, j) = coords_for(input);

    println!("Part 1: {}", i.abs() + j.abs());

    let mut map: Map<[i64; 2], i64> = Map::new();
    map.set([0, 0], 1);

    let mut n = 2;
    loop {
        let (i, j) = coords_for(n);

        let mut neighbor_sum = 0;
        for iofs in -1..=1 {
            for jofs in -1..=1 {
                neighbor_sum += map.get(&[i + iofs, j + jofs]).unwrap_or(&0);
            }
        }

        map.set([i, j], neighbor_sum);

        if neighbor_sum > input {
            println!("Part 2: {}", neighbor_sum);
            break;
        }

        n += 1;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_coords_for() {
        // 37  36  35  34  33  32  31
        // 38  17  16  15  14  13  30
        // 39  18   5   4   3  12  29
        // 40  19   6   1   2  11  28
        // 41  20   7   8   9  10  27
        // 42  21  22  23  24  25  26
        // 43  44  45  46  47  48  49
        assert_eq!(coords_for(1), (0, 0));
        assert_eq!(coords_for(2), (0, 1));
        assert_eq!(coords_for(3), (1, 1));
        assert_eq!(coords_for(4), (1, 0));
        assert_eq!(coords_for(5), (1, -1));
        assert_eq!(coords_for(6), (0, -1));
        assert_eq!(coords_for(7), (-1, -1));
        assert_eq!(coords_for(8), (-1, 0));
        assert_eq!(coords_for(9), (-1, 1));
        assert_eq!(coords_for(10), (-1, 2));
        assert_eq!(coords_for(11), (0, 2));
        assert_eq!(coords_for(12), (1, 2));
        assert_eq!(coords_for(13), (2, 2));
        assert_eq!(coords_for(14), (2, 1));
        assert_eq!(coords_for(15), (2, 0));
        assert_eq!(coords_for(16), (2, -1));
        assert_eq!(coords_for(17), (2, -2));
        assert_eq!(coords_for(18), (1, -2));
        assert_eq!(coords_for(19), (0, -2));
        assert_eq!(coords_for(20), (-1, -2));
        assert_eq!(coords_for(21), (-2, -2));
        assert_eq!(coords_for(22), (-2, -1));
        assert_eq!(coords_for(23), (-2, 0));
        assert_eq!(coords_for(24), (-2, 1));
        assert_eq!(coords_for(25), (-2, 2));
    }
}
