use crate::item::ItemReport;
use crate::WriteExt;
use byteorder::{LittleEndian, WriteBytesExt};
use failure::Error;
use std::io::Write;

pub fn generate_mappings_file(
    report: ItemReport,
    write_string_ids: bool,
) -> Result<Vec<u8>, Error> {
    let mut buf = Vec::new();

    buf.write_all(b"FEATHER_ITEM_DATA_FILE")?;

    // TODO handle non-native files
    assert!(write_string_ids, "unimplemented");

    let len = report.mappings.len();
    buf.write_u32::<LittleEndian>(len as u32)?;

    for (item_name, item) in report.mappings {
        let id = item.protocol_id;
        buf.write_string(&item_name)?;
        buf.write_i32::<LittleEndian>(id)?;
    }

    Ok(buf)
}
