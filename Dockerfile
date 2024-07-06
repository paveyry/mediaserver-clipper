# Heavy build image
FROM rust:1.79-alpine3.20 as buildimage

RUN apk --no-cache add build-base
COPY ./ /srcdir
WORKDIR /srcdir

RUN cargo build --release

# Lightweight Deployment image
FROM alpine:3.20

RUN apk --no-cache add ffmpeg
RUN apk --no-cache add font-dejavu

COPY --from=buildimage /srcdir/target/release/mediaserver-clipper /app/mediaserver-clipper
COPY --from=buildimage /srcdir/public /app/public
COPY --from=buildimage /srcdir/templates /app/templates
COPY --from=buildimage /srcdir/Rocket.toml /app/Rocket.toml

WORKDIR /app
CMD ./mediaserver-clipper

