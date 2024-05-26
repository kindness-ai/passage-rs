# Changelog

## 0.2.0 (2024-05-26)

- Changed `passage.user()` to `passage.users()` to accurately reflect the correct endpoint name

## 0.1.4 (2024-05-26)

- Resubmitting to crates.io

## 0.1.3 (2024-05-26)

- Added the support for the Passage auth Users endpoint (get_user, create_user)
- Made models public, the naming and location is likely to change in a future release
- Removed api_key config option as this crate should never need it unless we add support for 
- Updated to jsonwebtoken 9.3.0

