//! Point to point communication traits
pub use super::{Source, Destination, Send, BufferedSend, SynchronousSend, ReadySend, Probe,
                MatchedReceive, MatchedReceiveInto, MatchedReceiveVec, MatchedProbe,
                Receive, ReceiveInto, ReceiveVec,
                SendReceive, SendReceiveInto, SendReceiveReplaceInto,
                RawRequest, Wait, Test,
                ImmediateSend, ImmediateBufferedSend, ImmediateSynchronousSend, ImmediateReadySend,
                ImmediateReceiveInto, ImmediateProbe, ImmediateMatchedProbe,
                ImmediateMatchedReceiveInto};
