// Log downloads are returned as raw CSV bytes by the Forward Email API,
// not as JSON. There is no structured model to deserialize — callers
// receive the bytes directly and write them to a file or stream.
