use exam_q2_25T3_lib::{get_context, search_library, SearchResult};

fn main() {
    // Test get_context
    let book = "Hello world\nRust is great for systems programming\nGoodbye world";
    let query = "systems";
    
    if let Some(context) = get_context(book, query) {
        println!("Found context: {context}");
    }

    // Test search_library
    let books = vec![
        "Once upon the time\nA story began",
        "In the beginning\nThere was code",
    ];
    
    let results = search_library(&books, "the");
    
    println!("Search results:");
    for SearchResult { query, context } in &results {
        println!("  query: '{query}', context: '{context}'");
    }
}
