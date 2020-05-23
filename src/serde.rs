use crate::Dag;
use petgraph::algo::DfsSpace;
use petgraph::graph::IndexType;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

impl<N, E, Ix> Serialize for Dag<N, E, Ix>
where
    N: Serialize,
    E: Serialize,
    Ix: IndexType + Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.graph.serialize(serializer)
    }
}

impl<'de, N, E, Ix> Deserialize<'de> for Dag<N, E, Ix>
where
    Self: Sized,
    N: Deserialize<'de>,
    E: Deserialize<'de>,
    Ix: IndexType + Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let graph = Deserialize::deserialize(deserializer)?;
        let cycle_state = DfsSpace::new(&graph);
        let dag = Dag {
            graph: graph,
            cycle_state: cycle_state,
        };
        Ok(dag)
    }
}
