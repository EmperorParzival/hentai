initSidebarItems({"enum":[["HentaiError","Global error type for Hentai that catches all other types of errors that may occur. This still provides the original error message when printed."],["Website","In cases where the standard output domain nhentai.net may be restricted on the end user's network, an option can be provided to use nhentai.xxx URLs instead. Note that the `Hentai::new()` constructor will not work if the standard output domain is blocked. This is because nhentai.xxx does not reimplement nhentai's API and requests must still be made to the original URL."]],"struct":[["Hentai","The main object containing the formatted information retrieved from nhentai. The raw image data is converted to into image URLs. A brief explanation of each field is located below."],["Search",""],["Tag","Tags are used to categorize doujins. Each doujin published on nhentai has tags indicating its content and additional data like the language and artist. For example, the tag with the category field equal to \"artist\" contains the name by which the author is recognized on nhentai."],["Title","nhentai provides three different types of titles. The first one is `pretty`, which is a simple title meant to stand out. The `english` and `japanese` titles are also provided. These are more fleshed out versions of the `pretty` title."]],"type":[["Result","Shorthand for Result<T, HentaiError> meant to make catching errors slightly more simple."]]});