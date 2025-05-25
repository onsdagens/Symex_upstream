//! Helper functions to read dwarf debug data.

use std::{borrow::Cow, hash::Hash, io::BufRead};

use gimli::{
    AttributeValue,
    CompleteLineProgram,
    DW_AT_decl_file,
    DW_AT_decl_line,
    DW_AT_high_pc,
    DW_AT_low_pc,
    DW_AT_name,
    DebugAbbrev,
    DebugInfo,
    DebugLine,
    DebugStr,
    LineSequence,
    Reader,
};
use hashbrown::{HashMap, HashSet};
use object::{Object, ObjectSection};
use regex::Regex;

use crate::{debug, smt::SmtMap, trace};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SubProgram {
    pub name: String,
    pub bounds: (u64, u64),
    pub file: Option<(String, usize)>,
    /// Call site for an inlined sub routine.
    pub call_file: Option<(String, usize)>,
}

impl Hash for SubProgram {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

#[derive(Clone, Debug)]
pub struct SubProgramMap {
    pub index_1: HashMap<String, u64>,
    index_2: HashMap<u64, u64>,
    pub map: HashMap<u64, SubProgram>,
    counter: u64,
    pub symtab: HashMap<String, SubProgram>,
}

impl Default for SubProgramMap {
    fn default() -> Self {
        Self::_new()
    }
}

impl SubProgramMap {
    fn _new() -> Self {
        Self {
            index_1: HashMap::new(),
            index_2: HashMap::new(),
            map: HashMap::new(),
            counter: 0,
            symtab: HashMap::new(),
        }
    }

    #[cfg(test)]
    pub(crate) fn empty() -> Self {
        Self {
            index_1: HashMap::new(),
            index_2: HashMap::new(),
            map: HashMap::new(),
            counter: 0,
            symtab: HashMap::new(),
        }
    }

    pub fn insert_symtab(&mut self, symtab: HashMap<String, u64>) {
        //println!("Loading symtab {:?}", symtab);
        for (key, value) in symtab {
            let _ = self.symtab.insert(key.clone(), SubProgram {
                name: key,
                bounds: (value & ((u64::MAX >> 1) << 1), value & ((u64::MAX >> 1) << 1)),
                file: None,
                call_file: None,
            });
        }
    }

    pub fn in_bounds(&self, pc: u64) -> Vec<SubProgram> {
        self.map
            .values()
            .chain(self.symtab.values())
            .filter(|s| ((s.bounds.0)..=(s.bounds.1)).contains(&pc))
            .cloned()
            .collect()
    }

    pub fn get_all_names(&self) -> Vec<String> {
        let mut ret: Vec<String> = self.symtab.keys().cloned().collect::<Vec<_>>();
        ret.extend(self.index_1.keys().cloned());
        ret
    }

    fn insert(&mut self, name: String, address: u64, value: SubProgram) {
        let _ = self.index_1.insert(name, self.counter);
        let _ = self.index_2.insert(address & ((u64::MAX >> 1) << 1), self.counter);
        let _ = self.map.insert(self.counter, value);
        self.counter += 1;
    }

    pub fn get_by_name(&self, name: &str) -> Option<&SubProgram> {
        let idx = match self.index_1.get(name) {
            Some(val) => val,
            None => return self.symtab.get(name),
        };
        self.map.get(idx)
    }

    pub fn get_by_address(&self, address: &u64) -> Option<&SubProgram> {
        let idx = self.index_2.get(&(*address & ((u64::MAX >> 1) << 1)))?;
        self.map.get(idx)
    }

    pub fn get_by_regex(&self, pattern: &'static str) -> Option<&SubProgram> {
        let regex = Regex::new(pattern).ok()?;
        for (idx, prog) in self.index_1.iter() {
            if regex.is_match(idx) {
                return self.map.get(prog);
            }
        }
        for (idx, prog) in self.symtab.iter() {
            if regex.is_match(idx) {
                return Some(prog);
            }
        }
        None
    }

    pub fn get_all_by_regex(&self, pattern: &'static str) -> Vec<&SubProgram> {
        let regex = match Regex::new(pattern) {
            Ok(val) => val,
            Err(_) => return vec![],
        };
        let mut ret = HashSet::new();
        for (idx, prog) in self.index_1.iter() {
            if regex.is_match(idx) {
                if let Some(val) = self.map.get(prog) {
                    trace!("[{pattern}] :Matched  {val:?}");
                    ret.insert(val);
                }
            }
        }
        if !ret.is_empty() {
            return ret.into_iter().collect::<Vec<_>>();
        }
        for (idx, prog) in self.symtab.iter() {
            if regex.is_match(idx) {
                trace!("[{pattern}]2 : Matched  {prog:?}");
                ret.insert(prog);
            }
        }
        ret.into_iter().collect::<Vec<_>>()
    }

