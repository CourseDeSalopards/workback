
# workback

Backend for workadventure API

## Features

- Event
- Room variables edition

## Requirements

- WorkAdventure token

## Environment Variables

To run this project, you will need to add the following environment variables to your .env file
You can copy .env.example

`ROOMAPI_HOST` - Room API endpoint

`ROOMAPI_PORT` - Room API port

`ROOMAPI_PORT` - Room API port

`ROOM_URL` -  Room URL

`X_API_KEY` -  Your private token

## Installation

Install can be done using Cargo (rust package manager)

(Unoptimized)

```bash
  cargo build --profile=dev
```

(Optimized)

```bash
  cargo build --profile=release
```

## Run Locally

```bash
  cargo run
```

## Running Tests

To run tests, run the following command

```bash
  cargo test
```

To run only the integration tests

```bash
  cargo test --test integration
```

Integration tests require .env to be configured, the service need a database

## Authors

- [@valentin-barbotin](https://www.github.com/valentin-barbotin)

## License

[MIT](https://choosealicense.com/licenses/mit/)
