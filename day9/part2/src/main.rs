use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

const _INPUT: &str = "input";
const _TEST_INPUT: &str = "test_input";

struct DiskSpace {
    id: Option<i32>,
    c: char,
}

fn read_file_to_string(name: &str) -> String {
    let path = Path::new(name);
    let mut file = File::open(&path).unwrap();
    let mut data_string: String = String::new();
    file.read_to_string(&mut data_string).unwrap().to_string();

    data_string
}

fn unpack_disk(disk: String) -> Vec<DiskSpace> {
    let mut disk_unpacked: Vec<DiskSpace> = Vec::new();

    let mut i: i32 = 0;
    let mut id: i32 = 0;
    for c in disk.chars() {
        if !c.is_digit(10) {
            continue;
        }

        // Number represents
        if i % 2 == 0 {
            let file_size = c.to_digit(10).unwrap() as usize;
            for _j in 0..file_size {
                let disk_space = DiskSpace {
                    id: Some(id),
                    c: 'X',
                };
                disk_unpacked.push(disk_space);
            }
            id += 1;
        } else {
            let empty_space = c.to_digit(10).unwrap() as usize;
            for _j in 0..empty_space {
                let disk_space = DiskSpace { id: None, c: '.' };
                disk_unpacked.push(disk_space);
            }
        }

        i += 1;
    }

    disk_unpacked
}

// Modified for part 2
fn format_disk(disk: &mut Vec<DiskSpace>) {
    let disk_size = disk.len();

    // Seach for a file
    let mut i = disk_size - 1;
    loop {
        // There's nothing here, keep looking for the next file
        if disk[i].id == None {
            i -= 1;
            continue;
        }
        let file_size = get_file_size(disk, i);

        // Search for an empty block to the left side of the current data block
        let mut j = 0;
        while j < disk_size && j <= i {
            // There's a file here, keep looking for the next block of empty space
            if disk[j].id != None {
                j += 1;
                continue;
            }
            let empty_space_size = get_empty_space_size(disk, j);
            if empty_space_size < file_size {
                j += empty_space_size;
                continue;
            }
            // Empty block of appropriate size is found. Move the data block to the empty space
            for k in 0..file_size {
                disk[j + k].id = disk[i - k].id;
                disk[j + k].c = disk[i - k].c;
                disk[i - k].id = None;
                disk[i - k].c = '.';
            }
            break;
        }
        if (i as i32) - (file_size as i32) < 0 {
            break;
        }
        i -= file_size;
    }
}

// Given an index at the rightmost side of a file, returns the file size
fn get_file_size(disk: &Vec<DiskSpace>, idx: usize) -> usize {
    let mut file_size = 0;
    let mut i = idx as i32;
    while i >= 0 && disk[i as usize].id == disk[idx].id {
        file_size += 1;
        i -= 1;
    }

    file_size
}

// Given an index at the leftmost side of an empty space on the dis, returns the size of the empty space
fn get_empty_space_size(disk: &Vec<DiskSpace>, idx: usize) -> usize {
    let mut space_size = 0;
    let mut i = idx;
    while i < disk.len() && disk[i].id == disk[idx].id {
        space_size += 1;
        i += 1;
    }

    space_size
}

fn compute_checksum(disk: &Vec<DiskSpace>) -> i64 {
    let mut sum: i64 = 0;
    for i in 0..disk.len() {
        match disk[i].id {
            Some(id) => sum += (id * (i as i32)) as i64,
            None => (),
        }
    }

    sum
}

fn main() {
    let data_string = read_file_to_string(_INPUT);

    let mut disk = unpack_disk(data_string);
    format_disk(&mut disk);
    println!("{}", compute_checksum(&disk));
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_unpack_disk() {
        let data_string = read_file_to_string(_TEST_INPUT);
        let disk = unpack_disk(data_string);
        let mut disk_string: String = String::new();
        for space in disk {
            match space.id {
                Some(id) => disk_string.push((id as u8 + ('0' as u8)).into()),
                None => disk_string.push(space.c),
            }
        }

        eprintln!("{}", disk_string);

        let mut disk_target_string = "00...111...2...333.44.5555.6666.777.888899".to_string();
        for _i in 0..disk_string.len() {
            assert_eq!(disk_string.pop(), disk_target_string.pop());
        }
    }

    // Modified for part 2
    #[test]
    fn test_format_disk() {
        let data_string = read_file_to_string(_TEST_INPUT);
        let mut disk = unpack_disk(data_string);
        format_disk(&mut disk);
        let mut disk_string: String = String::new();
        for space in disk {
            match space.id {
                Some(id) => disk_string.push((id as u8 + ('0' as u8)).into()),
                None => disk_string.push(space.c),
            }
        }

        eprintln!("{}", disk_string);

        let mut disk_target_string = "00992111777.44.333....5555.6666.....8888..".to_string();
        for _i in 0..disk_string.len() {
            assert_eq!(disk_string.pop(), disk_target_string.pop());
        }
    }

    #[test]
    fn test_get_checksum() {
        let data_string = read_file_to_string(_TEST_INPUT);
        let mut disk = unpack_disk(data_string);
        format_disk(&mut disk);
        assert_eq!(compute_checksum(&disk), 2858);
    }

    #[test]
    fn test_get_file_size() {
        let data_string = read_file_to_string(_TEST_INPUT);
        let disk = unpack_disk(data_string);
        assert_eq!(get_file_size(&disk, 7), 3);
        assert_eq!(get_file_size(&disk, 11), 1);
    }

    #[test]
    fn test_get_empty_space_size() {
        let data_string = read_file_to_string(_TEST_INPUT);
        let disk = unpack_disk(data_string);
        assert_eq!(get_empty_space_size(&disk, 2), 3);
        assert_eq!(get_empty_space_size(&disk, 18), 1);
    }
}