use std::ffi::CString;

use curl::{curl_easy_perform, CURLoption_CURLOPT_URL};

mod curl;

fn main() {
    unsafe {
        let url = CString::new("https://insou.dev").expect("no NUL byte in str");
        let url_str = url.as_c_str().as_ptr();

        let curl = curl::curl_easy_init();
        if !curl.is_null() {
            curl::curl_easy_setopt(curl, CURLoption_CURLOPT_URL, url_str);

            assert_eq!(curl_easy_perform(curl), 0);
        }
    }
}
