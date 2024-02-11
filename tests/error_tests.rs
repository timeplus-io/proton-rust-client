use proton_client::prelude::ProtonClientError;

#[test]
fn test_proton_client_errors() {
    let query_error = ProtonClientError::QueryFailed("Query error".to_string());
    let insert_error = ProtonClientError::InsertFailed("Insert error".to_string());
    let inserter_error = ProtonClientError::InserterFailed("Inserter error".to_string());
    let fetch_error = ProtonClientError::FetchFailed("Fetch error".to_string());
    let fetch_all_error = ProtonClientError::FetchAllFailed("Fetch all error".to_string());
    let fetch_one_error = ProtonClientError::FetchOneFailed("Fetch one error".to_string());
    let fetch_optional_error =
        ProtonClientError::FetchOptionalFailed("Fetch optional error".to_string());

    assert_eq!(
        format!("{}", query_error),
        "[ProtonClient]: Query failed with error: Query error"
    );
    assert_eq!(
        format!("{}", insert_error),
        "[ProtonClient]: Insert failed with error: Insert error"
    );
    assert_eq!(
        format!("{}", inserter_error),
        "[ProtonClient]: Inserter failed with error: Inserter error"
    );
    assert_eq!(
        format!("{}", fetch_error),
        "[ProtonClient]: Fetch failed with error: Fetch error"
    );
    assert_eq!(
        format!("{}", fetch_all_error),
        "[ProtonClient]: FetchAll failed with error: Fetch all error"
    );
    assert_eq!(
        format!("{}", fetch_one_error),
        "[ProtonClient]: FetchOne failed with error: Fetch one error"
    );
    assert_eq!(
        format!("{}", fetch_optional_error),
        "[ProtonClient]: FetchOption failed with error: Fetch optional error"
    );
}
