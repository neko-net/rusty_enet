/// Hooks into the ENet packet send/receive path to add custom header bytes
/// before the standard ENet protocol header.
///
/// # Send path (outgoing)
///
/// `write_outgoing` is called once per outgoing packet. The provided `header`
/// slice is `header_size()` bytes long and is zero-initialized. The processor
/// writes its header bytes there. They are prepended before the standard ENet
/// header and sent over the wire.
///
/// # Receive path (incoming)
///
/// `validate_incoming` is called for every incoming packet. The provided
/// `header` slice contains the first `header_size()` bytes of the received
/// data. The processor validates them.
///
/// Returns `Some(new_reserved)` if the packet is valid. `new_reserved` is
/// stored in the peer's reserved field and passed back on future calls so the
/// processor can detect replays or track state per peer.
///
/// Returns `None` to silently drop the packet.
pub trait PacketProcessor: Send {
    /// Size of the header this processor adds to every packet.
    fn header_size(&self) -> usize;

    /// Write the processor header bytes before an outgoing packet is sent.
    ///
    /// `peer_id` is the peer this packet is addressed to.
    /// `port` is the host socket's local port.
    fn write_outgoing(&mut self, header: &mut [u8], peer_id: u16, port: u16);

    /// Validate the processor header bytes from an incoming packet.
    ///
    /// `port` is the host socket's local port.
    /// `peer_reserved` is the current value of the sending peer's reserved
    /// field (0 initially, then the value returned by the previous call).
    fn validate_incoming(&mut self, header: &[u8], port: u16, peer_reserved: u16) -> Option<u16>;
}
