use exam_q2_25T3_lib::{get_context, search_library, SearchResult};

fn main() {
    // Test get_context
    let book = String::from("Hello world\nGoodbye world\nFarewell world");
    let context = {
        let query = String::from("Goodbye");
        get_context(&book, &query).unwrap()
    };
    
    println!("Context: {context}");

    // Test search_library
    let book1 = String::from("The quick brown fox\nJumps over the lazy dog");
    let book2 = String::from("A lazy afternoon\nWith lazy clouds");
    let results = {
        let library = vec![book1.as_str(), book2.as_str()];
        let query = "lazy";
        
        search_library(&library, query)
    };
    
    println!("Found {} lazy lines:", results.len());
    for SearchResult { query: q, context } in &results {
        println!("  '{q}' in: {context}");
    }

    let first_search_result_query = results.first().unwrap().query;
    drop(book1);
    drop(book2);
    assert_eq!(first_search_result_query, "lazy");
}
