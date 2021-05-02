
/// 印象笔记中对笔记本的操作都是基于 [NoteStore](https://dev.yinxiang.com/doc/articles/creating_notes.php)
pub trait NoteStoreTrait {
    fn get_sync_state(token: &str);
    fn get_sync_state_with_metrics(token: &str);
}