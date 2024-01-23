
ARG SRC_PATH=/workspace/map_app_navigation_service
ARG BUILD_REL_PATH=${SRC_PATH}/target/release


FROM rust:1.75.0 as base_devel_env
ARG SRC_PATH
COPY . ${SRC_PATH}
WORKDIR ${SRC_PATH}

FROM base_devel_env as build_debug
RUN cargo build

FROM build_debug as run_tests
RUN cargo test

FROM base_devel_env as build_release
RUN cargo build --release

FROM scratch as get_release_build
ARG BUILD_REL_PATH
COPY --from=build_release ${BUILD_REL_PATH}/map_app_navigation_service_bin .

FROM debian as runner
ENV MAPBOX_TOKEN ""
ARG BUILD_REL_PATH
RUN mkdir /workspace
WORKDIR /workspace
COPY --from=build_release ${BUILD_REL_PATH}/map_app_navigation_service_bin .
EXPOSE 3000

RUN chmod 755 /workspace/map_app_navigation_service_bin
COPY entrypoint.sh /workspace/entrypoint.sh
RUN chmod +x /workspace/entrypoint.sh

ENTRYPOINT ["/workspace/entrypoint.sh"]
