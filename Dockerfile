FROM rust as rust_builder
WORKDIR /usr/src/app
COPY . .
ENV RUSTFLAGS="-C target-feature=+crt-static"
RUN cargo build --release --bin wol-server --target x86_64-unknown-linux-gnu

FROM node:18-alpine as react_builder
WORKDIR /usr/src/app
COPY frontend/package*.json .
RUN npm install
COPY frontend .
RUN npm run build

FROM alpine
COPY --from=rust_builder /usr/src/app/target/x86_64-unknown-linux-gnu/release/wol-server /wol
COPY --from=react_builder /usr/src/app/dist /frontend/dist
EXPOSE 8080
CMD ["/wol"]
