# test runner image
# TODO: this image should be one built under the Anoma GitHub organization
FROM ghcr.io/james-chf/devchain-container:v0.9.0
ENV RUST_BACKTRACE=full
COPY build/debug/ wasm/
RUN ./init_chain.sh

COPY build/tests/ tests/
