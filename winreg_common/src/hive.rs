use bytes::Buf;
use std::string::FromUtf8Error;
use utf16string::{LittleEndian, Utf16Error, WStr, WString};

const INDEX_LEAF: &str = "li";
const FAST_LEAF: &str = "lf";
const HASH_LEAF: &str = "lh";
const INDEX_ROOT: &str = "ri";
const NAMED_KEY: &str = "nk";
const VALUE_KEY: &str = "vk";
const KEY_SECURITY: &str = "sk";
const DATA_BLOCK: &str = "db";

#[derive(Debug)]
/// Format specification: https://github.com/libyal/libregf/blob/main/documentation/Windows%20NT%20Registry%20File%20(REGF)%20format.asciidoc
pub struct HivePrimaryFile {
    base_block: HiveBaseBlock,
    hive_bins: Vec<HiveBin>,
}

#[derive(Debug)]
pub struct HiveBaseBlock {
    signature: String,
    primary_sequence_number: u32,
    secondary_sequence_number: u32,
    last_written_timestamp: u64,
    major_version: u32,
    minor_version: u32,
    file_type: u32,
    file_format: u32,
    root_cell_offset: u32,
    hive_bins_data_size: u32,
    clustering_factor: u32,
    file_name: WString<LittleEndian>,
    // reserved 396 bytes
    // reserved_0: [u8],
    checksum: u32,
    // reserved 3576 bytes (Populated on Windows 10
    // reserved_1: [u8],
    boot_type: u32,
    boot_recover: u32,
}

#[derive(Debug)]
pub struct HiveBin {
    header: HiveBinHeader,
    cells: Vec<HiveBinCell>,
}

#[derive(Debug)]
pub struct HiveBinHeader {
    signature: String,
    offset: u32,
    size: u32,
    // reserved 8 bytes
    // reserved: [u8],
    timestamp: u64,
    spare: u32,
}

#[derive(Debug)]
pub struct HiveBinCell {
    size: i32,
    cell_data: CellData,
}

#[derive(Debug)]
pub enum CellData {
    IndexLeaf(IndexLeaf),
    FastLeaf(FastLeaf),
    HashLeaf(HashLeaf),
    IndexRoot(IndexRoot),
    NamedKey(NamedKey),
    ValueKey(ValueKey),
    SecurityKey(SecurityKey),
    DataBlock(DataBlock),
}

#[derive(Debug)]
pub struct IndexLeaf {
    signature: String,
    number_of_elements: u16,
    elements: Vec<IndexLeafElement>,
}

#[derive(Debug)]
pub struct IndexLeafElement {
    key_node_offset: u32,
}

#[derive(Debug)]
pub struct FastLeaf {
    signature: String,
    number_of_elements: u16,
    elements: Vec<FastLeafElement>,
}

#[derive(Debug)]
pub struct FastLeafElement {
    key_node_offset: u32,
    name_hint: String,
}

#[derive(Debug)]
pub struct HashLeaf {
    signature: String,
    number_of_elements: u16,
    elements: Vec<HashLeafElement>,
}

#[derive(Debug)]
pub struct HashLeafElement {
    key_node_offset: u32,
    name_hash: u32,
}

#[derive(Debug)]
pub struct IndexRoot {
    signature: String,
    number_of_elements: u16,
    elements: Vec<IndexRootElement>,
}

#[derive(Debug)]
pub struct IndexRootElement {
    subkeys_list_offset: u32,
}

#[derive(Debug)]
pub struct NamedKey {
    signature: String,
    flags: u16,
    last_written_timestamp: u64,
    access_bits: u32,
    parent_key_offset: u32,
    number_of_subkeys: u32,
    number_of_volatile_subkeys: u32,
    subkeys_list_offset: u32,
    volatile_subkeys_list_offset: u32,
    number_of_key_values: u32,
    key_values_list_offset: u32,
    key_security_offset: u32,
    class_name_offset: u32,
    largest_subkey_name_length: u32,
    largest_subkey_class_name_length: u32,
    largest_value_name_length: u32,
    largest_value_data_size: u32,
    work_var: u32,
    key_name_length: u16,
    class_name_length: u16,
    key_name_string: String, // Or ASCII extended(?)
}

