#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let node1_addr: SocketAddr = "127.0.0.1:8080".parse()?;
    let node2_addr: SocketAddr = "127.0.0.1:8081".parse()?;
    let cookie = "my-secret-cookie".to_string();

    let node1 =
        DistributedNode::start("node1@localhost".to_string(), node1_addr, cookie.clone()).await?;
    let _node2 =
        DistributedNode::start("node2@localhost".to_string(), node2_addr, cookie.clone()).await?;

    tokio::time::sleep(Duration::from_millis(100)).await;

    NodeServer::connect(node1.server.clone(), node2_addr).await??;
    tokio::time::sleep(Duration::from_millis(100)).await;

    println!("Node '{}' is alive: {}", await node1.get_name(), node1.is_alive());
    println!("Node '{}' has cookie: {}", await node1.get_name(), await node1.get_cookie());

    let nodes = node1.get_connected_nodes().await;
    println!("Nodes connected to node1: {:?}", nodes);

    if let Some(remote_node_name) = nodes.values().next() {
        println!("Spawning worker on remote node: {}", remote_node_name);
        let pid = node1
            .spawn_remote(remote_node_name, "remote-worker", Worker)
            .await?;
        println!("Successfully spawned remote worker with PID: {:?}", pid);
    }

    node1.disconnect_node("node2@localhost").await?;
    println!("Disconnected from node2@localhost.");

    tokio::time::sleep(Duration::from_secs(1)).await;

    Ok(())
}
