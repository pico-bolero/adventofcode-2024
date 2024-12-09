/// Solution:
// Done: Parse the data into a Vector of Enums of type Block: File{id, size}, FreeSpace{size}
// Iterate over the blocks and expand the File and FreeSpace blocks into new Enums MemItem: FileItem{id} FreeSpaceItem{}
// Swap FileItems from the End with FreeSpaceItem from the Front
// Do the final calculation: enumerate each MemItem, multiple the index by FileItem.id

/// Receives input and prints output
pub fn day09_part1(lines: &mut dyn Iterator<Item = String>) {
    let result: u64 = day09_part1_handler(lines);
    println!("Sum {}", result);
}

fn day09_part1_handler(lines: &mut (dyn Iterator<Item = String>)) -> u64 {
    let input = lines.next().unwrap(); // There is only one line of input
    let blocks = parse_input(&input);
    let mut mem: Vec<MemItem> = expand_blocks(&blocks);
    swap_free_memory(&mut mem);
    let sum: u64 = checksum(&mem);
    sum
}

fn checksum(mem: &Vec<MemItem>) -> u64 {
    mem.iter()
        .enumerate()
        .map(|(i, m)| match m {
            MemItem::FreeSpace {} => 0u64,
            MemItem::File { id } => {
                let idx: u64 = u64::try_from(i).unwrap();
                let val: u64 = u64::try_from(*id).unwrap();
                val * idx
            }
        })
        .sum()
}

fn expand_blocks(blocks: &[Block]) -> Vec<MemItem> {
    blocks
        .iter()
        .flat_map(|block| match block {
            Block::File { id, size } => (0..*size)
                .map(|_| MemItem::File { id: *id })
                .collect::<Vec<MemItem>>(),
            Block::FreeSpace { size } => (0..*size)
                .map(|_| MemItem::FreeSpace {})
                .collect::<Vec<MemItem>>(),
        })
        .collect()
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum Block {
    File { id: u32, size: u32 },
    FreeSpace { size: u32 },
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum MemItem {
    File { id: u32 },
    FreeSpace,
}

fn parse_input(input: &String) -> Vec<Block> {
    let mut blocks: Vec<Block> = vec![];
    input.chars().enumerate().for_each(|(i, c)| {
        assert!(c.is_ascii_digit());
        let size: u32 = char::to_digit(c, 10).unwrap();
        if i % 2 == 1 {
            blocks.push(Block::FreeSpace { size });
        } else {
            let id: u32 = u32::try_from(i).unwrap() / 2 + u32::try_from(i).unwrap() % 2;
            blocks.push(Block::File { id, size });
        }
    });
    blocks
}

fn swap_free_memory(mem: &mut Vec<MemItem>) -> () {
    let mut left_idx: usize = 0;
    let mut right_idx: usize = mem.len() - 1;

    while left_idx <= right_idx {
        // increment until free memory
        while let Some(MemItem::File { id: _ }) = mem.get(left_idx) {
            left_idx += 1;
        }
        // decrement until free File
        while let Some(MemItem::FreeSpace {}) = mem.get(right_idx) {
            right_idx -= 1;
        }
        if left_idx < right_idx {
            mem.swap(left_idx, right_idx);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_data() -> Vec<String> {
        let lines: Vec<String> = "2333133121414131402"
            .split('\n')
            .map(|x| x.to_string())
            .collect();
        lines
    }

    #[test]
    fn test_day09_part1_handler() {
        let lines = sample_data();
        let calculated = day09_part1_handler(&mut lines.iter().map(|x| x.to_string()));
        assert_eq!(1928, calculated);
    }

    #[test]
    fn test_parse_blocks() {
        let calculated = parse_input(&"123456".to_string());
        let expected = vec![
            Block::File { id: 0, size: 1 },
            Block::FreeSpace { size: 2 },
            Block::File { id: 1, size: 3 },
            Block::FreeSpace { size: 4 },
            Block::File { id: 2, size: 5 },
            Block::FreeSpace { size: 6 },
        ];
        assert_eq!(expected, calculated);
    }

    #[test]
    fn test_expand_blocks() {
        let blocks = vec![
            Block::File { id: 0, size: 1 },
            Block::FreeSpace { size: 2 },
            Block::File { id: 1, size: 3 },
            Block::FreeSpace { size: 4 },
        ];
        let calculated = expand_blocks(&blocks);

        let expected = vec![
            MemItem::File { id: 0 },
            MemItem::FreeSpace {},
            MemItem::FreeSpace {},
            MemItem::File { id: 1 },
            MemItem::File { id: 1 },
            MemItem::File { id: 1 },
            MemItem::FreeSpace {},
            MemItem::FreeSpace {},
            MemItem::FreeSpace {},
            MemItem::FreeSpace {},
        ];

        assert_eq!(expected, calculated);
    }

    #[test]
    fn test_swap_free_memory() {
        let mut mem = vec![
            MemItem::File { id: 0 },
            MemItem::FreeSpace {},
            MemItem::FreeSpace {},
            MemItem::File { id: 1 },
            MemItem::File { id: 1 },
            MemItem::File { id: 1 },
            MemItem::FreeSpace {},
            MemItem::FreeSpace {},
            MemItem::FreeSpace {},
            MemItem::FreeSpace {},
        ];

        let expected = vec![
            MemItem::File { id: 0 },
            MemItem::File { id: 1 },
            MemItem::File { id: 1 },
            MemItem::File { id: 1 },
            MemItem::FreeSpace {},
            MemItem::FreeSpace {},
            MemItem::FreeSpace {},
            MemItem::FreeSpace {},
            MemItem::FreeSpace {},
            MemItem::FreeSpace {},
        ];

        swap_free_memory(&mut mem);
        assert_eq!(expected, mem);
    }

    #[test]
    fn test_checksum() {
        let mem = vec![
            MemItem::File { id: 0 },
            MemItem::File { id: 1 },
            MemItem::File { id: 1 },
            MemItem::File { id: 1 },
            MemItem::File { id: 5 },
            MemItem::FreeSpace {},
            MemItem::FreeSpace {},
            MemItem::FreeSpace {},
            MemItem::FreeSpace {},
            MemItem::FreeSpace {},
            MemItem::FreeSpace {},
        ];
        let calculated = checksum(&mem);
        let expected: u64 = (0 * 0) + (1 * 1) + (2 * 1) + (3 * 1) + (4 * 5);
        assert_eq!(expected, calculated);
    }
}
