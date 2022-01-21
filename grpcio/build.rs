fn main() {
    protobuf_build::Builder::new()
        .includes(&["../proto"])
        .files(&["../proto/tikvpb.proto"])
        .generate();
}
