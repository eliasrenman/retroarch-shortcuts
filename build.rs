fn main() {
    let proto_file = "./assets/game.proto";
    protobuf_codegen_pure::Codegen::new()
        .out_dir("src/generated")
        .inputs(&[proto_file])
        .include("./assets")
        .run()
        .expect("Codegen failed.");
}
