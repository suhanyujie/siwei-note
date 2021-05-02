pub fn convert_to_enml(content: &str) -> String {
    let f1 = format!(
        "{}{}{}",
        r#"
    <?xml version="1.0" encoding="UTF-8"?>
       <!DOCTYPE en-note SYSTEM "http://xml.evernote.com/pub/enml2.dtd">
       <en-note>
   "#,
        content,
        r#"</en-note>"#
    );
    f1
}
