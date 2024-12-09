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
    let mut items: Vec<MemItem> = expand_blocks(&blocks);

    todo!()
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
        // let calculated = day09_part1_handler(&mut lines.iter().map(|x| x.to_string()));
        //assert_eq!(1928, calculated);
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
}
