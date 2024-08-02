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

COPY --from=buildimage /srcdir/backend/target/release/mediaserver-clipper /app/backend/mediaserver-clipper
COPY --from=buildimage /srcdir/backend/static /app/backend/public
COPY --from=buildimage /srcdir/ui/dist /app/ui/dist
COPY --from=buildimage /srcdir/backend/Rocket.toml /app/backend/Rocket.toml

WORKDIR /app/backend
CMD ./mediaserver-clipper

