# Ferret API
Very smol API for getting images of my ferrets

## Endpoints
The root route is /api

- `GET` /random - `{ url: "cdn_url" }`
- `GET` /ferret/{id} - `{ url: "cdn_url" }`
- `POST` /submit - `{ file: "file.xyz", token: "some-token-provided-by-me" }`

## Example

Getting a random image
```sh
curl \
  https://ferret-api.canarado.xyz/api/random
```

Submitting a ferret picture
```sh
curl \
  -F file="@~/images/someimage.png" \
  -F token="some-token-provided-by-me" \
  https://ferret-api.canarado.xyz/api/submit
```