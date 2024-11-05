#include <stdio.h>
#include <curl/curl.h>
 
int main(void) {
	CURL *curl = curl_easy_init();
	if (curl) {
		curl_easy_setopt(curl, CURLOPT_URL, "https://www.google.com");

		/* Perform the request, res gets the return code */
		CURLcode res = curl_easy_perform(curl);

		/* Check for errors */
		if (res != CURLE_OK) {
			fprintf(stderr, "curl failed: %s\n", curl_easy_strerror(res));
		}
	}

	return 0;
}
