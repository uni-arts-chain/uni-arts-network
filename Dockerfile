# ===== BUILD ======

FROM phusion/baseimage:0.11 as builder

ENV WASM_TOOLCHAIN=nightly-2020-08-23

ARG PROFILE=release

RUN apt-get update && \
	apt-get dist-upgrade -y -o Dpkg::Options::="--force-confold" && \
	apt-get install -y cmake pkg-config libssl-dev git clang-3.9 llvm

# Get project and run it
#RUN git clone https://github.com/uni-arts-chain/uni-arts-network.git /uniarts_chain
RUN mkdir uniarts_chain
WORKDIR /uniarts_chain
COPY . .

RUN curl https://sh.rustup.rs -sSf | sh -s -- -y && \
	export PATH="$PATH:$HOME/.cargo/bin" && \
	rustup toolchain uninstall $(rustup toolchain list) && \
	rustup default 1.47.0 && \
	rustup toolchain install $WASM_TOOLCHAIN && \
	rustup target add wasm32-unknown-unknown --toolchain $WASM_TOOLCHAIN && \
    rustup target list --installed && \
    rustup show && \
	cargo +nightly-2020-08-23 build "--$PROFILE"
	# && \
	# cargo test

RUN cd target/release && ls -la

# ===== RUN ======

FROM phusion/baseimage:0.11
ARG PROFILE=release

COPY --from=builder /uniarts_chain/target/$PROFILE/uart /usr/local/bin

EXPOSE 9944
VOLUME ["/chain-data"]

# Copy and run start script
COPY ["./run.sh", "./run.sh"]
RUN chmod +x ./run.sh
CMD ["bash", "-c", "./run.sh"]
