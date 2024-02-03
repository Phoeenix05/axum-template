ARG image=rust:alpine3.19

FROM $image

COPY . .

RUN cargo build --release

CMD ./target/release/{{crate_name}}
