use bytes::{Buf, Bytes};
use rayon::iter::IntoParallelRefMutIterator;
use rayon::iter::ParallelIterator;
use rayon::ThreadPool;

const HIVE_HEADER_SIZE: usize = 32;
const HIVE_BASE_BLOCK_SIG: &str = "regf";
const HIVE_BIN_HEADER_SIG: &str = "hbin";
const INDEX_LEAF_SIG: &str = "li";
const FAST_LEAF_SIG: &str = "lf";
const HASH_LEAF_SIG: &str = "lh";
const INDEX_ROOT_SIG: &str = "ri";
const NAMED_KEY_SIG: &str = "nk";
const VALUE_KEY_SIG: &str = "vk";
const KEY_SECURITY_SIG: &str = "sk";
const DATA_BLOCK_SIG: &str = "db";

#[derive(Debug)]
/// Format specification: https://github.com/libyal/libregf/blob/main/documentation/Windows%20NT%20Registry%20File%20(REGF)%20format.asciidoc
pub struct HivePrimaryFile {
    base_block: HiveBaseBlock,
    hive_bins: Vec<HiveBin>,
}

#[derive(Debug)]
pub struct HiveBaseBlock {
    primary_sequence_number: u32,
    secondary_sequence_number: u32,
    last_written_timestamp: u64,
    major_version: u32,
    minor_version: u32,
    file_type: u32,
    file_format: u32,
    root_key_offset: u32,
    hive_bins_data_size: u32,
    clustering_factor: u32,
    file_name: Vec<u8>,
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
    number_of_elements: u16,
    elements: Vec<IndexLeafElement>,
}

#[derive(Debug)]
pub struct IndexLeafElement {
    key_node_offset: u32,
}

#[derive(Debug)]
pub struct FastLeaf {
    number_of_elements: u16,
    elements: Vec<FastLeafElement>,
}

#[derive(Debug)]
pub struct FastLeafElement {
    key_node_offset: u32,
    name_hint: Vec<u8>,
}

#[derive(Debug)]
pub struct HashLeaf {
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
    number_of_elements: u16,
    elements: Vec<IndexRootElement>,
}

#[derive(Debug)]
pub struct IndexRootElement {
    subkeys_list_offset: u32,
}

#[derive(Debug)]
pub struct NamedKey {
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
    key_name: Vec<u8>, // Or ASCII extended(?)
}

#[derive(Debug)]
pub struct ValueKey {
    name_length: u16,
    data_size: u32,
    data_offset: u32,
    data_type: u32,
    flags: u16,
    spare: u16,
    value_name: Vec<u8>,
}

#[derive(Debug)]
pub struct SecurityKey {
    // reserved 2 bytes
    // reserved: [u8]
    previous_security_key_offset: u32,
    next_security_key_offset: u32,
    reference_count: u32,
    nt_security_descriptor_size: u32,
    nt_security_descriptor: Vec<u8>,
}

#[derive(Debug)]
pub struct DataBlock {
    number_of_segments: u16,
    data_block_list_offset: u32,
}

#[derive(Debug)]
pub struct HiveParseError;

impl HivePrimaryFile {
    fn build(mut buf: Bytes) -> Result<Self, HiveParseError> {
        let start = buf.remaining();

        let base_block = HiveBaseBlock::build(&mut buf)?;

        let mut bin_blocks = vec![];
        let mut offset = 0;
        while start > offset {
            let hive_bin_header = HiveBinHeader::build(&mut &buf[offset..offset + 32]).unwrap();
            if offset + hive_bin_header.size() as usize >= buf.len() {
                break;
            }
            bin_blocks.push(&buf[offset..offset + 32 + hive_bin_header.size() as usize]);
            offset += hive_bin_header.size() as usize;
        }

        let hive_bins = bin_blocks
            .par_iter_mut()
            .map(|bin| {
                let mut cells: Vec<HiveBinCell> = vec![];
                let header = HiveBinHeader::build(bin).unwrap();
                while let Some(cell) = HiveBinCell::build(bin) {
                    cells.push(cell);
                }
                HiveBin::new(header, cells)
            })
            .collect();

        Ok(HivePrimaryFile {
            base_block,
            hive_bins,
        })
    }
}

impl HiveBin {
    fn new(header: HiveBinHeader, cells: Vec<HiveBinCell>) -> Self {
        HiveBin { header, cells }
    }
}

