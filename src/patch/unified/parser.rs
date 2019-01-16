use std::fs::Permissions;
    #[fail(display = "Invalid mode: \"{}\"", mode_str)]
    BadMode { mode_str: String },
    BadMode,
            c if c == ParseErrorCode::BadMode as u32 => {
                ParseError::BadMode { mode_str: place_as_string }
fn parse_mode(input: CompleteByteSlice) -> IResult<CompleteByteSlice, u32> {
    if digits.len() != 6 { // This is what patch requires, but otherwise it fallbacks to 0, so maybe we should too?
        return Err(nom::Err::Failure(error_position!(input, nom::ErrorKind::Custom(ParseErrorCode::BadMode as u32))));
    let mode_str = std::str::from_utf8(&digits).unwrap(); // NOTE(unwrap): We know it is just digits 0-7, so it is guaranteed to be valid UTF8.
    match u32::from_str_radix(mode_str, 8) {
        Ok(number) => Ok((input, number)),
        Err(_) => Err(nom::Err::Failure(error_position!(input, nom::ErrorKind::Custom(ParseErrorCode::BadMode as u32)))),
fn test_parse_mode() {
    assert_parsed!(parse_mode, b"123456", 0o123456);
    assert_parsed!(parse_mode, b"012345", 0o12345);
    assert_parsed!(parse_mode, b"   123456", 0o123456);
    assert_parsed!(parse_mode, b"   012345", 0o12345);

    assert_parsed!(parse_mode, b"100755", 0o100755);
    assert_parsed!(parse_mode, b"100644", 0o100644);

    assert_parse_error_code!(parse_mode, b"100aaa", ParseErrorCode::BadMode as u32);
    assert_parse_error_code!(parse_mode, b"1", ParseErrorCode::BadMode as u32);
    assert_parse_error_code!(parse_mode, b"10000000", ParseErrorCode::BadMode as u32);
    assert_parse_error_code!(parse_mode, b"1000000000000000000000000000", ParseErrorCode::BadMode as u32);
    OldMode(u32),
    NewMode(u32),
    DeleteFileMode(u32),
    NewFileMode(u32),
        do_parse!(tag!(s!(b"old mode "))         >> mode: parse_mode >> newline >> (MetadataLine::OldMode(mode))) |
        do_parse!(tag!(s!(b"new mode "))         >> mode: parse_mode >> newline >> (MetadataLine::NewMode(mode))) |
        do_parse!(tag!(s!(b"delete file mode ")) >> mode: parse_mode >> newline >> (MetadataLine::DeleteFileMode(mode))) |
        do_parse!(tag!(s!(b"new file mode "))    >> mode: parse_mode >> newline >> (MetadataLine::NewFileMode(mode)))
    assert_parsed!(parse_metadata_line, b"old mode 100644\n",         OldMode(0o100644));
    assert_parsed!(parse_metadata_line, b"new mode 100644\n",         NewMode(0o100644));
    assert_parsed!(parse_metadata_line, b"delete file mode 100644\n", DeleteFileMode(0o100644));
    assert_parsed!(parse_metadata_line, b"new file mode 100644\n",    NewFileMode(0o100644));
    old_permissions: Option<Permissions>,
    new_permissions: Option<Permissions>,
    /// Do we have enough metadata to build hunk-less filepatch?
    pub fn can_build_hunkless_filepatch(&self) -> bool {
        // Renaming?
        if self.old_filename.is_some() && self.new_filename.is_some() &&
           self.rename_from && self.rename_to
        {
            return true;
        }

        // Mode changing?
        if (self.old_filename.is_some() || self.new_filename.is_some()) &&
            self.new_permissions.is_some() // Only the new one is needed for patch
        {
            return true;

        false
    pub fn build_filepatch<'a>(self, hunks: HunksVec<'a, &'a [u8]>) -> Option<TextFilePatch<'a>> {
        let mut builder = FilePatchBuilder::<&[u8]>::default();
        // Set the kind
        builder.kind(self.recognize_kind(&hunks));

        // Set the filenames
        let old_filename = match self.old_filename {
            Some(Filename::Real(old_filename)) => Some(old_filename),
            _ => None,
        };
        let new_filename = match self.new_filename {
            Some(Filename::Real(new_filename)) => Some(new_filename),
            _ => None,
        };

        if self.rename_from && self.rename_to {
            // If it is renaming patch, we must have both filenames
            if old_filename.is_none() || new_filename.is_none() {
                return None;
            }

            builder.is_rename(true);
            // If it is non-renaming patch, we must have at least one filename
            if old_filename.is_none() && new_filename.is_none() {
                return None;
            }

        builder.old_filename(old_filename);
        builder.new_filename(new_filename);

        // Set the permissions
        builder.old_permissions(self.old_permissions);
        builder.new_permissions(self.new_permissions);

        // Set the hunks
        builder.hunks(hunks);

        // Build
        Some(builder.build().unwrap()) // NOTE(unwrap): It would be our bug if we didn't provide all necessary values.
    }

    pub fn build_hunkless_filepatch<'a>(self) -> Option<TextFilePatch<'a>> {
        self.build_filepatch(HunksVec::new())
#[cfg(unix)]
fn permissions_from_mode(mode: u32) -> Option<Permissions> {
    use std::os::unix::fs::PermissionsExt;
    Some(Permissions::from_mode(mode))
}

#[cfg(not(unix))]
fn permissions_from_mode(mode: u32) -> Option<Permissions> {
    static WARN_ONCE: std::sync::Once = std::sync::Once::new();

    WARN_ONCE.call_once(|| {
        eprintln!("Permissions are ignored on non-unix systems!");
    });

    None
}

                // could be still valid patch that only renames a file or
                // changes permissions... So lets check for that.
                if metadata.can_build_hunkless_filepatch() {
                    // Note that in this case we don't set `input = input_`, because we don't want to consume the GitDiffSeparator

                    return Ok((input, tmp_metadata.build_hunkless_filepatch().unwrap())); // NOTE(unwrap): It must succeed since we checked with can_build_hunkless_filepatch.
            Metadata(OldMode(mode)) |
            Metadata(DeleteFileMode(mode)) => {
                metadata.old_permissions = permissions_from_mode(mode);
            }
            Metadata(NewMode(mode)) |
            Metadata(NewFileMode(mode)) => {
                metadata.new_permissions = permissions_from_mode(mode);
            }

    let filepatch = metadata.build_filepatch(hunks).ok_or_else(
    assert_eq!(file_patch.old_filename(), Some(&PathBuf::from("filename1")));
    assert_eq!(file_patch.new_filename(), Some(&PathBuf::from("filename1")));
    assert_eq!(file_patch.old_permissions(), None);
    assert_eq!(file_patch.new_permissions(), None);
    assert_eq!(file_patch.old_filename(), None);
    assert_eq!(file_patch.new_filename(), Some(&PathBuf::from("filename1")));
    assert_eq!(file_patch.old_permissions(), None);
    assert_eq!(file_patch.new_permissions(), None);
    assert_eq!(file_patch.old_filename(), Some(&PathBuf::from("filename1")));
    assert_eq!(file_patch.new_filename(), Some(&PathBuf::from("filename1")));
    assert_eq!(file_patch.old_permissions(), None);
    assert_eq!(file_patch.new_permissions(), None);
    assert_eq!(file_patch.old_filename(), Some(&PathBuf::from("filename1")));
    assert_eq!(file_patch.old_permissions(), None);
    assert_eq!(file_patch.new_permissions(), None);
    assert_eq!(file_patch.old_filename(), Some(&PathBuf::from("filename1")));
    assert_eq!(file_patch.new_filename(), Some(&PathBuf::from("filename1")));
    assert_eq!(file_patch.old_permissions(), None);
    assert_eq!(file_patch.new_permissions(), None);