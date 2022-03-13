# monster api

because everything needs an api now

## setup

requires a [working rust toolchain](https://rustup.rs/).

## data acquisition

```sh
$ cargo run --bin scraper
```

`db.sqlite3` will be created in the project root, containing all the highly
useful information the tool found in the site sources.

## api

```sh
$ cargo run
```

### endpoints

#### category list

`GET /categories`

#### category details

`GET /categories/<slug>`

#### products in category

`GET /categories/<slug>/products`

#### product list

`GET /products`

#### product details

`GET /products/<slug>`