impl HiveBaseBlock {
    pub fn build(buf: &mut impl Buf) -> Result<Self, HiveParseError> {
        let sig = &*read_arr(buf, 4);
        if sig != HIVE_BASE_BLOCK_SIG.as_bytes() {
            return Err(HiveParseError);
        }
        let primary_sequence_number = buf.get_u32_le();
        let secondary_sequence_number = buf.get_u32_le();
        let last_written_timestamp = buf.get_u64_le();
        let major_version = buf.get_u32_le();
        let minor_version = buf.get_u32_le();
        let file_type = buf.get_u32_le();
        let file_format = buf.get_u32_le();
        let root_key_offset = buf.get_u32_le();
        let hive_bins_data_size = buf.get_u32_le();
        let clustering_factor = buf.get_u32_le();
        let file_name = read_arr(buf, 64);

        // Skip reserved sector (Has data on Windows 10). See specification
        buf.advance(396);

        let checksum = buf.get_u32_le();

        // Skip reserved sector (Has data on Windows 10). See specification
        buf.advance(3576);

        let boot_type = buf.get_u32_le();
        let boot_recover = buf.get_u32_le();

        Ok(HiveBaseBlock {
            primary_sequence_number,
            secondary_sequence_number,
            last_written_timestamp,
            major_version,
            minor_version,
            file_type,
            file_format,
            root_key_offset,
            hive_bins_data_size,
            clustering_factor,
            file_name,
            checksum,
            boot_type,
            boot_recover,
        })
    }
}

