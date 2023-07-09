pub struct Stack<T> {
    contents: Vec<T>,
}

impl<T> Stack<T> {
    pub fn push(&mut self, element: T) {
        self.contents.push(element)
    }

    pub fn pop(&mut self) -> Option<T> {
        self.contents.pop()
    }

    pub fn new() -> Stack<T> {
        return Stack {
            contents: Vec::new(),
        };
    }
}
