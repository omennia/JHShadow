use std::net::{Ipv4Addr, SocketAddrV4};
use std::path::PathBuf;

use shadow_shim_helper_rs::HostId;

use crate::core::support::configuration::QDiscMode;
use crate::cshadow as c;
use crate::network::packet::Packet;
use crate::network::PacketDevice;
use crate::utility::{self, HostTreePointer};

#[derive(Debug, Clone)]
pub struct PcapOptions {
    pub path: PathBuf,
    pub capture_size_bytes: u32,
}

/// Represents a network device that can send and receive packets. All accesses
/// to the internal C implementation should be done through this module.
pub struct NetworkInterface {
    c_ptr: HostTreePointer<c::NetworkInterface>,
    addr: Ipv4Addr,
}

impl NetworkInterface {
    /// Create a new network interface for `host_id` with the assigned `addr`.
    ///
    /// # Safety
    ///
    /// This function will trigger undefined behavior if `addr` is
    /// invalid. The reference count of `addr` will be increased by one using
    /// `address_ref()`, so the caller should call `address_unref()` on it to
    /// drop their reference when they no longer need it.
    pub unsafe fn new(
        host_id: HostId,
        addr: *mut c::Address,
        pcap_options: Option<PcapOptions>,
        qdisc: QDiscMode,
    ) -> NetworkInterface {
        let maybe_pcap_dir = pcap_options
            .as_ref()
            .map(|x| utility::pathbuf_to_nul_term_cstring(x.path.clone()));
        let pcap_dir_cptr = maybe_pcap_dir
            .as_ref()
            .map_or(std::ptr::null(), |p| p.as_ptr());

        let pcap_capture_size = pcap_options
            .as_ref()
            .map(|x| x.capture_size_bytes)
            .unwrap_or(0);

        let c_ptr =
            unsafe { c::networkinterface_new(addr, pcap_dir_cptr, pcap_capture_size, qdisc) };

        let ipv4_addr: Ipv4Addr = {
            let addr = unsafe { c::address_toNetworkIP(addr) };
            u32::from_be(addr).into()
        };

        NetworkInterface {
            c_ptr: HostTreePointer::new_for_host(host_id, c_ptr),
            addr: ipv4_addr,
        }
    }

    pub fn associate(
        &self,
        socket_ptr: *const c::CompatSocket,
        protocol_type: c::ProtocolType,
        port: u16,
        peer_addr: SocketAddrV4,
    ) {
        let port = port.to_be();
        let peer_ip = u32::from(*peer_addr.ip()).to_be();
        let peer_port = peer_addr.port().to_be();

        unsafe {
            c::networkinterface_associate(
                self.c_ptr.ptr(),
                socket_ptr,
                protocol_type,
                port,
                peer_ip,
                peer_port,
            )
        };
    }

    pub fn disassociate(&self, protocol_type: c::ProtocolType, port: u16, peer_addr: SocketAddrV4) {
        let port = port.to_be();
        let peer_ip = u32::from(*peer_addr.ip()).to_be();
        let peer_port = peer_addr.port().to_be();

        unsafe {
            c::networkinterface_disassociate(
                self.c_ptr.ptr(),
                protocol_type,
                port,
                peer_ip,
                peer_port,
            )
        };
    }

    pub fn is_associated(&self, protocol: c::ProtocolType, port: u16, peer: SocketAddrV4) -> bool {
        let port = port.to_be();
        let peer_ip = u32::from(*peer.ip()).to_be();
        let peer_port = peer.port().to_be();

        (unsafe {
            c::networkinterface_isAssociated(self.c_ptr.ptr(), protocol, port, peer_ip, peer_port)
        }) != 0
    }

    pub fn add_data_source(&self, socket_ptr: *const c::CompatSocket) {
        unsafe { c::networkinterface_wantsSend(self.c_ptr.ptr(), socket_ptr) };
    }

    /// Disassociate all bound sockets and remove sockets from the sending queue. This should be
    /// called as part of the host's cleanup procedure.
    pub fn remove_all_sockets(&self) {
        unsafe { c::networkinterface_removeAllSockets(self.c_ptr.ptr()) };
    }
}

impl Drop for NetworkInterface {
    fn drop(&mut self) {
        // don't check the active host since we're in the middle of dropping the host
        unsafe { c::networkinterface_free(self.c_ptr.ptr_unchecked()) };
    }
}

impl PacketDevice for NetworkInterface {
    fn get_address(&self) -> Ipv4Addr {
        self.addr
    }

    fn pop(&self) -> Option<Packet> {
        let packet_ptr = unsafe { c::networkinterface_pop(self.c_ptr.ptr()) };
        match packet_ptr.is_null() {
            true => None,
            false => Some(Packet::from_raw(packet_ptr)),
        }
    }

    fn push(&self, packet: Packet) {
        let packet_ptr = packet.into_inner();
        unsafe { c::networkinterface_push(self.c_ptr.ptr(), packet_ptr) };
        unsafe { c::packet_unref(packet_ptr) };
    }
}
