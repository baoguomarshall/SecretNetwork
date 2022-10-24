ARG SCRT_BASE_IMAGE_SECRETD=enigmampc/rocksdb:v6.24.2-1.1.5
ARG SCRT_BASE_IMAGE_ENCLAVE=enigmampc/enigma-sgx-base:2004-1.1.5
ARG SCRT_RELEASE_BASE_IMAGE=enigmampc/enigma-sgx-base:2004-1.1.5

# ***************** COMPILE ENCLAVE ************** #

FROM $SCRT_BASE_IMAGE_ENCLAVE AS compile-enclave

ENV PATH="/root/.cargo/bin:$PATH"

# Set working directory for the build
WORKDIR /go/src/github.com/enigmampc/SecretNetwork/

ARG BUILD_VERSION="v0.0.0"
ARG SGX_MODE=SW
ARG FEATURES
ARG FEATURES_U

ENV VERSION=${BUILD_VERSION}
ENV SGX_MODE=${SGX_MODE}
ENV FEATURES=${FEATURES}
ENV FEATURES_U=${FEATURES_U}
ENV MITIGATION_CVE_2020_0551=LOAD

COPY rust-toolchain rust-toolchain
RUN rustup component add rust-src
RUN cargo install xargo --version 0.3.25

# Add submodules
COPY third_party third_party

# Add source files
COPY go-cosmwasm go-cosmwasm/
COPY cosmwasm cosmwasm/

WORKDIR /go/src/github.com/enigmampc/SecretNetwork/go-cosmwasm

RUN . /opt/sgxsdk/environment && env \
    && MITIGATION_CVE_2020_0551=LOAD VERSION=${VERSION} FEATURES=${FEATURES} FEATURES_U=${FEATURES_U} SGX_MODE=${SGX_MODE} make build-rust

ENTRYPOINT ["/bin/bash"]

# ***************** COMPILE SECRETD ************** #
FROM $SCRT_BASE_IMAGE_SECRETD AS compile-secretd

ENV GOROOT=/usr/local/go
ENV GOPATH=/go/
ENV PATH=$PATH:/usr/local/go/bin:$GOPATH/bin

ADD https://go.dev/dl/go1.19.linux-amd64.tar.gz go.linux-amd64.tar.gz
RUN tar -C /usr/local -xzf go.linux-amd64.tar.gz
RUN go install github.com/jteeuwen/go-bindata/go-bindata@latest && go-bindata -version

# Set working directory for the build
WORKDIR /go/src/github.com/enigmampc/SecretNetwork

ARG BUILD_VERSION="v0.0.0"
ARG SGX_MODE=SW
ARG FEATURES
ARG FEATURES_U
ARG DB_BACKEND=goleveldb
ARG CGO_LDFLAGS

ENV VERSION=${BUILD_VERSION}
ENV SGX_MODE=${SGX_MODE}
ENV FEATURES=${FEATURES}
ENV FEATURES_U=${FEATURES_U}
ENV MITIGATION_CVE_2020_0551=LOAD

# Add source files
COPY go-cosmwasm go-cosmwasm
# This is due to some esoteric docker bug with the underlying filesystem, so until I figure out a better way, this should be a workaround
RUN true
COPY x x
RUN true
COPY types types
RUN true
COPY app app
COPY go.mod .
COPY go.sum .
COPY cmd cmd
COPY Makefile .
RUN true
COPY client client

RUN ln -s /usr/lib/x86_64-linux-gnu/liblz4.so /usr/local/lib/liblz4.so  && ln -s /usr/lib/x86_64-linux-gnu/libzstd.so /usr/local/lib/libzstd.so

RUN mkdir -p /go/src/github.com/enigmampc/SecretNetwork/go-cosmwasm/target/release/

COPY --from=compile-enclave /go/src/github.com/enigmampc/SecretNetwork/go-cosmwasm/target/release/libgo_cosmwasm.so /go/src/github.com/enigmampc/SecretNetwork/go-cosmwasm/target/release/libgo_cosmwasm.so
COPY --from=compile-enclave /go/src/github.com/enigmampc/SecretNetwork/go-cosmwasm/librust_cosmwasm_enclave.signed.so /go/src/github.com/enigmampc/SecretNetwork/go-cosmwasm/librust_cosmwasm_enclave.signed.so
# COPY --from=compile-enclave /go/src/github.com/enigmampc/SecretNetwork/go-cosmwasm/librust_cosmwasm_query_enclave.signed.so /go/src/github.com/enigmampc/SecretNetwork/go-cosmwasm/librust_cosmwasm_query_enclave.signed.so