#[derive(Debug)]
pub struct ValueKey {
    signature: String,
    name_length: u16,
    data_size: u32,
    data_offset: u32,
    data_type: u32,
    flags: u16,
    spare: u16,
    value_name_string: String, // Or ASCII extneded(?)
}

#[derive(Debug)]
pub struct SecurityKey {
    signature: String,
    // reserved 2 bytes
    // reserved: [u8]
    previous_security_key_offset: u32,
    next_security_key_offset: u32,
    reference_count: u32,
    nt_security_descriptor_size: u32,
    // security_descriptor: [u8]
}

#[derive(Debug)]
pub struct DataBlock {
    signature: String,
    number_of_segments: u16,
    data_block_list_offset: u32,
}

#[derive(Debug)]
pub struct HiveParseError;

impl HivePrimaryFile {
    pub fn build(buf: &mut impl Buf) -> Result<Self, HiveParseError> {
        todo!()
    }
}

impl HiveBaseBlock {
    pub fn build(buf: &mut impl Buf) -> Result<Self, HiveParseError> {
        //println!("Starting hive base block at {}", buf.remaining());

        let signature = read_ascii_string(buf, 4).unwrap(); //String::from(std::str::from_utf8(bytes.get(..4).unwrap()).unwrap());
        let primary_sequence_number = buf.get_u32_le();
        let secondary_sequence_number = buf.get_u32_le();
        let last_written_timestamp = buf.get_u64_le();
        let major_version = buf.get_u32_le();
        let minor_version = buf.get_u32_le();
        let file_type = buf.get_u32_le();
        let file_format = buf.get_u32_le();
        let root_cell_offset = buf.get_u32_le();
        let hive_bins_data_size = buf.get_u32_le();
        let clustering_factor = buf.get_u32_le();
        let file_name = read_utf16_le_string(buf, 64).unwrap(); //WString::from_utf16le(Vec::from(bytes.get(..64).unwrap())).unwrap();

        // Skip reserved sector (Has data on Windows 10). See specification
        buf.advance(396);

        let checksum = buf.get_u32_le();

        // Skip reserved sector (Has data on Windows 10). See specification
        buf.advance(3576);

        let boot_type = buf.get_u32_le();
        let boot_recover = buf.get_u32_le();

        Ok(HiveBaseBlock {
            signature,
            primary_sequence_number,
            secondary_sequence_number,
            last_written_timestamp,
            major_version,
            minor_version,
            file_type,
            file_format,
            root_cell_offset,
            hive_bins_data_size,
            clustering_factor,
            file_name,
            checksum,
            boot_type,
            boot_recover,
        })
    }

    pub fn file_name(&self) -> &WStr<LittleEndian> {
        &self.file_name
    }
}

impl HiveBinHeader {
    pub fn build(buf: &mut impl Buf) -> Result<Self, HiveParseError> {
        //println!("Starting hive bin header at {}", buf.remaining());

        let signature = read_ascii_string(buf, 4).unwrap();
        let offset = buf.get_u32_le();
        let size = buf.get_u32_le();

        // Skip reserved sector
        buf.advance(8);

        let timestamp = buf.get_u64_le();
        let spare = buf.get_u32_le();

        Ok(HiveBinHeader {
            signature,
            offset,
            size,
            timestamp,
            spare,
        })
    }

    pub fn offset(&self) -> u32 {
        self.offset
    }
    pub fn size(&self) -> u32 {
        self.size
    }
}

impl HiveBinCell {
    pub fn build(
        buf: &mut impl Buf,
        hive_bin_start_pos: usize,
        hive_bin_size: u32,
    ) -> Option<Self> {
        loop {
            let start_pos = buf.remaining();
            let size = buf.get_i32_le();
            //dbg!(size);

            let bin_bytes_read = hive_bin_start_pos - buf.remaining();
            // println!(
            //     "We have read {} bytes from the bin, the bin is {} bytes long",
            //     bin_bytes_read, hive_bin_size
            // );

            if bin_bytes_read + size.abs() as usize > hive_bin_size as usize {
                let advance = hive_bin_size as usize - bin_bytes_read;
                //println!("Advancing {} and returning none", advance);
                buf.advance(advance);
                return None;
            }

            let cell_data = CellData::build(buf);

            if let Err(e) = cell_data {
                let end_pos = buf.remaining();
                let walked = start_pos - end_pos;
                let advance = size.abs() as usize - walked;
                // println!(
                //     "Got error, we have read {} bytes, but the total size is {}. Skipping {} bytes",
                //     walked, size, advance
                // );
                buf.advance(advance);
                continue;
            }

            let cell_data = cell_data.unwrap();

            let end_pos = buf.remaining();
            let walked = start_pos - end_pos;
            let advance = size.abs() as usize - walked;
            // println!(
            //     "We have read {} bytes, but the total size is {}. Skipping {} bytes",
            //     walked, size, advance
            // );
            buf.advance(advance);
            return Some(HiveBinCell { size, cell_data });
        }
    }

