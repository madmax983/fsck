use crate::filesystem::FilesystemGraph;
use petgraph::dot::{Dot, Config};

pub struct MapExporter;

impl MapExporter {
    #[must_use]
    pub fn export_dot(fs: &FilesystemGraph) -> String {
        #[cfg(feature = "nova")]
        {
            let graph = fs.raw_graph();
            let dot = Dot::with_config(graph, &[Config::EdgeNoLabel]);
            format!("{dot:?}")
        }
        #[cfg(not(feature = "nova"))]
        {
            String::new()
        }
    }
}
