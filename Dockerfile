# Memory Safety Demo — container met Rust toolchain en GCC voorgeïnstalleerd.
#
# Build:
#   docker build -t memory-safety-demo .
#
# Run interactief:
#   docker run -it --rm memory-safety-demo
#
# Run alle stappen achter elkaar:
#   docker run --rm memory-safety-demo ./run-all.sh

FROM rust:1.85-slim-bookworm

# GCC + make voor de C-versie; libasan6 voor AddressSanitizer
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
        gcc \
        make \
        libasan8 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /demo
COPY . /demo

# Pre-fetch Rust dependencies zodat de eerste run snel is
RUN cd rust/fixed && cargo fetch && \
    cd ../broken && cargo fetch || true

CMD ["/bin/bash"]
