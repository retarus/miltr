#[test]
fn client_v_server() {
    println!("Building and spawn the server");
    let server = escargot::CargoBuild::new()
        .current_release()
        .current_target()
        .manifest_path("./server/Cargo.toml")
        .example("print_server")
        .run()
        .expect("Failed building client");
    let mut server = server
        .command()
        .spawn()
        .expect("Failed running print example");

    println!("Build and start the client");
    let client = escargot::CargoBuild::new()
        .current_release()
        .current_target()
        .manifest_path("./client/Cargo.toml")
        .example("print_client")
        .run()
        .expect("Failed building client");

    let exit_status = client
        .command()
        .status()
        .expect("Failed running print example");

    // shutdown server
    server
        .kill()
        .expect("Failed killing server process in test");

    if !exit_status.success() {
        panic!("Client failed with status {}", exit_status);
    }
}
