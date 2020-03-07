// Finding out what is valid in id3v2 tags turned out to be aggravating.
// It might be possible for these to be used, but they will not display well
// in most cases, so they'll probably be acceptable.
pub const END_OF_FIELD: char = '\u{1f}';
pub const END_OF_RECORD: char = '\u{1e}';
pub const END_OF_HEADER: char = '\u{1d}';
