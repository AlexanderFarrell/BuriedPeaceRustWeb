struct SwapAlloc<T> {

}




struct SpacedVec<T> {
    data: Vec<Option<T>>,
    removed: Vec<i32>,
}

impl<T> SpacedVec<T> {
    pub fn new() -> SpacedVec<T> {
        SpacedVec {
            data: vec![],
            removed: vec![]
        }
    }

    pub fn push(&mut self, t: T) {
        self.data.push(t)
    }

    /*pub fn pop(&mut self) -> Option<Option<T>> {
        self.data.pop()
    }*/
}



/*struct SpacedVec<T> {
    data: Vec<Option<T>>,
    removed: Vec<i32>,
}

impl<T> SpacedVec<T> {
    pub fn new() -> SpacedVec<T> {
        SpacedVec {
            data: vec![],
            removed: vec![]
        }
    }

    pub fn push()
}
*/