    pub fn cell_data(&self) -> &CellData {
        &self.cell_data
    }
}

impl CellData {
    pub fn build(buf: &mut impl Buf) -> Result<Self, HiveParseError> {
        let signature = read_ascii_string(buf, 2).map_err(|e| HiveParseError)?;
        return match signature.as_str() {
            INDEX_LEAF => Ok(CellData::IndexLeaf(IndexLeaf::build(buf).unwrap())),
            FAST_LEAF => Ok(CellData::FastLeaf(FastLeaf::build(buf).unwrap())),
            HASH_LEAF => Ok(CellData::HashLeaf(HashLeaf::build(buf).unwrap())),
            INDEX_ROOT => Ok(CellData::IndexRoot(IndexRoot::build(buf).unwrap())),
            NAMED_KEY => Ok(CellData::NamedKey(NamedKey::build(buf).unwrap())),
            VALUE_KEY => Ok(CellData::ValueKey(ValueKey::build(buf).unwrap())),
            KEY_SECURITY => Ok(CellData::SecurityKey(SecurityKey::build(buf).unwrap())),
            DATA_BLOCK => Ok(CellData::DataBlock(DataBlock::build(buf).unwrap())),
            invalid => Err(HiveParseError),
        };
    }
}

impl IndexLeaf {
    pub fn build(buf: &mut impl Buf) -> Result<Self, HiveParseError> {
        let signature = INDEX_LEAF.to_string();
        let number_of_elements = buf.get_u16_le();
        let mut elements = vec![];
        for _ in 0..number_of_elements {
            elements.push(IndexLeafElement {
                key_node_offset: buf.get_u32_le(),
            });
        }
        Ok(IndexLeaf {
            signature,
            number_of_elements,
            elements,
        })
    }
}

impl FastLeaf {
    pub fn build(buf: &mut impl Buf) -> Result<Self, HiveParseError> {
        let signature = FAST_LEAF.to_string();
        let number_of_elements = buf.get_u16_le();
        let mut elements = vec![];
        for _ in 0..number_of_elements {
            elements.push(FastLeafElement {
                key_node_offset: buf.get_u32_le(),
                name_hint: read_ascii_string(buf, 4).unwrap(),
            });
        }
        Ok(FastLeaf {
            signature,
            number_of_elements,
            elements,
        })
    }
}

impl HashLeaf {
    pub fn build(buf: &mut impl Buf) -> Result<Self, HiveParseError> {
        let signature = HASH_LEAF.to_string();
        let number_of_elements = buf.get_u16_le();
        let mut elements = vec![];
        for _ in 0..number_of_elements {
            elements.push(HashLeafElement {
                key_node_offset: buf.get_u32_le(),
                name_hash: buf.get_u32_le(),
            });
        }
        Ok(HashLeaf {
            signature,
            number_of_elements,
            elements,
        })
    }
}

impl IndexRoot {
    pub fn build(buf: &mut impl Buf) -> Result<Self, HiveParseError> {
        let signature = INDEX_ROOT.to_string();
        let number_of_elements = buf.get_u16_le();
        let mut elements = vec![];
        for _ in 0..number_of_elements {
            elements.push(IndexRootElement {
                subkeys_list_offset: buf.get_u32_le(),
            });
        }
        Ok(IndexRoot {
            signature,
            number_of_elements,
            elements,
        })
    }
}

