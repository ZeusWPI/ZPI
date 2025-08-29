# Zeus Profile Images

let people upload profile images and make them available on a public endpoint.

# running

```bash
cp .env.example .env
cargo run
```

# how it works

login with zauth, upload an image

# public endpoints

GET `/image/{zauth_user_id}` -> gives that user's profile image

You can give any optional query parameters to change which reply you receive:

| query param | value                        | explanation                                 | default |
| ----------- | ---------------------------- | ------------------------------------------- | ------- |
| placeholder | `true` / `false`             | return a placeholder when user has no image | `true`  |
| size        | `64` / `128` / `256` / `512` | square resolution of the image              | `256`   |

> [!NOTE]
> `size` will return the next largest image if requested value is not available

# config

see [env example](./.env.example) for an example
| env var | explaination |
| --------------------- | ---------------------------------------------------------- |
| `ZAUTH_URL` | base url of zauth |
| `ZAUTH_CALLBACK_PATH` | callback url of ZPI |
| `ZAUTH_CLIENT_ID` | zauth client id |
| `ZAUTH_CLIENT_SECRET` | zauth client secret |
| `IMAGE_PATH` | path where the profile images will be stored and retrieved |
