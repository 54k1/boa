use super::*;
use std::collections::hash_map;
use std::iter::FusedIterator;

impl Object {
    #[inline]
    pub fn iter(&self) -> Iter<'_> {
        Iter {
            indexed_properties: self.indexed_properties.iter(),
            string_properties: self.properties.iter(),
        }
    }

    #[inline]
    pub fn keys(&self) -> Keys<'_> {
        Keys(self.iter())
    }

    #[inline]
    pub fn values(&self) -> Values<'_> {
        Values(self.iter())
    }

    #[inline]
    pub fn symbols(&self) -> Symbols<'_> {
        Symbols(self.symbol_properties.iter())
    }

    #[inline]
    pub fn symbol_keys(&self) -> SymbolKeys<'_> {
        SymbolKeys(self.symbol_properties.keys())
    }

    #[inline]
    pub fn symbol_values(&self) -> SymbolValues<'_> {
        SymbolValues(self.symbol_properties.values())
    }

    #[inline]
    pub fn indexes(&self) -> Indexes<'_> {
        Indexes(self.indexed_properties.iter())
    }

    #[inline]
    pub fn index_keys(&self) -> IndexKeys<'_> {
        IndexKeys(self.indexed_properties.keys())
    }

    #[inline]
    pub fn index_values(&self) -> IndexValues<'_> {
        IndexValues(self.indexed_properties.values())
    }

    #[inline]
    pub fn strings(&self) -> Strings<'_> {
        Strings(self.properties.iter())
    }

    #[inline]
    pub fn string_keys(&self) -> StringKeys<'_> {
        StringKeys(self.properties.keys())
    }

    #[inline]
    pub fn string_values(&self) -> StringValues<'_> {
        StringValues(self.properties.values())
    }
}

#[derive(Debug, Clone)]
pub struct Iter<'a> {
    indexed_properties: hash_map::Iter<'a, u32, Property>,
    string_properties: hash_map::Iter<'a, RcString, Property>,
}

impl<'a> Iterator for Iter<'a> {
    type Item = (PropertyKey, &'a Property);
    fn next(&mut self) -> Option<Self::Item> {
        if let Some((key, value)) = self.indexed_properties.next() {
            Some(((*key).into(), value))
        } else {
            let (key, value) = self.string_properties.next()?;
            Some((key.clone().into(), value))
        }
    }
}

impl ExactSizeIterator for Iter<'_> {
    #[inline]
    fn len(&self) -> usize {
        self.indexed_properties.len() + self.string_properties.len()
    }
}

impl FusedIterator for Iter<'_> {}

#[derive(Debug, Clone)]
pub struct Keys<'a>(Iter<'a>);

impl<'a> Iterator for Keys<'a> {
    type Item = PropertyKey;
    fn next(&mut self) -> Option<Self::Item> {
        let (key, _) = self.0.next()?;
        Some(key)
    }
}

impl ExactSizeIterator for Keys<'_> {
    #[inline]
    fn len(&self) -> usize {
        self.0.len()
    }
}

impl FusedIterator for Keys<'_> {}

#[derive(Debug, Clone)]
pub struct Values<'a>(Iter<'a>);

impl<'a> Iterator for Values<'a> {
    type Item = &'a Property;
    fn next(&mut self) -> Option<Self::Item> {
        let (_, value) = self.0.next()?;
        Some(value)
    }
}

impl ExactSizeIterator for Values<'_> {
    #[inline]
    fn len(&self) -> usize {
        self.0.len()
    }
}

impl FusedIterator for Values<'_> {}

#[derive(Debug, Clone)]
pub struct Symbols<'a>(hash_map::Iter<'a, RcSymbol, Property>);

impl<'a> Iterator for Symbols<'a> {
    type Item = (&'a RcSymbol, &'a Property);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}

impl ExactSizeIterator for Symbols<'_> {
    #[inline]
    fn len(&self) -> usize {
        self.0.len()
    }
}

impl FusedIterator for Symbols<'_> {}

#[derive(Debug, Clone)]
pub struct SymbolKeys<'a>(hash_map::Keys<'a, RcSymbol, Property>);

impl<'a> Iterator for SymbolKeys<'a> {
    type Item = &'a RcSymbol;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}

impl ExactSizeIterator for SymbolKeys<'_> {
    #[inline]
    fn len(&self) -> usize {
        self.0.len()
    }
}

impl FusedIterator for SymbolKeys<'_> {}

#[derive(Debug, Clone)]
pub struct SymbolValues<'a>(hash_map::Values<'a, RcSymbol, Property>);

impl<'a> Iterator for SymbolValues<'a> {
    type Item = &'a Property;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}

impl ExactSizeIterator for SymbolValues<'_> {
    #[inline]
    fn len(&self) -> usize {
        self.0.len()
    }
}

impl FusedIterator for SymbolValues<'_> {}

#[derive(Debug, Clone)]
pub struct Indexes<'a>(hash_map::Iter<'a, u32, Property>);

impl<'a> Iterator for Indexes<'a> {
    type Item = (&'a u32, &'a Property);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}

impl ExactSizeIterator for Indexes<'_> {
    #[inline]
    fn len(&self) -> usize {
        self.0.len()
    }
}

impl FusedIterator for Indexes<'_> {}

#[derive(Debug, Clone)]
pub struct IndexKeys<'a>(hash_map::Keys<'a, u32, Property>);

impl<'a> Iterator for IndexKeys<'a> {
    type Item = &'a u32;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}

impl ExactSizeIterator for IndexKeys<'_> {
    #[inline]
    fn len(&self) -> usize {
        self.0.len()
    }
}

impl FusedIterator for IndexKeys<'_> {}

#[derive(Debug, Clone)]
pub struct IndexValues<'a>(hash_map::Values<'a, u32, Property>);

impl<'a> Iterator for IndexValues<'a> {
    type Item = &'a Property;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}

impl ExactSizeIterator for IndexValues<'_> {
    #[inline]
    fn len(&self) -> usize {
        self.0.len()
    }
}

impl FusedIterator for IndexValues<'_> {}

#[derive(Debug, Clone)]
pub struct Strings<'a>(hash_map::Iter<'a, RcString, Property>);

impl<'a> Iterator for Strings<'a> {
    type Item = (&'a RcString, &'a Property);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}

impl ExactSizeIterator for Strings<'_> {
    #[inline]
    fn len(&self) -> usize {
        self.0.len()
    }
}

impl FusedIterator for Strings<'_> {}

#[derive(Debug, Clone)]
pub struct StringKeys<'a>(hash_map::Keys<'a, RcString, Property>);

impl<'a> Iterator for StringKeys<'a> {
    type Item = &'a RcString;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}

impl ExactSizeIterator for StringKeys<'_> {
    #[inline]
    fn len(&self) -> usize {
        self.0.len()
    }
}

impl FusedIterator for StringKeys<'_> {}

#[derive(Debug, Clone)]
pub struct StringValues<'a>(hash_map::Values<'a, RcString, Property>);

impl<'a> Iterator for StringValues<'a> {
    type Item = &'a Property;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}

impl ExactSizeIterator for StringValues<'_> {
    #[inline]
    fn len(&self) -> usize {
        self.0.len()
    }
}

impl FusedIterator for StringValues<'_> {}
