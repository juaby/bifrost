use bifrost::raft::*;

#[test]
fn startup(){
    let server = RaftServer::new(Options {
        storage: Storage::Default(),
        address: String::from("127.0.0.1:2000"),
    });
}