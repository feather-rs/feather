use std::mem;

use crate::Text;

impl<'a> Text<'a> {
    pub fn simplify(mut self) -> Self {
        let extra = mem::take(&mut self.extra);
        let extra = extra.into_iter().filter_map(|mut extra| {
            if extra.color == self.color {
                extra.color = None;
            }
            if extra.bold == self.bold {
                extra.bold = None;
            }
            if extra.italic == self.italic {
                extra.italic = None;
            }
            if self.content.is_text() {
                self.content = extra.content;
                None
            } else {
                Some(extra)
            }
        });
        self.extra = extra.collect();
        todo!()
    }
}
