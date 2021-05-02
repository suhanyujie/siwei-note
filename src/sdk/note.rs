struct Note {
    guid: String,
    title: String,
    tag_names: Vec<String>,
    content: String,
}

impl Note {
    pub fn new() -> Note {
        Note {
            title: ("".to_string()),
            tag_names: vec![],
            content: String::new(),
            guid: String::new(),
        }
    }
}

impl Default for Note {
    fn default() -> Self {
        Self::new()
    }
}
