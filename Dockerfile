ARG image=rust:alpine3.19

FROM $image

RUN apk add --no-cache build-base

WORKDIR app

COPY . .

RUN cargo build --release
RUN cargo install

CMD {{crate_name}}
