use anyhow::{Context as _, Result as AnyResult};
use std::{cmp::Reverse, collections::BinaryHeap};

fn main() -> AnyResult<()> {
    let disk_map = load_input().context("load input")?;

    let result = solve_part1(&disk_map);
    println!("Part 1: {result}");
    assert_eq!(result, 6201130364722);

    let result = solve_part2(&disk_map);
    println!("Part 2: {result}");
    assert_eq!(result, 6221662795602);

    Ok(())
}

fn solve_part1(disk_map: &[u8]) -> usize {
    let mut i = 0;
    let mut j = disk_map.len() - 1;
    let mut compacted_idx = 0;
    let mut cksum = 0;
    let mut blk_to_move = usize::from(disk_map[j]);
    // j - 1 because the i just before the last j to compact cannot be trusted
    // => it's an free block count that is now wrong since we've compacted
    // everything after it.
    while i < j - 1 {
        if i % 2 == 0 {
            // File already at the beginning, copy in place to update checksum.
            compact(i / 2, disk_map[i].into(), &mut compacted_idx, &mut cksum);
        } else {
            let mut free_count = usize::from(disk_map[i]);
            while free_count != 0 {
                let id = j / 2;
                if blk_to_move <= free_count {
                    // Move block from the end to fill the free space.
                    compact(id, blk_to_move, &mut compacted_idx, &mut cksum);
                    free_count -= blk_to_move;
                    // -2 to skip free space and move to the next blk to move.
                    j -= 2;
                    blk_to_move = usize::from(disk_map[j]);
                } else {
                    compact(id, free_count, &mut compacted_idx, &mut cksum);
                    // Move enough block to fill the free space and compute the
                    // leftover.
                    blk_to_move -= free_count;
                    free_count = 0;
                }
            }
        }
        i += 1;
    }
    // Compact the leftover block.
    compact(j / 2, blk_to_move, &mut compacted_idx, &mut cksum);

    cksum
}

// Copy `blk_count` block fron file `id` and update the checksum.
fn compact(
    id: usize,
    blk_count: usize,
    compacted_idx: &mut usize,
    cksum: &mut usize,
) {
    for _ in 0..blk_count {
        *cksum += id * *compacted_idx;
        *compacted_idx += 1;
    }
}

fn solve_part2(disk_map: &[u8]) -> usize {
    let mut fs = Vec::new();
    let mut free_map = vec![BinaryHeap::new(); 10];
    for (idx, size) in disk_map.iter().enumerate() {
        if idx % 2 == 0 {
            fs.extend(std::iter::repeat_n(Some(idx / 2), *size as usize));
        } else {
            free_map[*size as usize].push(Reverse(fs.len()));
            fs.extend(std::iter::repeat_n(None, *size as usize));
        }
    }

    let mut i = fs.len() - 1;
    while i > 0 {
        match fs[i] {
            Some(id) => {
                let mut filesize = 0;
                while i > 0 && fs[i] == Some(id) {
                    filesize += 1;
                    i -= 1;
                }
                let mut best_slot = usize::MAX;
                let mut slot_width = None;
                for (w, slots) in free_map.iter().enumerate().skip(filesize) {
                    if let Some(idx) = slots.peek()
                        && idx.0 < best_slot {
                            best_slot = idx.0;
                            slot_width = Some(w);
                        }
                }
                if best_slot < i && best_slot != usize::MAX {
                    let width = slot_width.expect("slot found");
                    free_map[width].pop();
                    for j in 0..filesize {
                        fs[best_slot + j] = Some(id);
                        fs[i + 1 + j] = None;
                    }
                    free_map[width - filesize]
                        .push(Reverse(best_slot + filesize));
                }
            }
            None => {
                i -= 1;
            }
        }
    }
    fs.iter()
        .enumerate()
        .map(|(idx, id)| id.map(|id| id * idx).unwrap_or_default())
        .sum()
}

fn load_input() -> AnyResult<Vec<u8>> {
    let mut bytes =
        std::fs::read("input/day09.txt").context("read input file")?;
    bytes.pop(); // remove newline
    for b in &mut bytes {
        *b -= b'0';
    }
    Ok(bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let disk_map = "2333133121414131402"
            .bytes()
            .map(|b| b - b'0')
            .collect::<Vec<_>>();
        let result = solve_part1(&disk_map);

        assert_eq!(result, 1928);
    }

    #[test]
    fn part2() {
        let disk_map = "2333133121414131402"
            .bytes()
            .map(|b| b - b'0')
            .collect::<Vec<_>>();
        let result = solve_part2(&disk_map);

        assert_eq!(result, 2858);
    }
}
