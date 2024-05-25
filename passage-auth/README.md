<div align="center">
  <img width="500px" src="./github/passage.jpg" />
</div>
<h1 align="center"> passage-auth </h1>
<p align="center"> Passage Authentication Library for Rust! 🦀 </p>
<div align="center">
    <a href="https://crates.io/crates/passage-auth">
      <img src="https://img.shields.io/crates/v/passage-auth.svg" />
    </a>
</div>
</div>

## Overview

`passage-auth` is an unofficial Rust library for [Passage by 1Password](https://passage.1password.com/).

- It's strictly following the [Passsage Authentication API](https://docs.passage.id/api-docs/authentication-api).

- Current features:
  - [x] Apps
  - [x] Authenticate
  - [x] Currentuser
  - [x] JWKS
  - [x] Login
  - [x] MagicLink
  - [x] OpenId
  - [x] OTP
  - [x] Register
  - [x] Tokens
  - [x] Users
  - [] OAuth2
  - [] Passkey Readiness

Models were automatically generated thanks to [OpenAPI Generator](https://openapi-generator.tech). The rest of the auth API was built by your friends at [Kindness](https://kindness.ai). With some prior art for the validation function from our friend [Rob Yoder](https://github.com/robyoder/passage-rust/blob/main/src/lib.rs).

> [!WARNING]  
> This crate is brand new and not all features have been tested or documented.
> Expect breaking changes.

## Usage

The library reads your Passage APP ID from the environment variable `PASSAGE_APP_ID` and optionally a API key from `PASSAGE_API_KEY`. But you can also pass a Config object or use the Config builder to create the Passsage client.

# Verify a JWT

```rust
// Create a new passage instance
let passage = Passage::with_config(Config::default().with_app_id(APP_ID.to_string()));
// Get the JWKS for your app containing the use the JWK's public key to verify a user's JWT.
let response: JwkResponse = passage.jwks().get_jwks().await?;
// Verify a user's JWT

```

# Get information about or modify

Only you have verified the user, your app can do it's thing, but this library is almost feature complete with the Passage auth API, so you can do a lot more, for example:

```rust
let passage = Passage::with_config(
 Config::default()
  .with_app_id(APP_ID.to_string())
  .with_user_bearer_token(JWT.to_string()),
);
let response: CurrentUserResponse = passage.current_user().get_current_user().await?;

CurrentUserResponse { user: CurrentUser { created_at: "2024-05-25T12:14:42.420571Z", email: "ted@tedlasso.org", email_verified: true, id: "AabRBkquedeVBxv9kFyfeXHI", last_login_at: "2024-05-25T14:27:53.825045Z", login_count: 3, phone: "", phone_verified: false, social_connections: UserSocialConnections { apple: None, github: None, google: None }, status: Active, updated_at: "2024-05-25T14:27:53.975632Z", user_metadata: None, webauthn: false, webauthn_devices: [], webauthn_types: [] } }
```

# Refresh or revoke refresh tokens

```rust
// APP ID loaded via environment vairable
let passage = Passage::new();

// Refresh tokens
let response = passage.tokens().refresh_auth_token(RefreshAuthTokenRequest{refresh_token}).await

// Revoke refresh token
let response = passage.tokens().revoke_refresh_token(refresh_token)
```

## License

This project is licensed under [MIT license](https://github.com/64bit/async-openai/blob/main/LICENSE).