    // fn process_unit<R: Reader>(line_info: &DebugLine<R>, target_pc: u64) ->
    // Option<(String, u32)> {     let mut entry_pc = 0;
    //     let mut entry_line = 1;
    //     let mut current_file = Cow::Borrowed("");
    //     let (line_program, line_sequence): (CompleteLineProgram<R, _>,
    // Vec<LineSequence<R>>) = line_info.program(0, 32, None,
    // None).ok()?.sequences().ok()?;
    //
    //     for instruction in line_sequence {
    //         // QUESTION: Is this correct?
    //         //
    //         // PROCEDURE: If this is not correct, please correct it.
    //         if !(instruction.start..instruction.end).contains(&target_pc) {
    //             continue;
    //         }
    //
    //         let prog: CompleteLineProgram<R, <R as Reader>::Offset> =
    // line_program.resume_from(&instruction);
    //
    //         todo!(" Get the line number, file name `with absolute path`");
    //     }
    //
    //     None
    // }

    pub fn new<R: Reader>(debug_info: &DebugInfo<R>, debug_abbrev: &DebugAbbrev<R>, debug_str: &DebugStr<R>, lines: &DebugLine<R>) -> SubProgramMap {
        let mut ret: SubProgramMap = SubProgramMap::_new();
        let mut units = debug_info.units();
        while let Some(unit) = units.next().unwrap() {
            let abbrev = unit.abbreviations(debug_abbrev).unwrap();
            let mut cursor = unit.entries(&abbrev);

            while let Some((_dept, entry)) = cursor.next_dfs().unwrap() {
                let tag = entry.tag();
                if tag != gimli::DW_TAG_subprogram {
                    // is not a function continue the search
                    continue;
                }
                let attr = match entry.attr_value(DW_AT_name).unwrap() {
                    Some(a) => a,
                    None => continue,
                };

                let entry_name = match attr {
                    AttributeValue::DebugStrRef(s) => s,
                    _ => continue,
                };
                let entry_name = debug_str.get_str(entry_name).unwrap();
                let name = entry_name.to_string().unwrap().to_string();

                let addr = match entry.attr_value(DW_AT_low_pc).unwrap() {
                    Some(AttributeValue::Addr(v)) => v,
                    Some(AttributeValue::Data1(v)) => v as u64,
                    Some(AttributeValue::Data2(v)) => v as u64,
                    Some(AttributeValue::Data4(v)) => v as u64,
                    Some(AttributeValue::Data8(v)) => v,
                    Some(AttributeValue::Udata(val)) => val,
                    _ => continue,
                } & ((u64::MAX >> 1) << 1);
                let addr_end = match entry.attr_value(DW_AT_high_pc).unwrap() {
                    Some(AttributeValue::Data1(v)) => v as u64,
                    Some(AttributeValue::Data2(v)) => v as u64,
                    Some(AttributeValue::Data4(v)) => v as u64,
                    Some(AttributeValue::Data8(v)) => v,
                    Some(AttributeValue::Udata(val)) => val,
                    _val => 0,
                } & ((u64::MAX >> 1) << 1);
                let file = match entry.attr_value(DW_AT_decl_file).unwrap() {
                    Some(AttributeValue::String(s)) => s.to_string().unwrap().to_string(),
                    _ => "".to_string(),
                };
                let line = match entry.attr_value(DW_AT_decl_line).unwrap() {
                    Some(AttributeValue::Data1(v)) => v as usize,
                    Some(AttributeValue::Data2(v)) => v as usize,
                    Some(AttributeValue::Data4(v)) => v as usize,
                    Some(AttributeValue::Data8(v)) => v as usize,
                    Some(AttributeValue::Udata(val)) => val as usize,
                    _ => 0,
                };
                if addr == 0 {
                    continue;
                }
                debug!("entry point {name} at addr {}", addr);

                ret.insert(name.clone(), addr, SubProgram {
                    name,
                    bounds: (addr, addr + addr_end),
                    file: Some((file, line)),
                    call_file: None,
                });
            }
        }
        ret
    }
}

#[derive(Clone, Debug)]
pub struct LineInfo {
    file: String,
    line: u64,
    col: u64,
    text: Option<String>,
}

#[repr(transparent)]
#[derive(Clone, Debug)]
pub struct LineMap {
    map: Option<&'static HashMap<u64, LineInfo>>,
}
impl LineMap {
    pub(crate) fn empty() -> Self {
        Self { map: None }
    }

