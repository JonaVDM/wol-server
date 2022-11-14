FROM rust as rust_builder
WORKDIR /usr/src/app
COPY . .
ENV RUSTFLAGS="-C target-feature=+crt-static"
RUN cargo build --release --bin wol-server

FROM node:18-alpine as react_builder
WORKDIR /usr/src/app
COPY frontend/package*.json .
RUN npm install
COPY frontend .
RUN npm run build

FROM scratch
COPY --from=rust_builder /usr/src/app/target/release/wol-server /wol
COPY --from=react_builder /usr/src/app/dist /frontend/dist
CMD ["/wol"]
