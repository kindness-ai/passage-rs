# Changelog

## [0.4.1](https://github.com/kindness-ai/passage-rs/compare/v0.4.0...v0.4.1) - 2025-02-16

### Other

- Update deps

## 0.4.0 (2024-05-26)

- Fixed: `authenticate().authenticate_token` error not wrapped in `PassageError`

## 0.3.0 (2024-05-26)

- Added audience validation for JWTs. This requires either setting the PASSAGE_APP_AUTH_ORIGIN environment variable or configuring it within the application settings.
- The generic configuration of the Passage client has been removed. This feature is no longer needed as originally planned.

## 0.2.1 (2024-05-26)

- Temporarily added `users().get_user_by_id` until we have the `passage-manage` crate published

## 0.2.0 (2024-05-26)

- Changed `passage.user()` to `passage.users()` to accurately reflect the correct endpoint name

## 0.1.4 (2024-05-26)

- Resubmitting to crates.io

## 0.1.3 (2024-05-26)

- Added the support for the Passage auth Users endpoint (get_user, create_user)
- Made models public, the naming and location is likely to change in a future release
- Removed api_key config option as this crate should never need it unless we add support for
- Updated to jsonwebtoken 9.3.0
