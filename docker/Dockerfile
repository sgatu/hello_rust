FROM rust:1.67 as build

RUN mkdir /opt/hello-rust
WORKDIR /opt/hello-rust
COPY . .
RUN ls -lha ./
RUN cargo build --release


FROM rust:1.67

RUN mkdir -p /opt/hello-rust

WORKDIR /opt/hello-rust/

COPY --from=build /opt/hello-rust/target/release/hello_rust /opt/hello-rust/hello_rust
RUN chmod +x /opt/hello-rust/hello_rust
COPY .env /opt/hello-rust/


CMD ["./hello_rust"]
