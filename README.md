# bekenntnis

A little learning project to play with Rocket, Diesel and Askama.
This is heavily inspired by [Late night confessions](https://medium.com/perimeterx/late-night-confessions-building-a-website-using-rust-rocket-diesel-and-askama-part-1-aeccade43252) by Johnny Tordgeman.


## Setup

### Database

- Create a postgres database `bekenntnis` with a user `bekenntnis` and password `bekenntnis` (or whatever is appropriate for you).
- Create a file `.env` in the project root which is used by the diesel command-line tool as well as the project:
```
DATABASE_URL=postgres://bekenntnis:bekenntnis@localhost/bekenntnis
```
- Install a development library of the postgres client, e.g. `sudo apt install libpq-dev`
- Run `diesel setup && diesel migration run`.

### Server

- Modify [Rocket.toml](Rocket.toml) for configuration changes if required.
- Launch the rocket via `cargo run`.


## Usage

- Visit the site in your browser at http://localhost:7878 . `7878` means `rust` when typing on the phone.
- You will be rotated through the existing bekenntnisse, but can also add new ones.
  
### API

- POST: `curl -X POST http://localhost:7878/api/bekenntnis -H "Content-Type: application/json" -d '{"content": "Das MGT" }'`
- GET: `curl http://localhost:7878/api/bekenntnis -H "Content-Type: application/json"`