    pub fn lookup(&self, address: u64) -> Option<&LineInfo> {
        let map = self.map?;
        map.get(&address)
    }
}
impl std::fmt::Display for LineInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.text {
            Some(text) => write!(f, "{text} (in file {} on line {})", self.file, self.line),
            None => write!(f, "in file {} on line {}", self.file, self.line),
        }
    }
}

//fn line_map<R:gimli::Reader>(pc:
//

/// All credit goes to [The gimli developers](https://github.com/gimli-rs/gimli/blob/master/crates/examples/src/bin/simple_line.rs#L20)
pub(crate) fn line_program(object: &object::File<'_>, endian: gimli::RunTimeEndian) -> Result<LineMap, Box<dyn std::error::Error>> {
    // Load a section and return as `Cow<[u8]>`.
    let load_section = |id: gimli::SectionId| -> Result<std::borrow::Cow<'_, [u8]>, Box<dyn std::error::Error>> {
        Ok(match object.section_by_name(id.name()) {
            Some(section) => section.uncompressed_data()?,
            None => std::borrow::Cow::Borrowed(&[]),
        })
    };

    // Borrow a `Cow<[u8]>` to create an `EndianSlice`.
    let borrow_section = |section| gimli::EndianSlice::new(std::borrow::Cow::as_ref(section), endian);

    // Load all of the sections.
    let dwarf_sections = gimli::DwarfSections::load(&load_section)?;

    // Create `EndianSlice`s for all of the sections.
    let dwarf = dwarf_sections.borrow(borrow_section);
    let mut map = HashMap::new();

    // Iterate over the compilation units.
    let mut iter = dwarf.units();
    while let Some(header) = iter.next()? {
        let unit = dwarf.unit(header)?;
        let unit = unit.unit_ref(&dwarf);

        // Get the line program for the compilation unit.
        if let Some(program) = unit.line_program.clone() {
            let comp_dir = if let Some(ref dir) = unit.comp_dir {
                // std::path::PathBuf::from(dir.to_string_lossy().into_owned())
                std::path::PathBuf::new()
            } else {
                std::path::PathBuf::new()
            };

            // Iterate over the line program rows.
            let mut rows = program.rows();
            while let Some((header, row)) = rows.next_row()? {
                if row.end_sequence() {
                    // End of sequence indicates a possible gap in addresses.
                } else {
                    // Determine the path. Real applications should cache this for performance.
                    let mut path = std::path::PathBuf::new();
                    if let Some(file) = row.file(header) {
                        path.clone_from(&comp_dir);

                        // The directory index 0 is defined to correspond to the compilation unit
                        // directory.
                        if file.directory_index() != 0 {
                            if let Some(dir) = file.directory(header) {
                                path.push(unit.attr_string(dir)?.to_string_lossy().as_ref());
                            }
                        }

                        path.push(unit.attr_string(file.path_name())?.to_string_lossy().as_ref());
                    }

                    // Determine line/column. DWARF line/column is never 0, so we use that
                    // but other applications may want to display this differently.
                    let line = match row.line() {
                        Some(line) => line.get(),
                        None => 0,
                    };
                    let column = match row.column() {
                        gimli::ColumnType::LeftEdge => 0,
                        gimli::ColumnType::Column(column) => column.get(),
                    };
                    let mut meta = LineInfo {
                        text: None,
                        file: path.display().to_string(),
                        line,
                        col: column,
                    };
                    // 'text: {
                    //     // The output is wrapped in a Result to allow matching on errors.
                    //     // Returns an Iterator to the Reader of the lines of the file.
                    //     fn read_lines<P>(filename: P) ->
                    // std::io::Result<std::io::Lines<std::io::BufReader<std::fs::File>>>
                    //     where
                    //         P: AsRef<std::path::Path>,
                    //     {
                    //         let file = std::fs::File::open(filename)?;
                    //         Ok(std::io::BufReader::new(file).lines())
                    //     }
                    //     if path.exists() {
                    //         match read_lines(path) {
                    //             Ok(mut val) => {
                    //                 if let Some(Ok(line)) = val.nth(line as usize) {
                    //                     meta.text = Some(line);
                    //                 }
                    //             }
                    //             Err(_) => break 'text,
                    //         }
                    //     }
                    // }
                    let _ = map.try_insert(row.address(), meta);
                }
            }
        }
    }

    Ok(LineMap {
        map: Some(Box::leak(Box::new(map))),
    })
}
