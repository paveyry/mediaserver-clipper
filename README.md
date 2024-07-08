MediaServer Clipper ![](https://github.com/paveyry/mediaserver-clipper/actions/workflows/build.yml/badge.svg)
==============================================

Description
-----------

This application written in Rust serves a web application that allows users to extract clips from
videos in their media server (Plex, Jellyfin, etc.). 

This is loosely inspired from jo-nike's clipplex (https://github.com/jo-nike/clipplex). However,
unlike clipplex, mediaserver-clipper does not rely on any specific mediaserver API and uses the
path to the file directly. This makes it slightly less easy to use but much simpler to setup and
it makes it compatible with absolutely any media server.

How to use
----------

On Plex, you can click the triple-dot button while watching a video and then `Get Info` in order
to get the absolute path to the file. If your Plex instance is running on the same system as
MediaServer Clipper or if you mounted the media volume the same way both in your Plex instance,
then this path will also work for the Clipper app to access it.

MediaServer Clipper will automatically list audio and subtitle tracks and you can select which
audio you want to use and whether or not you want to burn in subtitles in the clip.

![Get File Info in Plex](https://github.com/paveyry/mediaserver-clipper/assets/3884900/9018bcda-649e-4179-991b-5de4d11acd17)

![Copy file path](https://github.com/paveyry/mediaserver-clipper/assets/3884900/b9b7269e-dfb2-439e-b989-6f630e0280b3)

![Paste in Media Server Clipper](https://github.com/paveyry/mediaserver-clipper/assets/3884900/eb3eb996-5d80-4415-bb29-5b2dc542da99)

![Configure the clip settings](https://github.com/paveyry/mediaserver-clipper/assets/3884900/6d8ed6a6-6c30-4328-8b81-f0ee8a97c8e2)

![Job is added to queue and the clip appears on the home page once finished](https://github.com/paveyry/mediaserver-clipper/assets/3884900/92ae6d47-9a04-45f0-b0ae-711522a2ba2e)

![Videos show in a grid](https://github.com/paveyry/mediaserver-clipper/assets/3884900/f26af4d4-75de-4fd7-921a-2046ddffd468)

### Multiple video sources

The example shows how to use it with Plex, but because this will work with any absolute path to any video,
it is possible to use it with multiple sources to clip videos from anything on the drive.

For example, if using docker, you can mount the `Plex` library to `/media` in the container, and some other
directory with your personal video files to `/videos` for example. Then if you paste a path to
`/videos/holiday_summer_2023.mp4` in the app, it will be able to clip it just like plex videos if the
path was `/media/some_movie.mp4`.

You can even combine it with another app that can download videos on demand. If the directory it stores its
downloads is mounted in the Clipper container, it will be able to make clips from it.


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

Running from source
-------------------

To run the app locally, install the rust toolchain (https://rustup.rs/) and run:

    cargo run --release

You need to have `ffmpeg` and the `DejaVu` fonts installed.

Docker
------

Docker is the recommended approach to run this application. Use this command to run the image
(this exposes it on port 9987):

    docker run -d --name paveyry/mediaserver-clipper -p 9987:8000 -v ./media:/media -v ./clips:/app/output mediaserver-clipper

Docker-compose examples
-----------------------

### Standalone (`PUBLIC_LINK_PREFIX` is not set)

```yaml
plex:
#[...]
volumes:
    - ./media:/media

mediaserverclipper:
    image: paveyry/mediaserver-clipper
    ports:
        - 9987:8000
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

mediaserverclipper: # This can be protected by a htpassword
    image: paveyry/mediaserver-clipper
    ports:
        - 9987:8000
    environment:
        - PUBLIC_LINK_PREFIX=https://yourdomain.tld:9988 # Share links wil link to static_clips
    volumes:
        - ./media:/media
        - ./clips:/app/output

static_clips: # This has read-only access so it can be safely exposed without auth
    image: nginx
    ports:
        - 9988:80 # this is https://yourdomain.tld:9988 linked in PUBLIC_LINK_PREFIX
    volumes:
        - ./clips:/usr/share/nginx/html:ro
```

Note that the `download` and `link` buttons will still link to the mediaserverclipper url, only
the `share` button use the URL from `PUBLIC_LINK_PREFIX` (the `static_clips` nginx instance here)

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
