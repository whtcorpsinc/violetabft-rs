//! This module contains a collection of various tools to use to manipulate
//! and control messages and data associated with violetabft.

// Copyright 2019 EinsteinDB Project Authors. Licensed under Apache-2.0.

use slog::{OwnedKVList, Record, KV};
use std::fmt;
use std::fmt::Write;
use std::u64;

use crate::evioletabftpb::{Entry, Message};
use crate::HashSet;
use protobuf::Message as PbMessage;

/// A number to represent that there is no limit.
pub const NO_LIMIT: u64 = u64::MAX;

/// Truncates the list of entries down to a specific byte-length of
/// all entries together.
///
/// # Examples
///
/// ```
/// use violetabft::{util::limit_size, prelude::*};
///
/// let template = {
///     let mut entry = Entry::default();
///     entry.data = "*".repeat(100).into_bytes();
///     entry
/// };
///
/// // Make a bunch of entries that are ~100 bytes long
/// let mut entries = vec![
///     template.clone(),
///     template.clone(),
///     template.clone(),
///     template.clone(),
///     template.clone(),
/// ];
///
/// assert_eq!(entries.len(), 5);
/// limit_size(&mut entries, Some(220));
/// assert_eq!(entries.len(), 2);
///
/// // `entries` will always have at least 1 Message
/// limit_size(&mut entries, Some(0));
/// assert_eq!(entries.len(), 1);
/// ```
pub fn limit_size<T: PbMessage + Clone>(entries: &mut Vec<T>, max: Option<u64>) {
    if entries.len() <= 1 {
        return;
    }
    let max = match max {
        None | Some(NO_LIMIT) => return,
        Some(max) => max,
    };

    let mut size = 0;
    let limit = entries
        .iter()
        .take_while(|&e| {
            if size == 0 {
                size += u64::from(e.compute_size());
                true
            } else {
                size += u64::from(e.compute_size());
                size <= max
            }
        })
        .count();

    entries.truncate(limit);
}

/// Check whether the entry is continuous to the message.
/// i.e msg's next entry index should be equal to the first entries's index
pub fn is_continuous_ents(msg: &Message, ents: &[Entry]) -> bool {
    if !msg.entries.is_empty() && !ents.is_empty() {
        let expected_next_idx = msg.entries.last().unwrap().index + 1;
        return expected_next_idx == ents.first().unwrap().index;
    }
    true
}

struct FormatKeyValueList {
    pub buffer: String,
}

impl slog::Serializer for FormatKeyValueList {
    fn emit_arguments(&mut self, key: slog::Key, val: &fmt::Arguments) -> slog::Result {
        if !self.buffer.is_empty() {
            write!(&mut self.buffer, ", {}: {}", key, val).unwrap();
        } else {
            write!(&mut self.buffer, "{}: {}", key, val).unwrap();
        }
        Ok(())
    }
}

pub(crate) fn format_kv_list(kv_list: &OwnedKVList) -> String {
    let mut formatter = FormatKeyValueList {
        buffer: "".to_owned(),
    };
    let record = record_static!(slog::Level::Trace, "");
    kv_list
        .serialize(
            &Record::new(&record, &format_args!(""), b!()),
            &mut formatter,
        )
        .unwrap();
    formatter.buffer
}

/// Get the majority number of given nodes count.
#[inline]
pub fn majority(total: usize) -> usize {
    (total / 2) + 1
}

/// A convenient struct that handles queries to both HashSet.
pub struct Union<'a> {
    first: &'a HashSet<u64>,
    second: &'a HashSet<u64>,
}

impl<'a> Union<'a> {
    /// Creates a union.
    pub fn new(first: &'a HashSet<u64>, second: &'a HashSet<u64>) -> Union<'a> {
        Union { first, second }
    }

    /// Checks if id shows up in either HashSet.
    #[inline]
    pub fn contains(&self, id: u64) -> bool {
        self.first.contains(&id) || self.second.contains(&id)
    }

    /// Returns an iterator iterates the distinct values in two sets.
    pub fn iter(&self) -> impl Iterator<Item = u64> + '_ {
        self.first.union(self.second).cloned()
    }

    /// Checks if union is empty.
    pub fn is_empty(&self) -> bool {
        self.first.is_empty() && self.second.is_empty()
    }

    /// Gets the count of the union.
    ///
    /// The time complexity is O(n).
    pub fn len(&self) -> usize {
        // Usually, second is empty.
        self.first.len() + self.second.len() - self.second.intersection(&self.first).count()
    }
}
