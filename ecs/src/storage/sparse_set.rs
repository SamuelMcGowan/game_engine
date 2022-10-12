const PAGE_SIZE: usize = 64;

#[derive(Debug)]
struct DenseEntry<T> {
    sparse_index: usize,
    element: T,
}

#[derive(Debug)]
pub struct SparseSet<T> {
    sparse: SparseArray,
    dense: Vec<DenseEntry<T>>,
}

impl<T> Default for SparseSet<T> {
    fn default() -> Self {
        Self {
            sparse: SparseArray::default(),
            dense: vec![],
        }
    }
}

impl<T> SparseSet<T> {
    #[inline]
    pub fn get(&self, index: usize) -> Option<&T> {
        let dense_index = self.sparse.get(index)?;
        Some(&self.dense[dense_index].element)
    }

    #[inline]
    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        let dense_index = self.sparse.get(index)?;
        Some(&mut self.dense[dense_index].element)
    }

    pub fn insert(&mut self, index: usize, element: T) -> Option<T> {
        let entry = DenseEntry {
            sparse_index: index,
            element,
        };

        match self.sparse.get(index) {
            Some(dense_index) => {
                let prev = std::mem::replace(&mut self.dense[dense_index], entry);
                Some(prev.element)
            }
            None => {
                self.dense.push(entry);
                self.sparse.set(index, self.dense.len() - 1);
                None
            }
        }
    }

    pub fn remove(&mut self, index: usize) -> Option<T> {
        match self.sparse.get(index) {
            Some(dense_index) => {
                // Swap-remove the element from the dense-array.
                let removed = self.dense.swap_remove(dense_index);

                // If we swapped an element, update sparse-array entry for the swapped
                // element now located where the removed element previously was.
                if dense_index != self.dense.len() {
                    let sparse_swapped_index = self.dense[dense_index].sparse_index;
                    self.sparse.set(sparse_swapped_index, index);
                }

                // Remove the sparse-array entry for the removed element.
                self.sparse.remove(index);

                Some(removed.element)
            }
            None => None,
        }
    }

    #[inline]
    pub fn iter(&self) -> Iter<T> {
        Iter(self.dense.iter())
    }

    #[inline]
    pub fn iter_mut(&mut self) -> IterMut<T> {
        IterMut(self.dense.iter_mut())
    }

    #[inline]
    pub fn contains(&self, index: usize) -> bool {
        self.sparse.get(index).is_some()
    }
}

pub struct Iter<'a, T>(std::slice::Iter<'a, DenseEntry<T>>);

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|entry| &entry.element)
    }
}

pub struct IterMut<'a, T>(std::slice::IterMut<'a, DenseEntry<T>>);

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|entry| &mut entry.element)
    }
}

#[derive(Debug, Clone)]
struct SparseArrayPage {
    entries: Box<[Option<usize>; PAGE_SIZE]>,
}

impl Default for SparseArrayPage {
    fn default() -> Self {
        Self {
            entries: Box::new([None; PAGE_SIZE]),
        }
    }
}

#[derive(Default, Debug)]
struct SparseArray {
    pages: Vec<Option<SparseArrayPage>>,
}

impl SparseArray {
    fn get(&self, index: usize) -> Option<usize> {
        let (page_index, offset) = page_index(index);
        *self.pages.get(page_index)?.as_ref()?.entries.get(offset)?
    }

    fn set(&mut self, index: usize, dense_index: usize) {
        let (page_index, offset) = page_index(index);
        self.get_or_create_page(page_index).entries[offset] = Some(dense_index);
    }

    /// Panics if index isn't a valid entry.
    fn remove(&mut self, index: usize) {
        let (page_index, offset) = page_index(index);

        let entries = &mut self
            .pages
            .get_mut(page_index)
            .unwrap()
            .as_mut()
            .unwrap()
            .entries;

        entries[offset] = None;
    }

    fn get_or_create_page(&mut self, page_index: usize) -> &mut SparseArrayPage {
        if page_index >= self.pages.len() {
            let none_pages = self.pages.len();
            self.pages.reserve(none_pages + 1);
            self.pages.extend(std::iter::repeat(None).take(none_pages));
            self.pages.push(Some(SparseArrayPage::default()));
        }
        self.pages[page_index].as_mut().unwrap()
    }
}

fn page_index(index: usize) -> (usize, usize) {
    (index / PAGE_SIZE, index % PAGE_SIZE)
}
