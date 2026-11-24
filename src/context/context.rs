use crate::dasch_project::dasch_file::DaschFile;

pub fn create_context(file: &DaschFile) {
    // with the help of qwen-model we write a short context window of the project helping to determine the
    // best matching reference ontologies and to determine the best matches within
    // the respective reference ontologies




}
mod test{
    use serde_json::json;

    #[test]
    fn test_read_context() {
        let file = json!({});
        let result = read_context();
        assert!(result.is_ok());
    }
}
