# build stage
FROM rust:1.66.0-bullseye as build-stage
WORKDIR /app
COPY Cargo.toml ./
COPY Cargo.lock ./
COPY . .
# RUN sh install.sh
RUN rustup target add wasm32-unknown-unknown && \
    # cargo install wasm-pack && \
    cargo install trunk && \
    # cargo build --release && \
    trunk build

# production stage
FROM nginx:stable-alpine as production-stage
COPY --from=build-stage /app/dist /usr/share/nginx/html
EXPOSE 80
CMD ["nginx", "-g", "daemon off;"]
COPY nginx.conf /etc/nginx/
