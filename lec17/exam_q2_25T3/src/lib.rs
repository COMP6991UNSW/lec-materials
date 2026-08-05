use require_lifetimes::require_lifetimes;

/// Returns the line from `book` that contains `query`, if any.
/// (3 marks)
#[require_lifetimes]
pub fn get_context<'book, 'query>(book: &'book str, query: &'query str) -> Option<&'book str> {
    for line in book.lines() {
        if line.contains(query) {
            return Some(line);
        }
    }
    None
}

/// A search result containing the search term and its surrounding context.
/// (2 marks)
pub struct SearchResult<'query, 'context> {
    pub query: &'query str,
    pub context: &'context str,
}

/// Searches for `query` across all books in the library.
/// Returns a SearchResult for each line containing the query.
/// (5 marks)
#[require_lifetimes]
pub fn search_library<'library, 'book, 'query>(
    library: &'library Vec<&'book str>,
    query: &'query str,
) -> Vec<SearchResult<'query, 'book>>
{
    let mut results = vec![];
    for book in library {
        for line in book.lines() {
            if line.contains(query) {
                results.push(SearchResult {
                    query,
                    context: line,
                });
            }
        }
    }
    results
}
