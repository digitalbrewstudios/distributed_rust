use ractor::{Actor, ActorId, ActorProcessingErr, ActorRef, RpcReplyPort, call};
use ractor_cluster::node::NodeServerSessionInformation;
use ractor_cluster::{NodeServer, NodeServerMessage, RactorClusterMessage, rpc::SpawnActorRemote};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::net::SocketAddr;

/// A unique identifier for a node in the cluster, typically 'name@host:port'.
pub type NodeId = u64;
/// An authentication token used for connecting nodes, equivalent to Erlang's magic cookie.
pub type Cookie = String;
/// A unique identifier for a process (actor) within the cluster.
pub type Pid = ActorId;

/// Represents a running worker actor that can be spawned remotely.
#[derive(Debug, Clone)]
pub struct Worker;

/// A message that can be sent to the `Worker` actor.
/// Must derive `RactorClusterMessage` to be sent over the network.
#[derive(Debug, Clone, RactorClusterMessage, Serialize, Deserialize)]
pub enum WorkerMessage {
    Ping,
}

impl Actor for Worker {
    type Msg = WorkerMessage;
    type State = ();
    type Arguments = ();

    async fn pre_start(
        &self,
        myself: ActorRef<Self::Msg>,
        _: (),
    ) -> Result<Self::State, ActorProcessingErr> {
        println!("Worker actor with PID {:?} started.", myself.get_id());
        Ok(())
    }

    async fn handle(
        &self,
        _myself: ActorRef<Self::Msg>,
        message: Self::Msg,
        _state: &mut Self::State,
    ) -> Result<(), ActorProcessingErr> {
        match message {
            WorkerMessage::Ping => {
                println!("Worker received Ping!");
            }
        }
        Ok(())
    }
}

/// Provides a high-level API for managing a distributed node and its cluster.
/// This struct acts as a facade over `ractor_cluster`'s functionality.
pub struct DistributedNode {
    /// An actor reference to the underlying `NodeServer` which manages the cluster.
    server: ActorRef<NodeServerMessage>,
}

impl DistributedNode {
    /// Starts a new distributed node, listening for incoming connections.
    pub async fn start(
        name: String,
        listen_addr: SocketAddr,
        cookie: Cookie,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let server_actor = NodeServer::new(
            listen_addr.port(),
            cookie,
            name,
            listen_addr.ip().to_string(),
            None,
            None,
        );
        let (server, _) = Actor::spawn(None, server_actor, ()).await?;
        Ok(Self { server })
    }

    /// Returns the name of the current node.
    pub async fn get_name(&self) -> String {
        call!(&self.server, || NodeServerMessage::GetName)
            .unwrap_or_else(|_| Ok("".to_string()))
            .unwrap_or_default()
    }

    /// Returns the magic cookie of the current node.
    pub async fn get_cookie(&self) -> Cookie {
call!(
    &self.server,
    |reply: RpcReplyPort<HashMap<NodeId, NodeServerSessionInformation>>| {
        NodeServerMessage::GetSessions(reply)
    }
)
            .unwrap_or_else(|_| Ok("".to_string()))
            .unwrap_or_default()
    }

    /// Returns a list of all visible nodes this node is connected to.
    pub async fn get_connected_nodes(&self) -> HashMap<NodeId, String> {
        use ractor::port::RpcReplyPort;
        use ractor_cluster::node::NodeServerSessionInformation;

        // Use the `call!` macro with the correct message variant, `GetSessions`.
        let result = call!(
    &self.server,
    |reply: RpcReplyPort<HashMap<NodeId, NodeServerSessionInformation>>| {
        NodeServerMessage::GetSessions(reply)
    }
)
 
        // The RPC call returns a map of session info. We extract the node name.
        match result {
            Ok(sessions) => sessions
                .into_iter()
                .map(|(id, info)| (id, info.node_name))
                .collect(),
            Err(_) => HashMap::new(),
        }
    }

    /// Forces the disconnection of a node.
    pub async fn disconnect_node(&self, node_name: &str) -> Result<(), String> {
        todo!("ractor_cluster doesn't provide an API yet")
    }

    /// Creates a linked process at a remote node.
    pub async fn spawn_remote<A: Actor<Arguments = ()>>(
        &self,
        node_name: &str,
        actor_name: &str,
        actor: A,
    ) {
        todo!("ractor_cluster doesn't provide an API yet")
    }
}
