use crate::utils;
use anyhow::Result;


#[derive(Debug, Clone)]
struct File {
    id: usize,
    start: usize,
    length: usize,
}

#[derive(Debug, Clone)]
struct FreeSpace {
    start: usize,
    length: usize,
}

fn parse_disk_map(disk_map: &str) -> Vec<Option<usize>> {
    let mut blocks = Vec::new();
    let chars: Vec<char> = disk_map.trim().chars().collect();
    let mut file_id = 0;
    
    for (i, &digit) in chars.iter().enumerate() {
        let length = digit.to_digit(10).unwrap() as usize;
        
        if i % 2 == 0 {
            // File blocks
            for _ in 0..length {
                blocks.push(Some(file_id));
            }
            file_id += 1;
        } else {
            // Free space
            for _ in 0..length {
                blocks.push(None);
            }
        }
    }
    
    blocks
}

fn parse_disk_map_to_files_and_spaces(disk_map: &str) -> (Vec<File>, Vec<FreeSpace>) {
    let chars: Vec<char> = disk_map.trim().chars().collect();
    let mut files = Vec::new();
    let mut free_spaces = Vec::new();
    let mut position = 0;
    let mut file_id = 0;
    
    for (i, &digit) in chars.iter().enumerate() {
        let length = digit.to_digit(10).unwrap() as usize;
        
        if length > 0 {
            if i % 2 == 0 {
                // File blocks
                files.push(File {
                    id: file_id,
                    start: position,
                    length,
                });
                file_id += 1;
            } else {
                // Free space
                free_spaces.push(FreeSpace {
                    start: position,
                    length,
                });
            }
        }
        
        position += length;
    }
    
    (files, free_spaces)
}

fn compact_disk(mut blocks: Vec<Option<usize>>) -> Vec<Option<usize>> {
    let mut left = 0;
    let mut right = blocks.len() - 1;
    
    while left < right {
        // Find the next free space from the left
        while left < blocks.len() && blocks[left].is_some() {
            left += 1;
        }
        
        // Find the next file block from the right
        while right > 0 && blocks[right].is_none() {
            right -= 1;
        }
        
        // If we found both a free space and a file block, move the file block
        if left < right {
            blocks[left] = blocks[right];
            blocks[right] = None;
            left += 1;
            right -= 1;
        }
    }
    
    blocks
}

fn compact_whole_files(disk_map: &str) -> Vec<Option<usize>> {
    let (mut files, mut free_spaces) = parse_disk_map_to_files_and_spaces(disk_map);
    
    // Sort files by decreasing file ID
    files.sort_by(|a, b| b.id.cmp(&a.id));
    
    // Try to move each file to the leftmost suitable free space
    for file in &mut files {
        // Find the leftmost free space that can fit this file and is to the left of the file
        if let Some(space_idx) = free_spaces.iter().position(|space| {
            space.length >= file.length && space.start < file.start
        }) {
            let space = &mut free_spaces[space_idx];
            
            // Move the file to this free space
            file.start = space.start;
            
            // Update the free space
            if space.length == file.length {
                // Free space is completely used
                free_spaces.remove(space_idx);
            } else {
                // Reduce the free space
                space.start += file.length;
                space.length -= file.length;
            }
            
            // Sort free spaces by start position to maintain order
            free_spaces.sort_by_key(|s| s.start);
        }
    }
    
    // Reconstruct the disk layout
    let total_length = disk_map.trim().chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .sum::<usize>();
    
    let mut blocks = vec![None; total_length];
    
    for file in &files {
        for i in 0..file.length {
            blocks[file.start + i] = Some(file.id);
        }
    }
    
    blocks
}

fn calculate_checksum(blocks: &[Option<usize>]) -> u64 {
    blocks
        .iter()
        .enumerate()
        .filter_map(|(pos, block)| {
            block.map(|file_id| pos as u64 * file_id as u64)
        })
        .sum()
}

fn solve_part1(file_data: &Vec<String>) -> Result<()> {
    let disk_map = file_data.first().ok_or_else(|| anyhow::anyhow!("No input data"))?;
    
    println!("Processing disk map with {} characters", disk_map.len());
    
    // Parse the disk map into blocks
    let blocks = parse_disk_map(disk_map);
    
    println!("Created {} blocks", blocks.len());
    
    // Compact the disk
    let compacted = compact_disk(blocks);
    
    // Calculate checksum
    let checksum = calculate_checksum(&compacted);
    
    println!("Part 1: {}", checksum);
    Ok(())
}

fn solve_part2(file_data: &Vec<String>) -> Result<()> {
    let disk_map = file_data.first().ok_or_else(|| anyhow::anyhow!("No input data"))?;
    
    println!("Processing disk map for part 2 with {} characters", disk_map.len());
    
    // Compact whole files
    let compacted = compact_whole_files(disk_map);
    
    // Calculate checksum
    let checksum = calculate_checksum(&compacted);
    
    println!("Part 2: {}", checksum);
    Ok(())
}

pub fn solve() -> Result<()> {
    let file = utils::load_input(2024, 9)?;
    let input: Vec<String> = file.lines().map(|s| s.to_string()).collect();

    solve_part1(&input)?;
    solve_part2(&input)?;
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_example() {
        let disk_map = "2333133121414131402";
        let blocks = parse_disk_map(disk_map);
        let compacted = compact_disk(blocks);
        let checksum = calculate_checksum(&compacted);
        assert_eq!(checksum, 1928);
    }
    
    #[test]
    fn test_part2_example() {
        let disk_map = "2333133121414131402";
        let compacted = compact_whole_files(disk_map);
        let checksum = calculate_checksum(&compacted);
        assert_eq!(checksum, 2858);
    }
    
    #[test]
    fn test_simple_example() {
        let disk_map = "12345";
        let blocks = parse_disk_map(disk_map);
        
        // Should create: 0..111....22222
        let expected_initial = vec![
            Some(0), None, None, Some(1), Some(1), Some(1), None, None, None, None,
            Some(2), Some(2), Some(2), Some(2), Some(2)
        ];
        assert_eq!(blocks, expected_initial);
        
        let compacted = compact_disk(blocks);
        let checksum = calculate_checksum(&compacted);
        
        // Manual calculation for 022111222......
        // 0*0 + 1*2 + 2*2 + 3*1 + 4*1 + 5*1 + 6*2 + 7*2 + 8*2 = 0+2+4+3+4+5+12+14+16 = 60
        assert_eq!(checksum, 60);
    }
}