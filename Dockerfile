# Heavy build image
FROM rust:1.79-alpine3.20 as buildimage

RUN apk --no-cache add build-base
RUN rustup target add wasm32-unknown-unknown
RUN cargo install trunk
COPY ./ /srcdir

WORKDIR /srcdir/backend
RUN cargo build --release

# Lightweight Deployment image
FROM alpine:3.20

RUN apk --no-cache add ffmpeg
RUN apk --no-cache add font-dejavu

COPY --from=buildimage /srcdir/target/release/mediaserver-clipper /app/mediaserver-clipper
COPY --from=buildimage /srcdir/backend/static /app/static
COPY --from=buildimage /srcdir/backend/dist /app/dist
COPY --from=buildimage /srcdir/backend/Rocket.toml /app/Rocket.toml

WORKDIR /app
CMD ./mediaserver-clipper

