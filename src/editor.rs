#[derive(Debug, Default)]
pub struct Editor {
    cursor: usize,
    pub(super) content: String,
}

impl Editor {
    fn byte_index(&mut self) -> usize {
        self.content
            .char_indices()
            .map(|(i, _)| i)
            .nth(self.cursor)
            .unwrap_or(self.content.len())
    }

    fn empty(&self) -> bool {
        self.cursor == 0
    }

    fn mv_cursor(&self, pos: usize) -> usize {
        pos.clamp(0, self.content.chars().count())
    }

    pub fn mv_cursor_right(&mut self) {
        let movement = self.cursor.saturating_add(1);
        self.cursor = self.mv_cursor(movement);
    }

    pub fn mv_cursor_left(&mut self) {
        let movement = self.cursor.saturating_sub(1);
        self.cursor = self.mv_cursor(movement);
    }

    pub fn compute(&mut self, c: char) {
        let index = self.byte_index();
        self.content.insert(index, c);
        self.mv_cursor_right();
    }

    pub fn delete(&mut self) {
        if !self.empty() {
            let current = self.cursor;
            let next = current - 1;

            let rest = self.content.chars().take(next);
            let erase = self.content.chars().skip(current);

            self.content = rest.chain(erase).collect();

            self.mv_cursor_left();
        }
    }
}
