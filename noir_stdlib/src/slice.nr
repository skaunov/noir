impl<T> [T] {
    /// Push a new element to the end of the slice, returning a
    /// new slice with a length one greater than the
    /// original unmodified slice.
    #[builtin(slice_push_back)]
    pub fn push_back(self, elem: T) -> Self { }

    /// Push a new element to the front of the slice, returning a
    /// new slice with a length one greater than the 
    /// original unmodified slice. 
    #[builtin(slice_push_front)]
    pub fn push_front(self, elem: T) -> Self { }

    /// Remove the last element of the slice, returning the
    /// popped slice and the element in a tuple
    #[builtin(slice_pop_back)]
    pub fn pop_back(self) -> (Self, T) { }

    /// Remove the first element of the slice, returning the
    /// element and the popped slice in a tuple
    #[builtin(slice_pop_front)]
    pub fn pop_front(self) -> (T, Self) { }

    /// Insert an element at a specified index, shifting all elements 
    /// after it to the right
    #[builtin(slice_insert)]
    pub fn insert(self, index: Field, elem: T) -> Self { }

    /// Remove an element at a specified index, shifting all elements
    /// after it to the left, returning the altered slice and 
    /// the removed element
    #[builtin(slice_remove)]
    pub fn remove(self, index: Field) -> (Self, T) { }

    // Append each element of the `other` slice to the end of `self`.
    // This returns a new slice and leaves both input slices unchanged.
    pub fn append(mut self, other: Self) -> Self {
        for elem in other {
            self = self.push_back(elem);
        }
        self
    }
}
