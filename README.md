MediaServer Clipper
===================

This application written in Rust serves a web application that allows users to extract clips from
videos in their media server (Plex, Jellyfin, etc.). 

This is loosely inspired from jo-nike's clipplex (https://github.com/jo-nike/clipplex). However,
unlike clipplex, mediaserver-clipper does not rely on any specific mediaserver API and uses the
path to the file directly. This makes it slightly less easy to use but much simpler to setup and
it makes it compatible with absolutely any media server.