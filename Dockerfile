FROM rust:1.48

WORKDIR /usr/src/crustatious
COPY . .

RUN cargo install --path .

CMD ["crustatious"]