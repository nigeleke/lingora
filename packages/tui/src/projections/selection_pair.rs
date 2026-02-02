pub trait HasSelectionPair {
    type Item;

    fn reference(&self) -> Option<&Self::Item>;

    fn target(&self) -> Option<&Self::Item>;
}
