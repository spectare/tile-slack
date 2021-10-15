# syntax=docker/dockerfile:1

##
## Build
##
FROM golang:1.16-buster AS build

WORKDIR /

COPY main.go ./

RUN go build -o tile-slack ./main.go 

RUN pwd

RUN ls

##
## Deploy
##
FROM gcr.io/distroless/base-debian10

WORKDIR /

COPY --from=build /tile-slack /tile-slack

ENV PORT=5000

EXPOSE 5000

USER nonroot:nonroot

ENTRYPOINT ["/tile-slack"]