impl HiveBinHeader {
    fn build(buf: &mut impl Buf) -> Result<Self, HiveParseError> {
        let sig = &*read_arr(buf, 4);
        if sig != HIVE_BIN_HEADER_SIG.as_bytes() {
            return Err(HiveParseError);
        }
        let offset = buf.get_u32_le();
        let size = buf.get_u32_le();

        // Skip reserved sector
        buf.advance(8);

        let timestamp = buf.get_u64_le();
        let spare = buf.get_u32_le();

        Ok(HiveBinHeader {
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
    fn build(buf: &mut impl Buf) -> Option<Self> {
        loop {
            let data_start_pos = buf.remaining();
            if data_start_pos - HIVE_HEADER_SIZE == 0 {
                return None;
            }

            let size = buf.get_i32_le();
            let cell_data = CellData::build(buf);

            match cell_data {
                Ok(data) => {
                    let end_pos = buf.remaining();
                    let walked = data_start_pos - end_pos;
                    let advance = size.unsigned_abs() as usize - walked;
                    buf.advance(advance);
                    return Some(HiveBinCell {
                        size,
                        cell_data: data,
                    });
                }
                Err(_) => {
                    // Skip faulty cell(?) -- TODO: Is it really a bad cell though?
                    let end_pos = buf.remaining();
                    let walked = data_start_pos - end_pos;
                    let advance = size.unsigned_abs() as usize - walked;
                    buf.advance(advance);
                    continue;
                }
            }
        }
    }

    pub fn cell_data(&self) -> &CellData {
        &self.cell_data
    }
}

impl CellData {
    fn build(buf: &mut impl Buf) -> Result<Self, HiveParseError> {
        let sig_bytes = read_arr(buf, 2);
        let sig = std::str::from_utf8(&sig_bytes).map_err(|_e| HiveParseError)?;
        match sig {
            INDEX_LEAF_SIG => Ok(CellData::IndexLeaf(IndexLeaf::build(buf).unwrap())),
            FAST_LEAF_SIG => Ok(CellData::FastLeaf(FastLeaf::build(buf).unwrap())),
            HASH_LEAF_SIG => Ok(CellData::HashLeaf(HashLeaf::build(buf).unwrap())),
            INDEX_ROOT_SIG => Ok(CellData::IndexRoot(IndexRoot::build(buf).unwrap())),
            NAMED_KEY_SIG => Ok(CellData::NamedKey(NamedKey::build(buf).unwrap())),
            VALUE_KEY_SIG => Ok(CellData::ValueKey(ValueKey::build(buf).unwrap())),
            KEY_SECURITY_SIG => Ok(CellData::SecurityKey(SecurityKey::build(buf).unwrap())),
            DATA_BLOCK_SIG => Ok(CellData::DataBlock(DataBlock::build(buf).unwrap())),
            _invalid => Err(HiveParseError),
        }
    }
}

impl IndexLeaf {
    fn build(buf: &mut impl Buf) -> Result<Self, HiveParseError> {
        let number_of_elements = buf.get_u16_le();
        let mut elements = vec![];
        for _ in 0..number_of_elements {
            elements.push(IndexLeafElement {
                key_node_offset: buf.get_u32_le(),
            });
        }
        Ok(IndexLeaf {
            number_of_elements,
            elements,
        })
    }
}

impl FastLeaf {
    fn build(buf: &mut impl Buf) -> Result<Self, HiveParseError> {
        let number_of_elements = buf.get_u16_le();
        let mut elements = vec![];
        for _ in 0..number_of_elements {
            elements.push(FastLeafElement {
                key_node_offset: buf.get_u32_le(),
                name_hint: read_arr(buf, 4),
            });
        }
        Ok(FastLeaf {
            number_of_elements,
            elements,
        })
    }
}

impl HashLeaf {
    fn build(buf: &mut impl Buf) -> Result<Self, HiveParseError> {
        let number_of_elements = buf.get_u16_le();
        let mut elements = vec![];
        for _ in 0..number_of_elements {
            elements.push(HashLeafElement {
                key_node_offset: buf.get_u32_le(),
                name_hash: buf.get_u32_le(),
            });
        }
        Ok(HashLeaf {
            number_of_elements,
            elements,
        })
    }
}

impl IndexRoot {
    fn build(buf: &mut impl Buf) -> Result<Self, HiveParseError> {
        let number_of_elements = buf.get_u16_le();
        let mut elements = vec![];
        for _ in 0..number_of_elements {
            elements.push(IndexRootElement {
                subkeys_list_offset: buf.get_u32_le(),
            });
        }
        Ok(IndexRoot {
            number_of_elements,
            elements,
        })
    }
}

impl NamedKey {
    fn build(buf: &mut impl Buf) -> Result<Self, HiveParseError> {
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
        let key_name = read_arr(buf, key_name_length as usize);

        Ok(NamedKey {
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
            largest_subkey_name_length,
            largest_subkey_class_name_length,
            largest_value_name_length,
            largest_value_data_size,
            work_var,
            key_name_length,
            class_name_length,
            key_name,
        })
    }
}

impl ValueKey {
    fn build(buf: &mut impl Buf) -> Result<Self, HiveParseError> {
        let name_length = buf.get_u16_le();
        let data_size = buf.get_u32_le();
        let data_offset = buf.get_u32_le();
        let data_type = buf.get_u32_le();
        let flags = buf.get_u16_le();
        let spare = buf.get_u16_le();
        let value_name = read_arr(buf, name_length as usize);
        Ok(ValueKey {
            name_length,
            data_size,
            data_offset,
            data_type,
            flags,
            spare,
            value_name,
        })
    }
}

impl SecurityKey {
    fn build(buf: &mut impl Buf) -> Result<Self, HiveParseError> {
        // Skip unknown 2 bytes
        buf.advance(2);

        let previous_security_key_offset = buf.get_u32_le();
        let next_security_key_offset = buf.get_u32_le();
        let reference_count = buf.get_u32_le();
        let nt_security_descriptor_size = buf.get_u32_le();
        let nt_security_descriptor = read_arr(buf, nt_security_descriptor_size as usize);

        Ok(SecurityKey {
            previous_security_key_offset,
            next_security_key_offset,
            reference_count,
            nt_security_descriptor_size,
            nt_security_descriptor,
        })
    }
}

impl DataBlock {
    fn build(buf: &mut impl Buf) -> Result<Self, HiveParseError> {
        let number_of_segments = buf.get_u16_le();
        let data_block_list_offset = buf.get_u32_le();
        let _padding = buf.get_u32_le();
        Ok(DataBlock {
            number_of_segments,
            data_block_list_offset,
        })
    }
}

pub fn parse_registry(bytes: Bytes, pool: &ThreadPool) -> Result<HivePrimaryFile, HiveParseError> {
    pool.install(|| HivePrimaryFile::build(bytes))
}

fn read_arr(buf: &mut impl Buf, length: usize) -> Vec<u8> {
    let mut result = Vec::with_capacity(length);
    for _ in 0..length {
        result.push(buf.get_u8());
    }
    result
}
