FROM rustlang/rust:nightly

WORKDIR /usr/src/security-gate
COPY . .

RUN cargo install --path .

CMD ["security-gate"]
