MediaServer Clipper ![](https://github.com/paveyry/mediaserver-clipper/actions/workflows/build.yml/badge.svg) ![](https://github.com/paveyry/mediaserver-clipper/actions/workflows/docker.yml/badge.svg)
==============================================

Description
-----------

This application written in Rust serves a web application that allows users to extract clips from
videos in their media server (Plex, Jellyfin, etc.). 

This is loosely inspired from jo-nike's clipplex (https://github.com/jo-nike/clipplex). However,
unlike clipplex, mediaserver-clipper does not rely on any specific mediaserver API and uses the
path to the file directly. This makes it slightly less easy to use but much simpler to setup and
it makes it compatible with absolutely any media server.

[Demo](https://github.com/paveyry/mediaserver-clipper/assets/3884900/22eae62c-09e2-4580-8d9c-c60b60fa8baf)


How to use
----------

Media Server Clipper supports two ways to select the source for a clip:

* Enter the direct path to the file. On Plex Web, you can click the triple-dot button while watching
a video and then `Get Info` in order to get the absolute path to the file. If your Plex instance is
running on the same system as MediaServer Clipper or if you mounted the media volume the same way
both in your Plex instance, then this path will also work for the Clipper app to access it.

* Search for files indexed by Media Server Clipper. This can only find files in the libraries that
you have registered at setup (unlike direct path which can access any readable file), but it allows
you to find a file by just writing a few letters. 

MediaServer Clipper will automatically list audio and subtitle tracks and you can select which
audio you want to use and whether or not you want to burn in subtitles in the clip.

### Multiple video sources

The example shows how to use it with Plex, but because this will work with any absolute path to any video,
it is possible to use it with multiple sources to clip videos from anything on the drive.

For example, if using docker, you can mount the `Plex` library to `/media` in the container, and some other
directory with your personal video files to `/videos` for example. Then if you paste a path to
`/videos/holiday_summer_2023.mp4` in the app, it will be able to clip it just like plex videos if the
path was `/media/some_movie.mp4`.

You can even combine it with another app that can download videos on demand. If the directory it stores its
downloads is mounted in the Clipper container, it will be able to make clips from it.

How the search engine works
---------------------------

Files are indexed when the application start and a refresh of the index can be triggered using the button at the top of the search results later.

You can put several words in your search query separated by whitespace. The search engine will return any file for which the path (either the file name or any of its parent directories) contains **all** the words of your search. Search is case-insensitive. For example, with the following library:

    library
    ├── An.Awesome.TV.Show.x264.EN
    │   ├── S01
    │   │   ├── E01.mp4
    │   │   ├── E02.mp4
    │   │   └── E03.mp4
    │   └── S02
    │       ├── E01.mp4
    │       ├── E02.mp4
    │       └── E03.mp4
    └── The.Incredible.Series.x264.EN
        ├── S01
        │   ├── E01.mp4
        │   ├── E02.mp4
        │   └── E03.mp4
        └── S02
            ├── E01.mp4
            ├── E02.mp4
            └── E03.mp4

Searching for `awe` will return all episodes from both seasons of "An Awesome TV Show".

Searching for `awe s01` will return only the season 1 episodes of "An Awesome TV Show".

Searching for `awe s01 e02` will return episode s01e02 of "An Awesome TV Show".

Searching for `incred s01 e02` will return episode s01e02 of "The Incredible Series".

Searching for `s01 e02` will return episodes s01e02 of both shows.



Environment Variables
---------------------

All environment variables have a default and are optional, but setting these allows you to customize the
behaviour of the app.

* `APP_NAME`: defines the name of the app on the webpage. Default is `Media Server Clipper`.
* `OUTPUT_PATH`: defines the directory in which the app will store the clips. Default is the `output`
directory where the app is started from (`/app/output` in docker container).
* `PUBLIC_LINK_PREFIX`: defines a different base URL to the clips directory that will be used when
clicking the `Share` button for a clip. This is especially useful if you want to protect the MediaServer
Clipper instance behind a htpassword but wish to have a public static file server to share the clips with
people without giving your credentials (see docker-compose example below). Default is to link to the clip
hosted by the Media Server Clipper (so the `share` and `link` buttons will do the same thing).
* `MAX_CLIP_DURATION`: defines the maximum allowed duration (in seconds) of the clips. Default is 600 (10 minutes).
* `MAX_QUEUE_SIZE`: number of jobs in queue after which the Clipper will reject new clip jobs. This does
not count finished clips, only pending ones. Default is 4.
* `SEARCH_DIRS`: list (comma-separated) of paths in which the search engine should be indexing files. If empty or not set, the search engine is disabled and the search field does not appear in the app. Default is empty. Example: `SEARCH_DIRS=/media,/personal/videos,/jellyfin/media`
* `SEARCH_FILE_EXTS`: list (comma-separated) of file extensions (without including the dot) that should be indexed by the search engine. If empty or not set, all files will be indexed. Default is empty (no filtering). Example: `SEARCH_FILE_EXT=mp4,mkv,avi,mov`

Running from source
-------------------

To run the app locally, install the rust toolchain (https://rustup.rs/) and run:

    cargo run --release

You need to have `ffmpeg` and the `DejaVu` fonts installed.

Docker
------

Docker is the recommended approach to run this application. Use this command to run the image
(this exposes it on port 9987):

    docker run -d --name clipper -p 9987:8000 -v ./media:/media -v ./clips:/app/output paveyry/mediaserver-clipper:latest

Docker-compose examples
-----------------------

### Standalone (`PUBLIC_LINK_PREFIX` is not set)

```yaml
plex:
#[...]
volumes:
    - ./media:/media

clipper:
    image: paveyry/mediaserver-clipper:latest
    ports:
        - 9987:8000
    environment:
        - SEARCH_DIRS=/media
    volumes:
        - ./media:/media # mount it the same way as in plex/jellyfin
        - ./clips:/app/output
```

### With an external server for sharing (`PUBLIC_LINK_PREFIX` is set)

```yaml
plex:
#[...]
volumes:
    - ./media:/media

clipper: # This can be protected by a htpassword
    image: paveyry/mediaserver-clipper:latest
    ports:
        - 9987:8000
    environment:
        - SEARCH_DIRS=/media
        - PUBLIC_LINK_PREFIX=https://yourdomain.tld:9988 # Share links wil link to static_clips
    volumes:
        - ./media:/media
        - ./clips:/app/output

public_clips: # This has read-only access so it can be safely exposed without auth
    image: nginx
    ports:
        - 9988:80 # this is https://yourdomain.tld:9988 linked in PUBLIC_LINK_PREFIX
    volumes:
        - ./clips:/usr/share/nginx/html:ro
```

Note that the `download` and `link` buttons will still link to the clipper url, only
the `share` button use the URL from `PUBLIC_LINK_PREFIX` (the `static_clips` nginx instance in this example)

Development Roadmap (upcoming improvements)
-------------------------------------------

From highest to lowest priority:

* Search engine to lookup files without having the exact path
* Sorting clips in grid from most recent to oldest
* Better error messages when ffprobe and ffmpeg fail
* Selecting clip resolution
* Documentation
* Better logging
* Get rid of std::thread and use rocket's tokio runtime instead
* Asynchronously detect changes in the pending queue from front-end without refresh

Appendices
----------

### Getting direct link to file in Plex web

![Get File Info in Plex](https://github.com/paveyry/mediaserver-clipper/assets/3884900/9018bcda-649e-4179-991b-5de4d11acd17)

![Copy file path](https://github.com/paveyry/mediaserver-clipper/assets/3884900/b9b7269e-dfb2-439e-b989-6f630e0280b3)
