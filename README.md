# Crabby

A player for those who wants to listen to music on Spotify right from the terminal.


## Features

- Fast 
- Memory friendly
- Works as a Spotify device
- Works as a terminal music player


## Usage/Examples

```bash
$ crabby connect
$ crabby disconnect
$ crabby get-playback
$ crabby list
$ crabby next
$ crabby pause
$ crabby play
$ crabby previous
$ crabby search
$ crabby status
$ crabby stop
$ crabby suffle
```


## Installation

Install crabby with cargo

```bash
  cargo install crabby
```
    
## Documentation

[WIP] [Documentation](https://linktodocumentation)


## Roadmap

- Implement CLI structure
- Implement Spotify connection with token refresh
- Integrate spotifyd daemon on crabby
- Implement basic playback control


## Contributing

Contributions are always welcome!

See [WIP]`contributing.md` for ways to get started.

Please adhere to this project's [WIP]`code of conduct`.


## FAQ

#### What makes crabby different from spotifyd?

Spotifyd is a daemon which purpose is to be a device for streaming Spotify musics and podcasts, but it doesn't support playback control. crabby is aimed to be both a Spotify daemon and a Spotify playback player.


## License

[Apache License 2.0](https://choosealicense.com/licenses/apache-2.0/)

