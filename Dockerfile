FROM rust as builder

WORKDIR /usr/src/app
COPY . .

RUN cargo build --release

CMD ["sh"]

FROM scratch
COPY --from=builder /usr/src/app/target/release/wol-server /wol
CMD ["/wol"]
