fn main() {
    tonic_build::compile_protos("../proto/tikvpb.proto").unwrap();
}
