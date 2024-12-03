FROM rust:1.83.0 as build

RUN rustup override set nightly

RUN mkdir /wonders

WORKDIR /wonders

# Copy your source tree
COPY ./ ./

# Build for Release
RUN cargo build --release

FROM rust:1.83.0

RUN mkdir /etc/app
RUN mkdir /etc/app/cache

# Copy the build artifact from the build stage
COPY --from=build /wonders/target/release/wonders /etc/app/
COPY --from=build /wonders/rocket.toml /etc/app/

ENV ROCKET_CONFIG=/etc/app/rocket.toml
ENV ROCKET_DB=postgres://root:password@localhost:5432/wonders
ENV ROCKET_PORT=8000
ENV ROCKET_CACHE=/etc/app/cache
ENV ROCKET_NASA_API_KEY=XXXX-XXXX-XXXX-XXXX
ENV ROCKET_API_URL="https://api.nasa.gov/planetary/apod"

CMD ["/etc/app/wonders", "-s"]
