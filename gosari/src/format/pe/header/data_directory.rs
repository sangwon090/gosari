use nom::{IResult, number::complete::{le_u32}};


#[derive(Debug, Default)]
pub struct DataDirectory {
    virtual_address: u32,
    size: u32,
}

#[derive(Debug, Default)]
pub struct DataDirectories {
    export_table: DataDirectory,
    import_table: DataDirectory,
    resource_table: DataDirectory,
    exception_table: DataDirectory,
    certificate_table: DataDirectory,
    base_relocation_table: DataDirectory,
    debug: DataDirectory,
    architecture: DataDirectory,
    global_ptr: DataDirectory,
    tls_table: DataDirectory,
    load_config_table: DataDirectory,
    bound_import: DataDirectory,
    iat: DataDirectory,
    delay_import_descriptor: DataDirectory,
    clr_runtime_header: DataDirectory,
    reserved: DataDirectory,
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

impl DataDirectories {
    pub fn parse(input: &[u8]) -> IResult<&[u8], DataDirectories> {
        let (i, export_table) = DataDirectory::parse(input)?;
        let (i, import_table) = DataDirectory::parse(input)?;
        let (i, resource_table) = DataDirectory::parse(input)?;
        let (i, exception_table) = DataDirectory::parse(input)?;
        let (i, certificate_table) = DataDirectory::parse(input)?;
        let (i, base_relocation_table) = DataDirectory::parse(input)?;
        let (i, debug) = DataDirectory::parse(input)?;
        let (i, architecture) = DataDirectory::parse(input)?;
        let (i, global_ptr) = DataDirectory::parse(input)?;
        let (i, tls_table) = DataDirectory::parse(input)?;
        let (i, load_config_table) = DataDirectory::parse(input)?;
        let (i, bound_import) = DataDirectory::parse(input)?;
        let (i, iat) = DataDirectory::parse(input)?;
        let (i, delay_import_descriptor) = DataDirectory::parse(input)?;
        let (i, clr_runtime_header) = DataDirectory::parse(input)?;
        let (i, reserved) = DataDirectory::parse(input)?;

        Ok((i, DataDirectories {
            export_table,
            import_table,
            resource_table,
            exception_table,
            certificate_table,
            base_relocation_table,
            debug,
            architecture,
            global_ptr,
            tls_table,
            load_config_table,
            bound_import,
            iat,
            delay_import_descriptor,
            clr_runtime_header,
            reserved,
        }))
    }
}