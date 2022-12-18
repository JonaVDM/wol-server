FROM node:18-alpine AS react_builder
WORKDIR /usr/src/app
COPY frontend/package*.json .
RUN npm install
COPY frontend .
RUN npm run build

FROM rust:bullseye AS rust_builder
WORKDIR /usr/src/app
COPY . .
RUN cargo build --release

FROM debian:bullseye
WORKDIR /
COPY --from=react_builder /usr/src/app/dist /frontend/dist
COPY --from=rust_builder /usr/src/app/target/release/wol-server /wol
EXPOSE 8080
CMD ["/wol"]
