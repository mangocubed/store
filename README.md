# Mango³ Store

Digital distribution platform.

### Build Requirements

- Node.js 22.x
- Rust 1.91.x
- PostgreSQL 17.x
- Redis 8.x

## Run Requirements

- PostgreSQL 17.x
- Redis 8.x

## Environment variables

| Name                           | Type    | Default                                           | Client-side |
| ------------------------------ | ------- | ------------------------------------------------- | ----------- |
| APP_SERVER_URL                 | String  |                                                   | Yes         |
| APP_OLD_TOKENS                 | Array   | []                                                | No          |
| APP_TOKEN                      | String  |                                                   | Yes         |
| APP_TITLE                      | String  | Mango³                                            | Yes         |
| AUTH_CLIENT_ID                 | String  |                                                   | Yes         |
| AUTH_CLIENT_PROVIDER_API_URL   | String  |                                                   | No          |
| AUTH_CLIENT_PROVIDER_APP_URL   | String  |                                                   | Yes         |
| AUTH_CLIENT_SECRET             | String  |                                                   | No          |
| AUTH_CLIENT_WEBHOOK_SECRET     | String  |                                                   | No          |
