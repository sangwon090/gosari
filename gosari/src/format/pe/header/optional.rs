use nom::{IResult, number::complete::{le_u8, le_u16, le_u32, le_u64}, multi::count};

#[derive(Debug)]
pub enum OptionalHeader {
    PE32(OptionalHeader32),
    PE32Plus(OptionalHeader64),
    Unknown,
}

impl Default for OptionalHeader {
    fn default() -> OptionalHeader {
        OptionalHeader::Unknown
    }
}

#[derive(Debug, Default)]
pub struct OptionalHeader32 {
    pub magic: u16,
    pub major_linker_version: u8,
    pub minor_linker_version: u8,
    pub size_of_code: u32,
    pub size_of_initialized_data: u32,
    pub size_of_uninitialized_data: u32,
    pub address_of_entry_point: u32,
    pub base_of_code: u32,
    pub base_of_data: u32,
    pub image_base: u32,
    pub section_alignment: u32,
    pub file_alignment: u32,
    pub major_operating_system_version: u16,
    pub minor_operating_system_version: u16,
    pub major_image_version: u16,
    pub minor_image_version: u16,
    pub major_subsystem_version: u16,
    pub minor_subsystem_version: u16,
    pub win32_version_value: u32,
    pub size_of_image: u32,
    pub size_of_headers: u32,
    pub checksum: u32,
    pub subsystem: u16,
    pub dll_characteristics: u16,
    pub size_of_stack_reserve: u32,
    pub size_of_stack_commit: u32,
    pub size_of_heap_reserve: u32,
    pub size_of_heap_commit: u32,
    pub loader_flags: u32,
    pub number_of_rva_and_sizes: u32,
    pub data_directory: Vec<DataDirectory>,
}

#[derive(Debug, Default)]
pub struct OptionalHeader64 {
    pub magic: u16,
    pub major_linker_version: u8,
    pub minor_linker_version: u8,
    pub size_of_code: u32,
    pub size_of_initialized_data: u32,
    pub size_of_uninitialized_data: u32,
    pub address_of_entry_point: u32,
    pub base_of_code: u32,
    pub image_base: u64,
    pub section_alignment: u32,
    pub file_alignment: u32,
    pub major_operating_system_version: u16,
    pub minor_operating_system_version: u16,
    pub major_image_version: u16,
    pub minor_image_version: u16,
    pub major_subsystem_version: u16,
    pub minor_subsystem_version: u16,
    pub win32_version_value: u32,
    pub size_of_image: u32,
    pub size_of_headers: u32,
    pub checksum: u32,
    pub subsystem: u16,
    pub dll_characteristics: u16,
    pub size_of_stack_reserve: u64,
    pub size_of_stack_commit: u64,
    pub size_of_heap_reserve: u64,
    pub size_of_heap_commit: u64,
    pub loader_flags: u32,
    pub number_of_rva_and_sizes: u32,
    pub data_directory: Vec<DataDirectory>,
}

#[derive(Debug)]
pub struct DataDirectory {
    virtual_address: u32,
    size: u32,
}

impl OptionalHeader32 {
    pub fn parse(input: &[u8]) -> IResult<&[u8], OptionalHeader32> {
        let (i, major_linker_version) = le_u8(input)?;
        let (i, minor_linker_version) = le_u8(i)?;
        let (i, size_of_code) = le_u32(i)?;
        let (i, size_of_initialized_data) = le_u32(i)?;
        let (i, size_of_uninitialized_data) = le_u32(i)?;
        let (i, address_of_entry_point) = le_u32(i)?;
        let (i, base_of_code) = le_u32(i)?;
        let (i, base_of_data) = le_u32(i)?;
        let (i, image_base) = le_u32(i)?;
        let (i, section_alignment) = le_u32(i)?;
        let (i, file_alignment) = le_u32(i)?;
        let (i, major_operating_system_version) = le_u16(i)?;
        let (i, minor_operating_system_version) = le_u16(i)?;
        let (i, major_image_version) = le_u16(i)?;
        let (i, minor_image_version) = le_u16(i)?;
        let (i, major_subsystem_version) = le_u16(i)?;
        let (i, minor_subsystem_version) = le_u16(i)?;
        let (i, win32_version_value) = le_u32(i)?;
        let (i, size_of_image) = le_u32(i)?;
        let (i, size_of_headers) = le_u32(i)?;
        let (i, checksum) = le_u32(i)?;
        let (i, subsystem) = le_u16(i)?;
        let (i, dll_characteristics) = le_u16(i)?;
        let (i, size_of_stack_reserve) = le_u32(i)?;
        let (i, size_of_stack_commit) = le_u32(i)?;
        let (i, size_of_heap_reserve) = le_u32(i)?;
        let (i, size_of_heap_commit) = le_u32(i)?;
        let (i, loader_flags) = le_u32(i)?;
        let (i, number_of_rva_and_sizes) = le_u32(i)?;
        let (i, data_directory) = count(DataDirectory::parse, number_of_rva_and_sizes as usize)(i)?;
        
        Ok((i, OptionalHeader32 {
            magic: 0x010b,
            major_linker_version,
            minor_linker_version,
            size_of_code,
            size_of_initialized_data,
            size_of_uninitialized_data,
            address_of_entry_point,
            base_of_code,
            base_of_data,
            image_base,
            section_alignment,
            file_alignment,
            major_operating_system_version,
            minor_operating_system_version,
            major_image_version,
            minor_image_version,
            major_subsystem_version,
            minor_subsystem_version,
            win32_version_value,
            size_of_image,
            size_of_headers,
            checksum,
            subsystem,
            dll_characteristics,
            size_of_stack_reserve,
            size_of_stack_commit,
            size_of_heap_reserve,
            size_of_heap_commit,
            loader_flags,
            number_of_rva_and_sizes,
            data_directory,
        }))
    }
}

