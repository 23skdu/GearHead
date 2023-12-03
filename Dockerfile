FROM rustlang:latest as builder
ENV USER root

WORKDIR (fill in builder shit later)

FROM alpine:latest
COPY --from=builder gearhead/target/release/gearhead /gearhead
CMD ["/gearhead"]

