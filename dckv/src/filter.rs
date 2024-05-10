#[derive(Debug, Clone, PartialEq, Eq, Default)]
enum FilterMode {
    #[default]
    None,
    Whitelist,
    Blacklist,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Filter {
    mode: FilterMode,
}

impl Filter {
    #[inline]
    pub fn none() -> Self {
        Self::default()
    }
}
