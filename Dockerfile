FROM public.ecr.aws/ubuntu/ubuntu:24.04 as BUILDER

ARG DEBIAN_FRONTEND=noninteractive

LABEL maintainer="apsoyka@protonmail.com"

RUN --mount=type=cache,target=/var/cache/apt,sharing=locked \
    --mount=type=cache,target=/var/lib/apt,sharing=locked \
    apt update && \
    apt install --no-install-recommends --yes \
    build-essential curl tree ca-certificates

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

WORKDIR /build

COPY . .

RUN /root/.cargo/bin/cargo build --release && tree -a .

FROM public.ecr.aws/ubuntu/ubuntu:24.04

RUN printf 'CREATE_MAIL_SPOOL=no' >> /etc/default/useradd && \
    mkdir -p /home/ciphergen /home/scripts && \
    groupadd ciphergen && \
    useradd ciphergen -g ciphergen -d /home/ciphergen && \
    chown ciphergen:ciphergen /home/ciphergen

COPY --from=BUILDER --chown=ciphergen:ciphergen /build/target/release/ciphergen /usr/bin/ciphergen

USER ciphergen:ciphergen
WORKDIR /home/ciphergen
ENTRYPOINT [ "/usr/bin/ciphergen" ]
CMD [ "--help" ]
