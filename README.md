MediaServer Clipper
===================

This application written in Rust serves a web application that allows users to extract clips from
videos in their media server (Plex, Jellyfin, etc.). 

This is loosely inspired from jo-nike's clipplex (https://github.com/jo-nike/clipplex). However,
unlike clipplex, mediaserver-clipper does not rely on any specific mediaserver API and uses the
path to the file directly. This makes it slightly less easy to use but much simpler to setup and
it makes it compatible with absolutely any media server.

How to use
----------

On Plex, you can click the triple-dot button while watching a video and then `Get Info` in order to get the absolute path to the file.
If your Plex instance is running on the same system as MediaServer Clipper or if you mounted the media volume the same way both in your Plex instance, then this path will also work for the Clipper app to access it.

MediaServer Clipper will automatically list audio and subtitle tracks and you can select which audio you want to use and whether or not you want to burn in subtitles in the clip.

![Get File Info in Plex](https://private-user-images.githubusercontent.com/3884900/346305859-56947528-cc8e-44f8-9e8d-e74c01eb18c1.png?jwt=eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJnaXRodWIuY29tIiwiYXVkIjoicmF3LmdpdGh1YnVzZXJjb250ZW50LmNvbSIsImtleSI6ImtleTUiLCJleHAiOjE3MjAzMTQ5NzgsIm5iZiI6MTcyMDMxNDY3OCwicGF0aCI6Ii8zODg0OTAwLzM0NjMwNTg1OS01Njk0NzUyOC1jYzhlLTQ0ZjgtOWU4ZC1lNzRjMDFlYjE4YzEucG5nP1gtQW16LUFsZ29yaXRobT1BV1M0LUhNQUMtU0hBMjU2JlgtQW16LUNyZWRlbnRpYWw9QUtJQVZDT0RZTFNBNTNQUUs0WkElMkYyMDI0MDcwNyUyRnVzLWVhc3QtMSUyRnMzJTJGYXdzNF9yZXF1ZXN0JlgtQW16LURhdGU9MjAyNDA3MDdUMDExMTE4WiZYLUFtei1FeHBpcmVzPTMwMCZYLUFtei1TaWduYXR1cmU9YmJmODFhMDJhMTg1ZGJhMmViNDgxZGJiMTM5ZTk3MDA0ZTRjNTZjOGNkN2MzMzdjNDU5ZTExMzJkMTFmYWJmYyZYLUFtei1TaWduZWRIZWFkZXJzPWhvc3QmYWN0b3JfaWQ9MCZrZXlfaWQ9MCZyZXBvX2lkPTAifQ.vCkW2gyoMa7oZ_xELmvDnrv1EbsfAn3HOYS3oEaxwzs)

![Copy file path](https://private-user-images.githubusercontent.com/3884900/346305845-891b2b3b-4ebd-425d-be9d-901b9c486bcd.png?jwt=eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJnaXRodWIuY29tIiwiYXVkIjoicmF3LmdpdGh1YnVzZXJjb250ZW50LmNvbSIsImtleSI6ImtleTUiLCJleHAiOjE3MjAzMTQ5MjQsIm5iZiI6MTcyMDMxNDYyNCwicGF0aCI6Ii8zODg0OTAwLzM0NjMwNTg0NS04OTFiMmIzYi00ZWJkLTQyNWQtYmU5ZC05MDFiOWM0ODZiY2QucG5nP1gtQW16LUFsZ29yaXRobT1BV1M0LUhNQUMtU0hBMjU2JlgtQW16LUNyZWRlbnRpYWw9QUtJQVZDT0RZTFNBNTNQUUs0WkElMkYyMDI0MDcwNyUyRnVzLWVhc3QtMSUyRnMzJTJGYXdzNF9yZXF1ZXN0JlgtQW16LURhdGU9MjAyNDA3MDdUMDExMDI0WiZYLUFtei1FeHBpcmVzPTMwMCZYLUFtei1TaWduYXR1cmU9NjBkNTNhZjE3MzgxODhkYTg3MjYxOTBhOTk1MjhkZDVmNTdjOTJjZTExOWZlODgwMTUzN2M3NTliYzBmZDM2ZiZYLUFtei1TaWduZWRIZWFkZXJzPWhvc3QmYWN0b3JfaWQ9MCZrZXlfaWQ9MCZyZXBvX2lkPTAifQ.oQEjvcm1u7ioAW3juOitluj1bAuHH3gcb3Fe8jG2aps)

![Paste in Media Server Clipper](https://private-user-images.githubusercontent.com/3884900/346305880-52f9779f-52e1-4a9e-acb6-aecc0f0ad72b.png?jwt=eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJnaXRodWIuY29tIiwiYXVkIjoicmF3LmdpdGh1YnVzZXJjb250ZW50LmNvbSIsImtleSI6ImtleTUiLCJleHAiOjE3MjAzMTUwMzYsIm5iZiI6MTcyMDMxNDczNiwicGF0aCI6Ii8zODg0OTAwLzM0NjMwNTg4MC01MmY5Nzc5Zi01MmUxLTRhOWUtYWNiNi1hZWNjMGYwYWQ3MmIucG5nP1gtQW16LUFsZ29yaXRobT1BV1M0LUhNQUMtU0hBMjU2JlgtQW16LUNyZWRlbnRpYWw9QUtJQVZDT0RZTFNBNTNQUUs0WkElMkYyMDI0MDcwNyUyRnVzLWVhc3QtMSUyRnMzJTJGYXdzNF9yZXF1ZXN0JlgtQW16LURhdGU9MjAyNDA3MDdUMDExMjE2WiZYLUFtei1FeHBpcmVzPTMwMCZYLUFtei1TaWduYXR1cmU9ODI3NjE1NzY2NGUyMDgzNGQ0NzhjOTYxZmIwZmI3NDk0ODEwMDJkNTE3YzE1NjFjYjEwM2E1ZGUwNmQzZDU2NiZYLUFtei1TaWduZWRIZWFkZXJzPWhvc3QmYWN0b3JfaWQ9MCZrZXlfaWQ9MCZyZXBvX2lkPTAifQ.iVLgDbUXXvljn4tI6DeUg62bDtZeS_uZ40ZQ74EWDYg)

![Set the clip information](https://private-user-images.githubusercontent.com/3884900/346305535-3a29ff80-a9e5-492c-a801-a2c6988910d0.png?jwt=eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJnaXRodWIuY29tIiwiYXVkIjoicmF3LmdpdGh1YnVzZXJjb250ZW50LmNvbSIsImtleSI6ImtleTUiLCJleHAiOjE3MjAzMTUwMzcsIm5iZiI6MTcyMDMxNDczNywicGF0aCI6Ii8zODg0OTAwLzM0NjMwNTUzNS0zYTI5ZmY4MC1hOWU1LTQ5MmMtYTgwMS1hMmM2OTg4OTEwZDAucG5nP1gtQW16LUFsZ29yaXRobT1BV1M0LUhNQUMtU0hBMjU2JlgtQW16LUNyZWRlbnRpYWw9QUtJQVZDT0RZTFNBNTNQUUs0WkElMkYyMDI0MDcwNyUyRnVzLWVhc3QtMSUyRnMzJTJGYXdzNF9yZXF1ZXN0JlgtQW16LURhdGU9MjAyNDA3MDdUMDExMjE3WiZYLUFtei1FeHBpcmVzPTMwMCZYLUFtei1TaWduYXR1cmU9OGU5NzA1NzcxYzlhOGM3NTgyZTc2ZDNkODRhZTA0OGU0NmRkYzA4YzE3M2M2YmEyYzFlZjc4NWE5ZTRlNDFmNyZYLUFtei1TaWduZWRIZWFkZXJzPWhvc3QmYWN0b3JfaWQ9MCZrZXlfaWQ9MCZyZXBvX2lkPTAifQ.HNS28IsjuA7gntRpxs45OhXqnVykyu0wXvWx3LJl_34)

![Job is added to queue and the clip appears on the home page once finished](https://private-user-images.githubusercontent.com/3884900/346306145-eb67dc4c-7f3a-427e-a144-f44e649aa684.png?jwt=eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJnaXRodWIuY29tIiwiYXVkIjoicmF3LmdpdGh1YnVzZXJjb250ZW50LmNvbSIsImtleSI6ImtleTUiLCJleHAiOjE3MjAzMTU2MzgsIm5iZiI6MTcyMDMxNTMzOCwicGF0aCI6Ii8zODg0OTAwLzM0NjMwNjE0NS1lYjY3ZGM0Yy03ZjNhLTQyN2UtYTE0NC1mNDRlNjQ5YWE2ODQucG5nP1gtQW16LUFsZ29yaXRobT1BV1M0LUhNQUMtU0hBMjU2JlgtQW16LUNyZWRlbnRpYWw9QUtJQVZDT0RZTFNBNTNQUUs0WkElMkYyMDI0MDcwNyUyRnVzLWVhc3QtMSUyRnMzJTJGYXdzNF9yZXF1ZXN0JlgtQW16LURhdGU9MjAyNDA3MDdUMDEyMjE4WiZYLUFtei1FeHBpcmVzPTMwMCZYLUFtei1TaWduYXR1cmU9NmY0ZTJhNTYzZWE1OGRmYWQxNWM5OWYwN2M2YWQ1M2FiYjIwNjVmMDUzMTkwODk3M2FlOWQ1NGQzNjAwMDUwZiZYLUFtei1TaWduZWRIZWFkZXJzPWhvc3QmYWN0b3JfaWQ9MCZrZXlfaWQ9MCZyZXBvX2lkPTAifQ.dSX6VEG8DoS99EFJvFzL0sfLWO-5kLy8feF1Q7XYsP0)

![Videos show in a grid](https://private-user-images.githubusercontent.com/3884900/346306047-111ab26e-781e-4e49-a642-7a846fdc1e49.png?jwt=eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJnaXRodWIuY29tIiwiYXVkIjoicmF3LmdpdGh1YnVzZXJjb250ZW50LmNvbSIsImtleSI6ImtleTUiLCJleHAiOjE3MjAzMTU0NDcsIm5iZiI6MTcyMDMxNTE0NywicGF0aCI6Ii8zODg0OTAwLzM0NjMwNjA0Ny0xMTFhYjI2ZS03ODFlLTRlNDktYTY0Mi03YTg0NmZkYzFlNDkucG5nP1gtQW16LUFsZ29yaXRobT1BV1M0LUhNQUMtU0hBMjU2JlgtQW16LUNyZWRlbnRpYWw9QUtJQVZDT0RZTFNBNTNQUUs0WkElMkYyMDI0MDcwNyUyRnVzLWVhc3QtMSUyRnMzJTJGYXdzNF9yZXF1ZXN0JlgtQW16LURhdGU9MjAyNDA3MDdUMDExOTA3WiZYLUFtei1FeHBpcmVzPTMwMCZYLUFtei1TaWduYXR1cmU9ZWViYjlkZGNkZWE4MjVkOTUyZDBjYTIzOTFmZDA1YmJhZjAxODYzY2I2ZDFiYmI0YWQyZWViNTRjMWViZDM2OSZYLUFtei1TaWduZWRIZWFkZXJzPWhvc3QmYWN0b3JfaWQ9MCZrZXlfaWQ9MCZyZXBvX2lkPTAifQ.CMHoH2E-KTi28YD0i0rZ4jkV_CO-a20sj3-j8wdhcf4)



Environment Variables
---------------------

All environment variables have a default and are optional, but setting these allows you to customize the behaviour of the app.

* `APP_NAME`: defines the name of the app on the webpage. Default is `Media Server Clipper`.
* `OUTPUT_PATH`: defines the directory in which the app will store the clips. Default is the `output` directory where the app is started from (`/app/output` in docker container).
* `PUBLIC_LINK_PREFIX`: defines a different base URL to the clips directory that will be used when clicking the `Share` button for a clip.
This is especially useful if you want to protect the MediaServer Clipper instance behind a htpassword but wish to have a public static file server to share the clips with people without giving your credentials (see docker-compose example below). Default is to link to the clip hosted by the Media Server Clipper (so the `share` and `link` buttons will do the same thing).
* `MAX_CLIP_DURATION`: defines the maximum allowed duration (in seconds) of the clips. Default is 600 (10 minutes).
* `MAX_QUEUE_SIZE`: number of jobs in queue after which the Clipper will reject new clip jobs. This does not count finished clips, only pending ones. Default is 4.

Running from source
-------------------

To run the app locally, install the rust toolchain (https://rustup.rs/) and run:

    cargo run --release

You need to have `ffmpeg` and the `DejaVu` fonts installed.

Docker
------

Docker is the recommended approach to run this application. To build the image, checkout in the root directory of this repository and run:

    docker build -t mediaserver-clipper .

Then run the image (this exposes it on port 9987):

    docker run -d --name mediaserver-clipper -p 9987:8000 -v ./media:/media -v ./clips:/app/output mediaserver-clipper

Docker-compose examples
-----------------------

### Standalone (`PUBLIC_LINK_PREFIX` is not set)

```yaml
    plex:
    #[...]
    volumes:
        - ./media:/media

    mediaserverclipper:
        image: mediaserver-clipper
        ports:
            - 9987:8000
        environment:
            - PUBLIC_LINK_PREFIX=https://yourdomain.tld:9988
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
        image: mediaserver-clipper
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

Note that the `download` and `link` buttons will still link to the mediaserverclipper url, only the `share` button use the URL from `PUBLIC_LINK_PREFIX` (the `static_clips` nginx instance here)