impl NamedKey {
    pub fn build(buf: &mut impl Buf) -> Result<Self, HiveParseError> {
        let signature = NAMED_KEY.to_string();
        let flags = buf.get_u16_le();
        let last_written_timestamp = buf.get_u64_le();
        let access_bits = buf.get_u32_le();
        let parent = buf.get_u32_le();
        let number_of_subkeys = buf.get_u32_le();
        let number_of_volatile_subkeys = buf.get_u32_le();
        let subkeys_list_offset = buf.get_u32_le();
        let volatile_subkeys_list_offset = buf.get_u32_le();
        let number_of_key_values = buf.get_u32_le();
        let key_values_list_offset = buf.get_u32_le();
        let key_security_offset = buf.get_u32_le();
        let class_name_offset = buf.get_u32_le();
        let largest_subkey_name_length = buf.get_u32_le();
        let largest_subkey_class_name_length = buf.get_u32_le();
        let largest_value_name_length = buf.get_u32_le();
        let largest_value_data_size = buf.get_u32_le();
        let work_var = buf.get_u32_le();
        let key_name_length = buf.get_u16_le();
        let class_name_length = buf.get_u16_le();

        //dbg!(key_name_length);
        let key_name_string = read_ascii_string(buf, key_name_length as usize).unwrap();

        Ok(NamedKey {
            signature,
            flags,
            last_written_timestamp,
            access_bits,
            parent_key_offset: parent,
            number_of_subkeys,
            number_of_volatile_subkeys,
            subkeys_list_offset,
            volatile_subkeys_list_offset,
            number_of_key_values,
            key_values_list_offset,
            key_security_offset,
            class_name_offset,
            // Has been split starting from Windows Vista, Windows Server 2003 SP2, and Windows XP SP3
            // See specification
            largest_subkey_name_length,
            largest_subkey_class_name_length,
            largest_value_name_length,
            largest_value_data_size,
            work_var,
            key_name_length,
            class_name_length,
            key_name_string,
        })
    }

    pub fn key_name(&self) -> &str {
        &self.key_name_string
    }
}

impl ValueKey {
    pub fn build(buf: &mut impl Buf) -> Result<Self, HiveParseError> {
        let signature = VALUE_KEY.to_string();
        let name_length = buf.get_u16_le();
        let data_size = buf.get_u32_le();
        let data_offset = buf.get_u32_le();
        let data_type = buf.get_u32_le();
        let flags = buf.get_u16_le();
        let spare = buf.get_u16_le();
        let value_name_string = read_ascii_string(buf, name_length as usize).map_err(|err| {
            dbg!(name_length);
            dbg!(data_size);
            dbg!(data_offset);
            dbg!(data_type);
            dbg!(flags);
            dbg!(spare);
            err
        }).unwrap_or("ERROR".to_string());
        Ok(ValueKey {
            signature,
            name_length,
            data_size,
            data_offset,
            data_type,
            flags,
            spare,
            value_name_string,
        })
    }
}

impl SecurityKey {
    pub fn build(buf: &mut impl Buf) -> Result<Self, HiveParseError> {
        let signature = KEY_SECURITY.to_string();

        // Skip unknown 2 bytes
        buf.advance(2);

        let previous_security_key_offset = buf.get_u32_le();
        let next_security_key_offset = buf.get_u32_le();
        let reference_count = buf.get_u32_le();
        let nt_security_descriptor_size = buf.get_u32_le();

        // Skip nt security descriptor...
        buf.advance(nt_security_descriptor_size as usize);

        Ok(SecurityKey {
            signature,
            previous_security_key_offset,
            next_security_key_offset,
            reference_count,
            nt_security_descriptor_size,
        })
    }
}

impl DataBlock {
    pub fn build(buf: &mut impl Buf) -> Result<Self, HiveParseError> {
        let signature = DATA_BLOCK.to_string();
        let number_of_segments = buf.get_u16_le();
        let data_block_list_offset = buf.get_u32_le();
        let padding = buf.get_u32_le();
        Ok(DataBlock {
            signature,
            number_of_segments,
            data_block_list_offset
        })
    }
}

fn read_ascii_string(buf: &mut impl Buf, length: usize) -> Result<String, FromUtf8Error> {
    let mut result = vec![];
    for _ in 0..length {
        result.push(buf.get_u8());
    }
    String::from_utf8(result.to_ascii_lowercase())
}

fn read_utf16_le_string(
    buf: &mut impl Buf,
    length: usize,
) -> Result<WString<LittleEndian>, Utf16Error> {
    let mut result = vec![];
    for _ in 0..length {
        result.push(buf.get_u8());
    }
    WString::from_utf16le(result)
}
