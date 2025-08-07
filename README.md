# Zeus Profile Images

let people upload profile imagess and make them available on an endpoint.

# running

```
cp .env.example .env
cargo run
```

# how it works

login with zauth, upload an image

# public endpoints

GET `/image/{zauth_user_id}` -> gives that user's profile image

> [!NOTE]
> by default this returns a 404 if the user does not have an image.
> if you want a placeholder image instead, add `?placeholder=true`

# config

see [env example](./.env.example) for an example
| env var | explaination |
| --------------------- | ---------------------------------------------------------- |
| `ZAUTH_URL` | base url of zauth |
| `ZAUTH_CALLBACK_PATH` | callback url of ZPI |
| `ZAUTH_CLIENT_ID` | zauth client id |
| `ZAUTH_CLIENT_SECRET` | zauth client secret |
| `IMAGE_PATH` | path where the profile images will be stored and retrieved |
