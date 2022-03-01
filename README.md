
# Spotty

A player for those who wants to listen to music on Spotify right from the terminal.


## Features

- Fast 
- Memory friendly
- Works as a Spotify device
- Works as a terminal music player


## Usage/Examples

```bash
$ spotty connect
$ spotty playlists
$ spotty search
$ spotty toggle play-pause
$ spotty previous
$ spotty next
$ spotty mute
$ spotty list <Object_URI>
```


## Installation

Install spotty with cargo

```bash
  cargo install spotty
```
    
## Documentation

[WIP] [Documentation](https://linktodocumentation)


## Roadmap

- Implement CLI structure
- Implement Spotify connection with token refresh
- Integrate spotifyd daemon on spotty
- Implement basic playback control


## Contributing

Contributions are always welcome!

See [WIP]`contributing.md` for ways to get started.

Please adhere to this project's [WIP]`code of conduct`.


## FAQ

#### What makes spotty different from spotifyd?

Spotifyd is a daemon which purpose is to be a device for streaming Spotify musics and podcasts, but it doesn't support playback control. Spotty is aimed to be both a Spotify daemon and a Spotify playback player.


## License

[Apache License 2.0](https://choosealicense.com/licenses/apache-2.0/)

