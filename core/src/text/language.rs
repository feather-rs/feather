use crate::text::{Text, Translate};

pub trait Language {
    fn translate_with<I>(&self, translate: Translate, with: I) -> Text
    where
        I: IntoIterator,
        I::Item: Into<Text>;
}

impl Language for std::collections::HashMap<Translate, String> {
    fn translate_with<I>(&self, _translate: Translate, _with: I) -> Text
    where
        I: IntoIterator,
        I::Item: Into<Text>,
    {
        unimplemented!()
    }
}

impl Language for hashbrown::HashMap<Translate, String> {
    fn translate_with<I>(&self, _translate: Translate, _with: I) -> Text
    where
        I: IntoIterator,
        I::Item: Into<Text>,
    {
        unimplemented!()
    }
}
