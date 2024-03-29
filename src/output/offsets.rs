use std::collections::BTreeMap;
use std::fmt::Write;

use heck::{AsPascalCase, AsSnakeCase};

use super::{format_module_name, CodeGen, OffsetMap, Results};

use crate::error::Result;

impl CodeGen for OffsetMap {
    fn to_cs(&self, results: &Results, indent_size: usize) -> Result<String> {
        self.write_content(results, indent_size, |fmt| {
            fmt.block("namespace CS2Dumper.Offsets", false, |fmt| {
                for (module_name, offsets) in self {
                    writeln!(fmt, "// Module: {}", module_name)?;

                    fmt.block(
                        &format!(
                            "public static class {}",
                            AsPascalCase(format_module_name(module_name))
                        ),
                        false,
                        |fmt| {
                            for offset in offsets {
                                writeln!(
                                    fmt,
                                    "public const nint {} = {:#X};",
                                    offset.name, offset.value
                                )?;
                            }

                            Ok(())
                        },
                    )?;
                }

                Ok(())
            })?;

            Ok(())
        })
    }

    fn to_hpp(&self, results: &Results, indent_size: usize) -> Result<String> {
        self.write_content(results, indent_size, |fmt| {
            writeln!(fmt, "#pragma once\n")?;
            writeln!(fmt, "#include <cstddef>\n")?;

            fmt.block("namespace cs2_dumper", false, |fmt| {
                fmt.block("namespace offsets", false, |fmt| {
                    for (module_name, offsets) in self {
                        writeln!(fmt, "// Module: {}", module_name)?;

                        fmt.block(
                            &format!("namespace {}", AsSnakeCase(format_module_name(module_name))),
                            false,
                            |fmt| {
                                for offset in offsets {
                                    writeln!(
                                        fmt,
                                        "constexpr std::ptrdiff_t {} = {:#X};",
                                        offset.name, offset.value
                                    )?;
                                }

                                Ok(())
                            },
                        )?;
                    }

                    Ok(())
                })
            })?;

            Ok(())
        })
    }

    fn to_json(&self, _results: &Results, _indent_size: usize) -> Result<String> {
        let content: BTreeMap<_, _> = self
            .iter()
            .map(|(module_name, offsets)| {
                let offsets: BTreeMap<_, _> = offsets
                    .iter()
                    .map(|offset| (&offset.name, offset.value))
                    .collect();

                (module_name, offsets)
            })
            .collect();

        serde_json::to_string_pretty(&content).map_err(Into::into)
    }

    fn to_rs(&self, results: &Results, indent_size: usize) -> Result<String> {
        self.write_content(results, indent_size, |fmt| {
            writeln!(fmt, "#![allow(non_upper_case_globals, unused)]\n")?;

            fmt.block("pub mod cs2_dumper", false, |fmt| {
                fmt.block("pub mod offsets", false, |fmt| {
                    for (module_name, offsets) in self {
                        writeln!(fmt, "// Module: {}", module_name)?;

                        fmt.block(
                            &format!("pub mod {}", AsSnakeCase(format_module_name(module_name))),
                            false,
                            |fmt| {
                                for offset in offsets {
                                    writeln!(
                                        fmt,
                                        "pub const {}: usize = {:#X};",
                                        offset.name, offset.value
                                    )?;
                                }

                                Ok(())
                            },
                        )?;
                    }

                    Ok(())
                })
            })?;

            Ok(())
        })
    }
}