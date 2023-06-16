# solostrike

![GitHub followers](https://img.shields.io/github/followers/leo848?label=Follow&style=social)
![GitHub](https://img.shields.io/github/license/leo848/solostrike)
![GitHub code size in bytes](https://img.shields.io/github/languages/code-size/leo848/solostrike)
![Website](https://img.shields.io/website?down_message=down&up_message=up&url=https%3A%2F%2Fleo848.github.io%2Fsolostrike)

## [Click here to play online](https://leo848.github.io/solostrike)

Train your chess skills by playing randomly-generated mate-in-1 puzzles as fast
as possible.

## Local installation

```shell
$ # Clone the repository
$ git clone https://github.com/leo848/solostrike
$ cd solostrike

$ # Optional: generate your own FEN file
$ # Visit the backend directory
$ cd matemovemasters
$ # Generate a JSON file of chess positions (your own!)
$ cargo run --release
$ cd ..

$ # Build frontend
$ cd frontend
$ npm install
$ npm run dev
```
