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

fn format_disk(disk: &mut Vec<DiskSpace>) {
    let compacted_data_size = get_data_size(disk);
    let disk_size= disk.len();

    for i in 0..compacted_data_size {
        match disk[i].id {
            Some(_) => continue,
            None => (),
        }

        for j in (0..disk_size).rev() {
            if disk[j].id == None {
                continue;
            }

            disk[i].id = disk[j].id;
            disk[i].c = disk[j].c;
            disk[j].id = None;
            disk[j].c = '.';
            break
        }
    }
}

fn get_data_size(disk: &Vec<DiskSpace>) -> usize {
    let mut size = 0;

    for disk_space in disk {
        if disk_space.id != None {
            size += 1;
        }
    }

    size
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

        let mut disk_target_string = "0099811188827773336446555566..............".to_string();
        for _i in 0..disk_string.len() {
            assert_eq!(disk_string.pop(), disk_target_string.pop());
        }
    }

    #[test]
    fn test_get_data_size() {
        let data_string = read_file_to_string(_TEST_INPUT);
        let mut disk = unpack_disk(data_string);
        format_disk(&mut disk);
        assert_eq!(get_data_size(&disk), 28)
    }

    #[test]
    fn test_get_checksum() {
        let data_string = read_file_to_string(_TEST_INPUT);
        let mut disk = unpack_disk(data_string);
        format_disk(&mut disk);
        assert_eq!(compute_checksum(&disk), 1928);
    }
}
