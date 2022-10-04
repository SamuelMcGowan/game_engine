const PAGE_SIZE: usize = 64;

struct DenseEntry<T> {
    sparse_index: usize,
    element: T,
}

pub struct SparseSet<T> {
    lookup: Lookup,
    dense: Vec<DenseEntry<T>>,
}

impl<T> SparseSet<T> {
    pub fn insert(&mut self, index: usize, element: T) -> Option<T> {
        let entry = DenseEntry {
            sparse_index: index,
            element,
        };

        match self.lookup.get(index) {
            Some(dense_index) => {
                let prev = std::mem::replace(&mut self.dense[dense_index], entry);
                Some(prev.element)
            }
            None => {
                self.dense.push(entry);
                self.lookup.set(index, self.dense.len() - 1);
                None
            }
        }
    }

    pub fn remove(&mut self, index: usize) -> Option<T> {
        match self.lookup.get(index) {
            Some(dense_index) => {
                let entry = self.dense.swap_remove(dense_index);
                self.lookup.swap_remove(index, entry.sparse_index);
                Some(entry.element)
            }
            None => None,
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> + '_ {
        self.dense.iter().map(|entry| &entry.element)
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> + '_ {
        self.dense.iter_mut().map(|entry| &mut entry.element)
    }
}

#[derive(Clone)]
struct LookupPage {
    entries: Box<[Option<usize>; PAGE_SIZE]>,
}

impl Default for LookupPage {
    fn default() -> Self {
        Self {
            entries: Box::new([None; PAGE_SIZE]),
        }
    }
}

struct Lookup {
    pages: Vec<Option<LookupPage>>,
}

impl Lookup {
    fn get(&self, index: usize) -> Option<usize> {
        let (page_index, offset) = page_index(index);
        *self.pages.get(page_index)?.as_ref()?.entries.get(offset)?
    }

    fn set(&mut self, index: usize, dense_index: usize) {
        let (page_index, offset) = page_index(index);
        self.get_or_create_page(page_index).entries[offset] = Some(dense_index);
    }

    fn swap_remove(&mut self, remove_index: usize, swap_index: usize) {
        let remove_dense_index = self.remove(remove_index).unwrap();
        self.set(swap_index, remove_dense_index);
    }

    fn remove(&mut self, index: usize) -> Option<usize> {
        let (page_index, offset) = page_index(index);

        let entries = &mut self.pages.get_mut(page_index)?.as_mut()?.entries;
        std::mem::take(&mut entries[offset])
    }

    fn get_or_create_page(&mut self, page_index: usize) -> &mut LookupPage {
        if page_index < self.pages.len() {
            let none_pages = self.pages.len();
            self.pages.reserve(none_pages + 1);
            self.pages.extend(std::iter::repeat(None).take(none_pages));
            self.pages.push(Some(LookupPage::default()));
        }
        self.pages[page_index].as_mut().unwrap()
    }
}

fn page_index(index: usize) -> (usize, usize) {
    (index / PAGE_SIZE, index % PAGE_SIZE)
}
