FROM rust:1.67

WORKDIR /usr/src/swissgrid_chart_transformer

# copy sources
COPY Cargo.toml Cargo.lock ./
COPY src ./src

# build dependencies
RUN cargo build --release

#CMD ["myapp"]
