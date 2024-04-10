# Secrets Cafe
Source of the web page [secrets.cafe](https://secrets.cafe/) which is a platform for people to share their stories anonymously.

Currently, this is not a finished product, it lacks many features and is filled with bugs. 
I am not a frontend developer, so the website lacks proper support for mobile devices and the JS scripts might have a lot of issues.

### Usage
If you want to run **secrets.cafe** locally I would recommend building it in a release mode without any features. <br>
`cargo run --release`

Running in the debug mode might cause a crash when doing some database operations.
It can be fixed by increasing the stack size with tokio runtime builder, but I did not implement it yet.

### Features
- `tls` - This is an internal feature used as a "_dependency_" of other features for serving data over HTTPS.
- `cloudflare` - Implementation of the TLS to work with Cloudflare certificates.
- `rate-limits` - Enables rate limits on some API calls.

### ToDo
This is a list of things to do before 1.0 release
- [ ] Test HTML sanitizer.
- [ ] Improve security.
- [ ] Rework frontend.
- [ ] Rework resource loading. _(?)_
- [x] Add rate limits.
- [ ] Add proper API documentation.
- [ ] Add post page browser.

### API
Check out the [API.md](API.md) file for details.

### Console
The backend accepts input commands that can be executed at runtime:
- > `debug` - Responds with Ok status
- > `reload frontend` - Reloads all resources from "frontend" directory.
- > `mod add <name> <pass> <tier>` - Adds a new moderator account.
  > ###### Example: `mod add User1 1234567 1`
- > `mod authorise <input> <tier>` - Changes mod's permissions tier.
  > ###### Example: `mod authorise mod:62345321545 3`

Tier numbers can be found at `src/database/types/tier.rs`

---
[secrets.cafe](https://secrets.cafe) © 2024 by [FssAy](https://github.com/FssAy) is licensed under [CC BY-NC 4.0](https://creativecommons.org/licenses/by-nc/4.0/?ref=chooser-v1)
