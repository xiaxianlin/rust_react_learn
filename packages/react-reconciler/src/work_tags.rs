#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WorkTag {
    FunctionComponent = 0,
    HostRoot = 3,
    HostComponent = 5,
    HostText = 6,
}
