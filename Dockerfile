FROM scratch as builder
WORKDIR /src
COPY . .
# RUN touch /bin/app

FROM scratch
COPY --from=builder /bin/app /app
ENTRYPOINT ["/app"]