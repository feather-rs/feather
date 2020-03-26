use crate::text::{Text, Translate};

pub trait Language {
    fn translate_with<I>(&self, translate: Translate, with: I) -> Text where I: IntoIterator, I::Item: Into<Text>;
}