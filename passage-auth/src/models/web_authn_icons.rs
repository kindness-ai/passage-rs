use serde::{Deserialize, Serialize};

/// WebAuthnIcons : Contains the light and dark SVG icons that represent the
/// brand of those devices Values can be null or base64 encoded SVG. Example of
/// SVG output:
/// data:image/svg+xml;base64,
/// PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIGVuYWJsZS1iYWNrZ3JvdW5kPSJuZXcgMCAwIDE5MiAxOTIiIGhlaWdodD0iMjRweCIgdmlld0JveD0iMCAwIDE5MiAxOTIiIHdpZHRoPSIyNHB4Ij48cmVjdCBmaWxsPSJub25lIiBoZWlnaHQ9IjE5MiIgd2lkdGg9IjE5MiIgeT0iMCIvPjxnPjxwYXRoIGQ9Ik02OS4yOSwxMDZjLTMuNDYsNS45Ny05LjkxLDEwLTE3LjI5LDEwYy0xMS4wMywwLTIwLTguOTctMjAtMjBzOC45Ny0yMCwyMC0yMCBjNy4zOCwwLDEzLjgzLDQuMDMsMTcuMjksMTBoMjUuNTVDOTAuMyw2Ni41NCw3Mi44Miw1Miw1Miw1MkMyNy43NCw1Miw4LDcxLjc0LDgsOTZzMTkuNzQsNDQsNDQsNDRjMjAuODIsMCwzOC4zLTE0LjU0LDQyLjg0LTM0IEg2OS4yOXoiIGZpbGw9IiM0Mjg1RjQiLz48cmVjdCBmaWxsPSIjRkJCQzA0IiBoZWlnaHQ9IjI0IiB3aWR0aD0iNDQiIHg9Ijk0IiB5PSI4NCIvPjxwYXRoIGQ9Ik05NC4zMiw4NEg2OHYwLjA1YzIuNSwzLjM0LDQsNy40Nyw0LDExLjk1cy0xLjUsOC42MS00LDExLjk1VjEwOGgyNi4zMiBjMS4wOC0zLjgyLDEuNjgtNy44NCwxLjY4LTEyUzk1LjQxLDg3LjgyLDk0LjMyLDg0eiIgZmlsbD0iI0VBNDMzNSIvPjxwYXRoIGQ9Ik0xODQsMTA2djI2aC0xNnYtOGMwLTQuNDItMy41OC04LTgtOHMtOCwzLjU4LTgsOHY4aC0xNnYtMjZIMTg0eiIgZmlsbD0iIzM0QTg1MyIvPjxyZWN0IGZpbGw9IiMxODgwMzgiIGhlaWdodD0iMjQiIHdpZHRoPSI0OCIgeD0iMTM2IiB5PSI4NCIvPjwvZz48L3N2Zz4=
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebAuthnIcons {
    #[serde(rename = "light", deserialize_with = "Option::deserialize")]
    pub light: Option<String>,
    #[serde(rename = "dark", deserialize_with = "Option::deserialize")]
    pub dark: Option<String>,
}

impl WebAuthnIcons {
    /// Contains the light and dark SVG icons that represent the brand of those
    /// devices Values can be null or base64 encoded SVG. Example of SVG output:
    /// data:image/svg+xml;base64,
    /// PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIGVuYWJsZS1iYWNrZ3JvdW5kPSJuZXcgMCAwIDE5MiAxOTIiIGhlaWdodD0iMjRweCIgdmlld0JveD0iMCAwIDE5MiAxOTIiIHdpZHRoPSIyNHB4Ij48cmVjdCBmaWxsPSJub25lIiBoZWlnaHQ9IjE5MiIgd2lkdGg9IjE5MiIgeT0iMCIvPjxnPjxwYXRoIGQ9Ik02OS4yOSwxMDZjLTMuNDYsNS45Ny05LjkxLDEwLTE3LjI5LDEwYy0xMS4wMywwLTIwLTguOTctMjAtMjBzOC45Ny0yMCwyMC0yMCBjNy4zOCwwLDEzLjgzLDQuMDMsMTcuMjksMTBoMjUuNTVDOTAuMyw2Ni41NCw3Mi44Miw1Miw1Miw1MkMyNy43NCw1Miw4LDcxLjc0LDgsOTZzMTkuNzQsNDQsNDQsNDRjMjAuODIsMCwzOC4zLTE0LjU0LDQyLjg0LTM0IEg2OS4yOXoiIGZpbGw9IiM0Mjg1RjQiLz48cmVjdCBmaWxsPSIjRkJCQzA0IiBoZWlnaHQ9IjI0IiB3aWR0aD0iNDQiIHg9Ijk0IiB5PSI4NCIvPjxwYXRoIGQ9Ik05NC4zMiw4NEg2OHYwLjA1YzIuNSwzLjM0LDQsNy40Nyw0LDExLjk1cy0xLjUsOC42MS00LDExLjk1VjEwOGgyNi4zMiBjMS4wOC0zLjgyLDEuNjgtNy44NCwxLjY4LTEyUzk1LjQxLDg3LjgyLDk0LjMyLDg0eiIgZmlsbD0iI0VBNDMzNSIvPjxwYXRoIGQ9Ik0xODQsMTA2djI2aC0xNnYtOGMwLTQuNDItMy41OC04LTgtOHMtOCwzLjU4LTgsOHY4aC0xNnYtMjZIMTg0eiIgZmlsbD0iIzM0QTg1MyIvPjxyZWN0IGZpbGw9IiMxODgwMzgiIGhlaWdodD0iMjQiIHdpZHRoPSI0OCIgeD0iMTM2IiB5PSI4NCIvPjwvZz48L3N2Zz4=
    pub fn new(light: Option<String>, dark: Option<String>) -> WebAuthnIcons {
        WebAuthnIcons { light, dark }
    }
}
