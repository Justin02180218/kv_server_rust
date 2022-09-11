fn main() {
    let mut conf = prost_build::Config::new();
    conf.bytes(&["."]);
    conf.type_attribute(".", "#[derive(PartialOrd)]");
    conf.out_dir("src/pb")
        .compile_protos(&["cmd.proto"], &["."])
        .unwrap();
}
