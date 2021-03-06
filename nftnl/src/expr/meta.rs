use super::{Expression, Rule};
use nftnl_sys::{self as sys, libc};
use std::os::raw::c_char;

/// A meta expression refers to meta data associated with a packet.
#[non_exhaustive]
pub enum Meta {
    /// Packet ethertype protocol (skb->protocol), invalid in OUTPUT.
    Protocol,
    /// Packet mark.
    Mark { set: bool },
    /// Packet input interface index (dev->ifindex).
    Iif,
    /// Packet output interface index (dev->ifindex).
    Oif,
    /// Packet input interface name (dev->name)
    IifName,
    /// Packet output interface name (dev->name).
    OifName,
    /// Packet input interface type (dev->type).
    IifType,
    /// Packet output interface type (dev->type).
    OifType,
    /// Originating socket UID (fsuid).
    SkUid,
    /// Originating socket GID (fsgid).
    SkGid,
    /// Netfilter protocol (Transport layer protocol).
    NfProto,
    /// Layer 4 protocol number.
    L4Proto,
    /// Socket control group (skb->sk->sk_classid).
    Cgroup,
    /// A 32bit pseudo-random number
    PRandom,
}

impl Meta {
    /// Returns the corresponding `NFT_*` constant for this meta expression.
    pub fn to_raw_key(&self) -> u32 {
        use Meta::*;
        match *self {
            Protocol => libc::NFT_META_PROTOCOL as u32,
            Mark { .. } => libc::NFT_META_MARK as u32,
            Iif => libc::NFT_META_IIF as u32,
            Oif => libc::NFT_META_OIF as u32,
            IifName => libc::NFT_META_IIFNAME as u32,
            OifName => libc::NFT_META_OIFNAME as u32,
            IifType => libc::NFT_META_IIFTYPE as u32,
            OifType => libc::NFT_META_OIFTYPE as u32,
            SkUid => libc::NFT_META_SKUID as u32,
            SkGid => libc::NFT_META_SKGID as u32,
            NfProto => libc::NFT_META_NFPROTO as u32,
            L4Proto => libc::NFT_META_L4PROTO as u32,
            Cgroup => libc::NFT_META_CGROUP as u32,
            PRandom => libc::NFT_META_PRANDOM as u32,
        }
    }
}

impl Expression for Meta {
    fn to_expr(&self, _rule: &Rule) -> *mut sys::nftnl_expr {
        unsafe {
            let expr = try_alloc!(sys::nftnl_expr_alloc(
                b"meta\0" as *const _ as *const c_char
            ));

            if let Meta::Mark { set: true } = self {
                sys::nftnl_expr_set_u32(
                    expr,
                    sys::NFTNL_EXPR_META_SREG as u16,
                    libc::NFT_REG_1 as u32,
                );
            } else {
                sys::nftnl_expr_set_u32(
                    expr,
                    sys::NFTNL_EXPR_META_DREG as u16,
                    libc::NFT_REG_1 as u32,
                );
            }
            sys::nftnl_expr_set_u32(expr, sys::NFTNL_EXPR_META_KEY as u16, self.to_raw_key());
            expr
        }
    }
}

#[macro_export]
macro_rules! nft_expr_meta {
    (proto) => {
        $crate::expr::Meta::Protocol
    };
    (mark set) => {
        $crate::expr::Meta::Mark { set: true }
    };
    (mark) => {
        $crate::expr::Meta::Mark { set: false }
    };
    (iif) => {
        $crate::expr::Meta::Iif
    };
    (oif) => {
        $crate::expr::Meta::Oif
    };
    (iifname) => {
        $crate::expr::Meta::IifName
    };
    (oifname) => {
        $crate::expr::Meta::OifName
    };
    (iiftype) => {
        $crate::expr::Meta::IifType
    };
    (oiftype) => {
        $crate::expr::Meta::OifType
    };
    (skuid) => {
        $crate::expr::Meta::SkUid
    };
    (skgid) => {
        $crate::expr::Meta::SkGid
    };
    (nfproto) => {
        $crate::expr::Meta::NfProto
    };
    (l4proto) => {
        $crate::expr::Meta::L4Proto
    };
    (cgroup) => {
        $crate::expr::Meta::Cgroup
    };
    (random) => {
        $crate::expr::Meta::PRandom
    };
}
