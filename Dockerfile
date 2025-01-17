FROM rust:alpine AS backend
WORKDIR /backend
RUN apk add build-base
COPY packages/backend/Cargo.toml packages/backend/Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
COPY packages/backend/src ./src
RUN cargo build --release

FROM node:hydrogen-alpine AS frontend
WORKDIR /frontend
COPY packages/frontend/package.json ./
RUN npm install
COPY packages/frontend ./
RUN npm run build

FROM alpine
WORKDIR /comiya
COPY --from=backend /backend/target/release/backend ./backend
COPY --from=frontend /frontend/dist ./dist
CMD ["./backend"]
EXPOSE 8000