impl OptionalHeader64 {
    pub fn parse(input: &[u8]) -> IResult<&[u8], OptionalHeader64> {
        let (i, major_linker_version) = le_u8(input)?;
        let (i, minor_linker_version) = le_u8(i)?;
        let (i, size_of_code) = le_u32(i)?;
        let (i, size_of_initialized_data) = le_u32(i)?;
        let (i, size_of_uninitialized_data) = le_u32(i)?;
        let (i, address_of_entry_point) = le_u32(i)?;
        let (i, base_of_code) = le_u32(i)?;
        let (i, image_base) = le_u64(i)?;
        let (i, section_alignment) = le_u32(i)?;
        let (i, file_alignment) = le_u32(i)?;
        let (i, major_operating_system_version) = le_u16(i)?;
        let (i, minor_operating_system_version) = le_u16(i)?;
        let (i, major_image_version) = le_u16(i)?;
        let (i, minor_image_version) = le_u16(i)?;
        let (i, major_subsystem_version) = le_u16(i)?;
        let (i, minor_subsystem_version) = le_u16(i)?;
        let (i, win32_version_value) = le_u32(i)?;
        let (i, size_of_image) = le_u32(i)?;
        let (i, size_of_headers) = le_u32(i)?;
        let (i, checksum) = le_u32(i)?;
        let (i, subsystem) = le_u16(i)?;
        let (i, dll_characteristics) = le_u16(i)?;
        let (i, size_of_stack_reserve) = le_u64(i)?;
        let (i, size_of_stack_commit) = le_u64(i)?;
        let (i, size_of_heap_reserve) = le_u64(i)?;
        let (i, size_of_heap_commit) = le_u64(i)?;
        let (i, loader_flags) = le_u32(i)?;
        let (i, number_of_rva_and_sizes) = le_u32(i)?;
        let (i, data_directory) = count(DataDirectory::parse, number_of_rva_and_sizes as usize)(i)?;

        Ok((i, OptionalHeader64 {
            magic: 0x020b,
            major_linker_version,
            minor_linker_version,
            size_of_code,
            size_of_initialized_data,
            size_of_uninitialized_data,
            address_of_entry_point,
            base_of_code,
            image_base,
            section_alignment,
            file_alignment,
            major_operating_system_version,
            minor_operating_system_version,
            major_image_version,
            minor_image_version,
            major_subsystem_version,
            minor_subsystem_version,
            win32_version_value,
            size_of_image,
            size_of_headers,
            checksum,
            subsystem,
            dll_characteristics,
            size_of_stack_reserve,
            size_of_stack_commit,
            size_of_heap_reserve,
            size_of_heap_commit,
            loader_flags,
            number_of_rva_and_sizes,
            data_directory,
        }))
    }
}

impl DataDirectory {
    pub fn parse(input: &[u8]) -> IResult<&[u8], DataDirectory> {
        let (i, virtual_address) = le_u32(input)?;
        let (i, size) = le_u32(i)?;

        Ok((i, DataDirectory {
            virtual_address,
            size,
        }))
    }
}