RUN mkdir -p /go/src/github.com/enigmampc/SecretNetwork/ias_keys/develop
RUN mkdir -p /go/src/github.com/enigmampc/SecretNetwork/ias_keys/sw_dummy
RUN mkdir -p /go/src/github.com/enigmampc/SecretNetwork/ias_keys/production

RUN --mount=type=secret,id=SPID,dst=/run/secrets/spid.txt cat /run/secrets/spid.txt > /go/src/github.com/enigmampc/SecretNetwork/ias_keys/develop/spid.txt
RUN --mount=type=secret,id=SPID,dst=/run/secrets/spid.txt cat /run/secrets/spid.txt > /go/src/github.com/enigmampc/SecretNetwork/ias_keys/sw_dummy/spid.txt
RUN --mount=type=secret,id=SPID,dst=/run/secrets/spid.txt cat /run/secrets/spid.txt > /go/src/github.com/enigmampc/SecretNetwork/ias_keys/production/spid.txt

RUN --mount=type=secret,id=API_KEY,dst=/run/secrets/api_key.txt cat /run/secrets/api_key.txt > /go/src/github.com/enigmampc/SecretNetwork/ias_keys/develop/api_key.txt
RUN --mount=type=secret,id=API_KEY,dst=/run/secrets/api_key.txt cat /run/secrets/api_key.txt > /go/src/github.com/enigmampc/SecretNetwork/ias_keys/sw_dummy/api_key.txt
RUN --mount=type=secret,id=API_KEY,dst=/run/secrets/api_key.txt cat /run/secrets/api_key.txt >  /go/src/github.com/enigmampc/SecretNetwork/ias_keys/production/api_key.txt

RUN . /opt/sgxsdk/environment && env && CGO_LDFLAGS=${CGO_LDFLAGS} DB_BACKEND=${DB_BACKEND} MITIGATION_CVE_2020_0551=LOAD VERSION=${VERSION} FEATURES=${FEATURES} SGX_MODE=${SGX_MODE} make build_local_no_rust
RUN . /opt/sgxsdk/environment && env && MITIGATION_CVE_2020_0551=LOAD VERSION=${VERSION} FEATURES=${FEATURES} SGX_MODE=${SGX_MODE} make build_cli

# ******************* RELEASE IMAGE ******************** #
FROM $SCRT_RELEASE_BASE_IMAGE as release-image

