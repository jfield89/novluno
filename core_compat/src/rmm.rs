use std::str::from_utf8;
use std::io::Cursor;
use std::io::Seek;
use std::io::SeekFrom;

use byteorder::ReadBytesExt;
use byteorder::LittleEndian as LE;

use error::*;

struct Map {
    size_x: u32,
    size_y: u32,
    id_count: u8,
    id_list: Vec<u8>,
    number: u16,
    unknown_1: u16,
    unknown_2: u16,
    unknown_3: u16,
    tiles: Vec<MapTile>,
}

struct MapTile {
    unknown_1: u16,
    unknown_2: u16,
    unknown_3: u16,
    unknown_4: u16,
}

impl Map {
    pub fn new() -> Map {
        Map {
            size_x: 0,
            size_y: 0,
            id_count: 0,
            id_list: Vec::new(),
            number: 0,
            unknown_1: 0,
            unknown_2: 0,
            unknown_3: 0,
            tiles: Vec::new(),
        }
    }

    pub fn load(data: &[u8]) -> Result<Map, Error> {

        let mut cursor = Cursor::new(data);
        let mut map = Map::new();

        // filetype string: needs to equal "Resource File\n"
        let string_length = cursor.read_u8()?;
        let mut string = Vec::<u8>::new();
        for _ in 0..string_length {
            let chr = cursor.read_u8()?;
            string.push(chr);
        }
        let file_type: &str = from_utf8(&string)?;

        if file_type != "RedMoon MapData 1.0" {
            println!("{:?}", file_type);
            return Err(Error::MissingMapIdentifier);
        }

        // map size (x, y) in number of tiles
        map.size_x = cursor.read_u32::<LE>()?;
        map.size_y = cursor.read_u32::<LE>()?;

        // NOTE: The use of this array is currently unknown...
        // read in array count and the array values
        map.id_count = cursor.read_u8()?;
        for idx in 0..(map.id_count) {
            let val = cursor.read_u8()?;
            map.id_list.push(val);
        }

        // the map number described by this file...
        map.number = cursor.read_u16::<LE>()?;

        // three unknown u16 values...
        map.unknown_1 = cursor.read_u16::<LE>()?;
        map.unknown_2 = cursor.read_u16::<LE>()?;
        map.unknown_3 = cursor.read_u16::<LE>()?;

        // read in the tile values...
        let count = map.size_x * map.size_y;
        for tile in 0..count {
            let tile = MapTile {
                unknown_1: cursor.read_u16::<LE>()?,
                unknown_2: cursor.read_u16::<LE>()?,
                unknown_3: cursor.read_u16::<LE>()?,
                unknown_4: cursor.read_u16::<LE>()?,
            };
            map.tiles.push(tile);
        }

        Ok(map)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_map00001_rmm() {
        let data = include_bytes!("../../data/DATAs/Map/Map00001.rmm");
        let map = Map::load(data).unwrap();
        assert_eq!((map.size_x * map.size_y) as usize, map.tiles.len());
    }
    #[test]
    fn test_map00005_rmm() {
        let data = include_bytes!("../../data/DATAs/Map/Map00005.rmm");
        let map = Map::load(data).unwrap();
        assert_eq!((map.size_x * map.size_y) as usize, map.tiles.len());
    }
}