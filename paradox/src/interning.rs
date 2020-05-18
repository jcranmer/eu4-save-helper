use std::convert::TryInto;

/// An interned string collection that also returns a unique, autoincrementing
/// indexing for every interned string.
pub struct IdBox {
    string_table: String,
    indexes: Vec<(u16, u16)>
}

impl IdBox {
    pub fn new() -> Self {
        Self {
            string_table: String::with_capacity(4096),
            indexes: Vec::with_capacity(16)
        }
    }

    /// Add a string to the collection. If it already exists, return the index
    /// of the string. Otherwise, return None.
    pub fn add_string(&mut self, string: &str) -> u16 {
        let found = self.get_index(string);
        if let Some(idx) = found {
            idx
        } else {
            let base = self.string_table.len().try_into().unwrap();
            let len = string.len().try_into().unwrap();
            self.string_table.push_str(string);
            self.indexes.push((base, len));
            self.indexes.len().try_into().unwrap()
        }
    }

    /// Return the index of the string in the collection, if it exists.
    pub fn get_index(&self, string: &str) -> Option<u16> {
        self.string_pairs()
            .enumerate()
            .find_map(|(i, (_, s))| if string == s {
                Some((i + 1) as u16)
            } else {
                None
            })
    }

    /// Count how many strings are currently in the collection.
    pub fn len(&self) -> usize {
        self.indexes.len()
    }

    /// Return the string located at the given index.
    ///
    /// Panics if the index is out of bounds.
    pub fn get_string(&self, index: u16) -> &str {
        if index == 0 {
            return "";
        }
        let (base, len) : (u16, u16) = self.indexes[(index - 1) as usize];
        let base : usize = base.into();
        let len : usize = len.into();
        &self.string_table[base..base + len]
    }

    fn string_pairs(&self) -> impl Iterator<Item=(u16, &str)> + '_ {
        let strings = &self.string_table;
        self.indexes.iter()
            .map(move |&(start, len)| {
                (start, &strings[start.into()..(start + len).into()])
            })
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_id_box() {
        let mut idbox = super::IdBox::new();
        assert_eq!(idbox.add_string("abcd"), 1);
        assert_eq!(idbox.add_string("efgh"), 2);
        assert_eq!(idbox.add_string("abcd"), 1);
        assert_eq!(idbox.add_string("cdef"), 3);
        assert_eq!(idbox.get_index("efgh"), Some(2));
        assert_eq!(idbox.get_index("xyzw"), None);
        assert_eq!(idbox.get_string(1), "abcd");
        assert_eq!(idbox.get_string(2), "efgh");
        assert_eq!(idbox.get_string(3), "cdef");
        assert_eq!(idbox.len(), 3);
    }
}
