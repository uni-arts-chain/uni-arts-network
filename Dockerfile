# ===== BUILD ======

FROM phusion/baseimage:0.11 as builder

ENV DEBIAN_FRONTEND=noninteractive
ENV RUST_TOOLCHAIN=nightly-2020-09-30
ENV PATH="$PATH:/root/.cargo/bin"
ARG PROFILE=release

RUN apt-get update && \
	apt-get dist-upgrade -y -o Dpkg::Options::="--force-confold" && \
	apt-get install -y cmake pkg-config libssl-dev git clang llvm build-essential libclang-dev libc6-dev


RUN curl https://sh.rustup.rs -sSf | sh -s -- -y 
# && \
RUN	rustup uninstall stable && \
    rustup install 1.46.0 && \
    rustup default 1.46.0-x86_64-unknown-linux-gnu && \
	rustup toolchain install $RUST_TOOLCHAIN && \
	rustup target add wasm32-unknown-unknown --toolchain $RUST_TOOLCHAIN && \
	rustup default $RUST_TOOLCHAIN

# Get project and run it
#RUN git clone https://github.com/uni-arts-chain/uni-arts-network.git /uniarts_chain

RUN mkdir uniarts_chain
WORKDIR /uniarts_chain
COPY . .

RUN	cargo +nightly-2020-09-30 build "--$PROFILE"

RUN cd target/release && ls -la

# ===== RUN ======

FROM phusion/baseimage:0.11
ARG PROFILE=release

COPY --from=builder /uniarts_chain/target/$PROFILE/uart /usr/local/bin

EXPOSE 30333 9933 9944
VOLUME ["/chain-data"]

# Copy and run start script
COPY ["./run.sh", "./run.sh"]
RUN chmod +x ./run.sh
CMD ["bash", "-c", "./run.sh"]
