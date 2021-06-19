# Build the Purple_Exporter Executable
FROM ekidd/rust-musl-builder:latest AS builder

ADD --chown=rust:rust . ./

RUN cargo build --release

# Place the builder in an optimized runtime image
FROM alpine:latest

# Add SSL Certs for the executable
RUN apk --no-cache add ca-certificates

# Create user and group for exporter
RUN addgroup -g 1000 purple_exporter
RUN adduser -D -s /bin/sh -u 1000 -G purple_exporter purple_exporter

COPY --from=builder  \ 
  /home/rust/src/target/x86_64-unknown-linux-musl/release/purple_exporter  \
  /usr/local/bin/

# Run exporter as user 
RUN chown purple_exporter:purple_exporter /usr/local/bin/purple_exporter
USER purple_exporter

CMD ["/usr/local/bin/purple_exporter"]