# wasmi-sgx-test script requirements
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    #### Base utilities ####
    jq \
    openssl \
    curl \
    wget \
    libsnappy-dev \
    libgflags-dev \
    bash-completion && \
    rm -rf /var/lib/apt/lists/*

RUN echo "source /etc/profile.d/bash_completion.sh" >> ~/.bashrc

RUN curl -sL https://deb.nodesource.com/setup_16.x | bash - && \
    apt-get update && \
    apt-get install -y nodejs && \
    npm i -g local-cors-proxy

ARG SGX_MODE=SW
ENV SGX_MODE=${SGX_MODE}

ARG SECRET_NODE_TYPE=BOOTSTRAP
ENV SECRET_NODE_TYPE=${SECRET_NODE_TYPE}

ENV PKG_CONFIG_PATH=""
ENV SCRT_ENCLAVE_DIR=/usr/lib/

# workaround because paths seem kind of messed up
RUN ln -s /opt/sgxsdk/lib64/libsgx_urts_sim.so /usr/lib/x86_64-linux-gnu/libsgx_urts_sim.so
RUN ln -s /opt/sgxsdk/lib64/libsgx_uae_service_sim.so /usr/lib/x86_64-linux-gnu/libsgx_uae_service_sim.so

# Install ca-certificates
WORKDIR /root

# Copy over binaries from the build-env
COPY --from=compile-secretd /go/src/github.com/enigmampc/SecretNetwork/go-cosmwasm/target/release/libgo_cosmwasm.so /usr/lib/
COPY --from=compile-secretd /go/src/github.com/enigmampc/SecretNetwork/go-cosmwasm/librust_cosmwasm_enclave.signed.so /usr/lib/
#COPY --from=build-env-rust-go /go/src/github.com/enigmampc/SecretNetwork/go-cosmwasm/librust_cosmwasm_query_enclave.signed.so /usr/lib/
COPY --from=compile-secretd /go/src/github.com/enigmampc/SecretNetwork/secretd /usr/bin/secretd

COPY deployment/docker/bootstrap/bootstrap_init.sh .
COPY deployment/docker/node/node_init.sh .
COPY deployment/docker/startup.sh .
COPY deployment/docker/node_key.json .

RUN chmod +x /usr/bin/secretd
RUN chmod +x bootstrap_init.sh
RUN chmod +x startup.sh
RUN chmod +x node_init.sh

RUN secretd completion > /root/secretd_completion

RUN echo 'source /root/secretd_completion' >> ~/.bashrc

RUN mkdir -p /root/.secretd/.compute/
RUN mkdir -p /opt/secret/.sgx_secrets/
RUN mkdir -p /root/.secretd/.node/
RUN mkdir -p /root/config/


####### Node parameters
ARG MONIKER=default
ARG CHAINID=secretdev-1
ARG GENESISPATH=https://raw.githubusercontent.com/enigmampc/SecretNetwork/master/secret-testnet-genesis.json
ARG PERSISTENT_PEERS=201cff36d13c6352acfc4a373b60e83211cd3102@bootstrap.southuk.azure.com:26656

ENV GENESISPATH="${GENESISPATH}"
ENV CHAINID="${CHAINID}"
ENV MONIKER="${MONIKER}"
ENV PERSISTENT_PEERS="${PERSISTENT_PEERS}"

#ENV LD_LIBRARY_PATH=/opt/sgxsdk/libsgx-enclave-common/:/opt/sgxsdk/lib64/

# Run secretd by default, omit entrypoint to ease using container with secretcli
ENTRYPOINT ["/bin/bash", "startup.sh"]

# ***************** MAINNET UPGRADE ************** #

FROM release-image as mainnet-release

RUN STORAGE_PATH=`echo ${VERSION} | sed -e 's/\.//g' | head -c 2` \
    && wget -O /usr/lib/librust_cosmwasm_enclave.signed.so https://engfilestorage.blob.core.windows.net/v$STORAGE_PATH/librust_cosmwasm_enclave.signed.so

COPY deployment/docker/node/mainnet_node.sh .
RUN chmod +x mainnet_node.sh

ENTRYPOINT ["/bin/bash", "mainnet_node.sh"]

# ***************** BUILD DEBIAN ************** #
# ARG SCRT_COMPILED_BINARIES_SOURCE=compile-secretd
FROM $SCRT_RELEASE_BASE_IMAGE as build-deb

# wasmi-sgx-test script requirements
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    #### Base utilities ####
    git \
    make \
    wget && \
    rm -rf /var/lib/apt/lists/*

ARG BUILD_VERSION="v0.5.0-rc1"
ARG SGX_MODE=SW
ENV VERSION=${BUILD_VERSION}
ENV SGX_MODE=${SGX_MODE}

# Install ca-certificates
WORKDIR /root

RUN mkdir -p ./go-cosmwasm/api/

COPY Makefile .

# Copy over binaries from the build-env
COPY --from=compile-secretd /go/src/github.com/enigmampc/SecretNetwork/go-cosmwasm/target/release/libgo_cosmwasm.so ./go-cosmwasm/api/
COPY --from=compile-secretd /go/src/github.com/enigmampc/SecretNetwork/go-cosmwasm/librust_cosmwasm_enclave.signed.so ./go-cosmwasm/
#COPY --from=build-env-rust-go /go/src/github.com/enigmampc/SecretNetwork/go-cosmwasm/librust_cosmwasm_query_enclave.signed.so ./go-cosmwasm/
COPY --from=compile-secretd /go/src/github.com/enigmampc/SecretNetwork/secretd secretd
COPY --from=compile-secretd /go/src/github.com/enigmampc/SecretNetwork/secretcli secretcli

COPY ./deployment/deb ./deployment/deb
COPY ./deployment/docker/builder/build_deb.sh .

RUN chmod +x build_deb.sh

# Run secretd by default, omit entrypoint to ease using container with secretcli
CMD ["/bin/bash", "build_deb.sh"]