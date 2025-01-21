FROM rust:alpine AS backend
WORKDIR /backend
RUN apk add build-base
COPY packages/backend/Cargo.toml packages/backend/Cargo.lock ./
COPY packages/backend/migration/Cargo.toml ./migration/Cargo.toml
COPY packages/backend/entity/Cargo.toml ./entity/Cargo.toml
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN mkdir migration/src && touch migration/src/lib.rs
RUN mkdir entity/src && touch entity/src/lib.rs
RUN cargo build --release
COPY packages/backend/migration/src ./migration/src
COPY packages/backend/entity/src ./entity/src
COPY packages/backend/src ./src
# break the Cargo cache
RUN touch src/main.rs && touch migration/src/lib.rs && touch entity/src/lib.rs
RUN DATABASE_URL="sqlite://data/comiya.db?mode=rwc" cargo build --release

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