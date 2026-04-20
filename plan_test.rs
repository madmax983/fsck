pub fn graph(&self) -> &DiGraph<DirNode, EdgeType> {
    &self.graph
}
pub fn current(&self) -> NodeIndex {
    self.current
}
