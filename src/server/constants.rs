// // https://developer.mozilla.org/en-US/docs/Web/HTTP/Methods
// pub enum HttpMethod {
//     GET,
//     HEAD,
//     POST,
//     PUT,
//     DELETE,
//     CONNECT,
//     OPTIONS,
//     TRACE,
//     PATCH,
// }
//
// // https://developer.mozilla.org/en-US/docs/Web/HTTP/Status
// #[derive(Debug)]
// enum HttpStatusCode {
//     // Informational responses (100 – 199)
//     Continue = 100,
//     SwitchingProtocols = 101,
//     EarlyHints = 103,
//
//     // Successful responses (200 – 299)
//     Ok = 200,
//     Created = 201,
//     Accepted = 202,
//     NonAuthoritativeInformation = 203,
//     NoContent = 204,
//     ResetContent = 205,
//     PartialContent = 206,
//
//     // Redirection messages (300 – 399)
//     MultipleChoices = 300,
//     MovedPermanently = 301,
//     Found = 302,
//     SeeOther = 303,
//     NotModified = 304,
//     UseProxy = 305, // Deprecated
//     TemporaryRedirect = 307,
//     PermanentRedirect = 308,
//
//     // Client error responses (400 – 499)
//     BadRequest = 400,
//     Unauthorized = 401,
//     PaymentRequired = 402, // Experimental
//     Forbidden = 403,
//     NotFound = 404,
//     MethodNotAllowed = 405,
//     NotAcceptable = 406,
//     ProxyAuthenticationRequired = 407,
//     RequestTimeout = 408,
//     Conflict = 409,
//     Gone = 410,
//     LengthRequired = 411,
//     PreconditionFailed = 412,
//     PayloadTooLarge = 413,
//     URITooLong = 414,
//     UnsupportedMediaType = 415,
//     RangeNotSatisfiable = 416,
//     ExpectationFailed = 417,
//     MisdirectedRequest = 421,
//     UpgradeRequired = 426,
//     PreconditionRequired = 428,
//     TooManyRequests = 429,
//     RequestHeaderFieldsTooLarge = 431,
//     UnavailableForLegalReasons = 451,
//
//     // Server error responses (500 – 599)
//     InternalServerError = 500,
//     NotImplemented = 501,
//     BadGateway = 502,
//     ServiceUnavailable = 503,
//     GatewayTimeout = 504,
//     HTTPVersionNotSupported = 505,
//     VariantAlsoNegotiates = 506,
//     NotExtended = 510,
//     NetworkAuthenticationRequired = 511,
// }
//
// // AI was invented for a reason :)
// impl HttpStatusCode {
//     fn description(&self) -> &'static str {
//         match self {
//             // Informational responses
//             Self::Continue => "Continue",
//             Self::SwitchingProtocols => "Switching Protocols",
//             Self::EarlyHints => "Early Hints",
//
//             // Successful responses
//             Self::Ok => "OK",
//             Self::Created => "Created",
//             Self::Accepted => "Accepted",
//             Self::NonAuthoritativeInformation => "Non-Authoritative Information",
//             Self::NoContent => "No Content",
//             Self::ResetContent => "Reset Content",
//             Self::PartialContent => "Partial Content",
//
//             // Redirection messages
//             Self::MultipleChoices => "Multiple Choices",
//             Self::MovedPermanently => "Moved Permanently",
//             Self::Found => "Found",
//             Self::SeeOther => "See Other",
//             Self::NotModified => "Not Modified",
//             Self::UseProxy => "Use Proxy",
//             Self::TemporaryRedirect => "Temporary Redirect",
//             Self::PermanentRedirect => "Permanent Redirect",
//
//             // Client error responses
//             Self::BadRequest => "Bad Request",
//             Self::Unauthorized => "Unauthorized",
//             Self::PaymentRequired => "Payment Required",
//             Self::Forbidden => "Forbidden",
//             Self::NotFound => "Not Found",
//             Self::MethodNotAllowed => "Method Not Allowed",
//             Self::NotAcceptable => "Not Acceptable",
//             Self::ProxyAuthenticationRequired => "Proxy Authentication Required",
//             Self::RequestTimeout => "Request Timeout",
//             Self::Conflict => "Conflict",
//             Self::Gone => "Gone",
//             Self::LengthRequired => "Length Required",
//             Self::PreconditionFailed => "Precondition Failed",
//             Self::PayloadTooLarge => "Payload Too Large",
//             Self::URITooLong => "URI Too Long",
//             Self::UnsupportedMediaType => "Unsupported Media Type",
//             Self::RangeNotSatisfiable => "Range Not Satisfiable",
//             Self::ExpectationFailed => "Expectation Failed",
//             Self::MisdirectedRequest => "Misdirected Request",
//             Self::UpgradeRequired => "Upgrade Required",
//             Self::PreconditionRequired => "Precondition Required",
//             Self::TooManyRequests => "Too Many Requests",
//             Self::RequestHeaderFieldsTooLarge => "Request Header Fields Too Large",
//             Self::UnavailableForLegalReasons => "Unavailable For Legal Reasons",
//
//             // Server error responses
//             Self::InternalServerError => "Internal Server Error",
//             Self::NotImplemented => "Not Implemented",
//             Self::BadGateway => "Bad Gateway",
//             Self::ServiceUnavailable => "Service Unavailable",
//             Self::GatewayTimeout => "Gateway Timeout",
//             Self::HTTPVersionNotSupported => "HTTP Version Not Supported",
//             Self::VariantAlsoNegotiates => "Variant Also Negotiates",
//             Self::NotExtended => "Not Extended",
//             Self::NetworkAuthenticationRequired => "Network Authentication Required",
//         }
//     }
// }
