FROM node:18-alpine as react_builder
WORKDIR /usr/src/app
COPY frontend/package*.json .
RUN npm install
COPY frontend .
RUN npm run build

FROM rust
COPY --from=react_builder /usr/src/app/dist /frontend/dist
WORKDIR /usr/src/app
COPY . .
RUN cargo build --release
WORKDIR /
EXPOSE 8080
CMD ["/usr/src/app/target/release/wol-server"]

