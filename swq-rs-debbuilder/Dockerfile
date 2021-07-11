FROM rust:latest

RUN git clone https://github.com/thecatster/swq-rs
WORKDIR "/swq-rs"
RUN git pull --force
RUN cargo install cargo-deb

RUN apt update -y && apt upgrade -y
RUN apt install libc6-dev-i386 -y
RUN apt install libxcb-randr0-dev libxcb-xtest0-dev libxcb-xinerama0-dev libxcb-shape0-dev libxcb-xkb-dev -y
RUN apt install libx11-dev libdbus-1-dev -y
RUN apt install libxmu-dev libxmu-headers freeglut3-dev libxext-dev libxi-dev -y
RUN git clone --depth=1 https://github.com/raspberrypi/tools /raspberrypi-tools
ENV PATH=/raspberrypi-tools/arm-bcm2708/gcc-linaro-arm-linux-gnueabihf-raspbian-x64/bin/:$PATH
ENV CARGO_TARGET_ARM_UNKNOWN_LINUX_GNUEABIHF_LINKER=arm-linux-gnueabihf-gcc
RUN mkdir /root/.cargo
RUN echo "[target.arm-unknown-linux-gnueabihf]" >> /root/.cargo/config
RUN echo "strip = { path = \"arm-linux-gnueabihf-strip\" }" >> /root/.cargo/config
RUN echo "objcopy = { path = \"arm-linux-gnueabihf-objcopy\" }" >> /root/.cargo/config

COPY ./entrypoint.sh /entrypoint.sh
RUN chmod +x /entrypoint.sh
ENTRYPOINT ["/entrypoint.sh"]
