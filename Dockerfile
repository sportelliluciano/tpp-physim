FROM rust:1.80-slim AS builder

WORKDIR /tpp-physim
COPY . .

RUN cargo build --release

FROM debian:12-slim

RUN apt-get update && apt-get install -y libgcrypt20 libglib2.0-0 libpixman-1-0 libsdl2-2.0-0 \
    libslirp0 git wget flex bison gperf python3 python3-pip python3-venv cmake ninja-build ccache\
    libffi-dev libssl-dev dfu-util libusb-1.0-0

WORKDIR /tools
RUN git clone -b v5.2.1 --recursive https://github.com/espressif/esp-idf.git && \
    cd esp-idf && ./install.sh esp32 && \
    python3 /tools/esp-idf/tools/idf_tools.py install qemu-xtensa && \
    bash -c '. /tools/esp-idf/export.sh; ln -s $(which qemu-system-xtensa) /usr/bin/qemu-system-xtensa'

COPY --from=builder /tpp-physim/target/release/tpp-physim /usr/bin/tpp-physim

CMD ["sh", "-c", "/usr/bin/tpp-physim"]
