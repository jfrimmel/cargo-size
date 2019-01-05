//! A module containing functions for interacting with binaries.
use crate::error::Error;
use std::path::Path;

/// All sections, that contain program code.
///
/// The size of those sections (if present) are added up in order to calculate
/// the program size.
pub const PROGRAM_SECTIONS: [&str; 3] = [".vector_table", ".text", ".rodata"];

/// All sections, that contain program data.
///
/// The size of those sections (if present) are added up in order to calculate
/// the data size.
pub const DATA_SECTIONS: [&str; 2] = [".bss", ".data"];

/// Read the code and data size from the binary.
///
/// The size of the sections listed in [`PROGRAM_SECTIONS`][program] and
/// [`DATA_SECTIONS`][data] are added up and returned. Non-existing sections
/// are ignored.
///
/// [program]: constant.PROGRAM_SECTIONS.html
/// [data]: constant.DATA_SECTIONS.html
pub fn read_size_from(file: &Path) -> Result<(u64, u64), Error> {
    let file = elf::File::open_path(file)?;

    let code = PROGRAM_SECTIONS
        .into_iter()
        .map(|section| file.get_section(section))
        .filter_map(|section| section)
        .map(|section| section.shdr.size)
        .sum();

    let data = DATA_SECTIONS
        .into_iter()
        .map(|section| file.get_section(section))
        .filter_map(|section| section)
        .map(|section| section.shdr.size)
        .sum();

    Ok((code, data))
}
