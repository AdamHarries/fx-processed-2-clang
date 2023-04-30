
// Helper types for lookups and iteration over processed profile tables.

use std::marker::PhantomData;


pub fn lookup_through_option<T>(op: &Option<T>) -> Option<&T>{
    match op {
        Some(v) => Some(v),
        None => None
    }
}
pub trait TableLookup<Entry> {
    fn length(&self) -> usize;
    fn lookup(&self, ix: usize) -> Entry;
    fn iter(&self) -> TableIterator<Self, Entry> where Self : Sized;
}

pub struct TableIterator<'a, T, E> where T: TableLookup<E> {
    cur_ix: usize,
    table: &'a T,
    phantom: PhantomData<E>
}

impl<'a, T, E> TableIterator<'a, T,E> where T: TableLookup<E> {
    pub fn from(table: &T) -> TableIterator<T,E> {
        TableIterator { cur_ix: 0, table: table, phantom: PhantomData}
    }
}

impl<'a, T, E> Iterator for TableIterator<'a, T,E> where T: TableLookup<E>{
    type Item = E;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cur_ix >= self.table.length() {
            None
        }else {
            let lu = self.table.lookup(self.cur_ix);
            self.cur_ix = self.cur_ix + 1;
            Some(lu)
        }
    }
}
