//
// Copyright (c) 2022 ZettaScale Technology
//
// This program and the accompanying materials are made available under the
// terms of the Eclipse Public License 2.0 which is available at
// http://www.eclipse.org/legal/epl-2.0, or the Apache License, Version 2.0
// which is available at https://www.apache.org/licenses/LICENSE-2.0.
//
// SPDX-License-Identifier: EPL-2.0 OR Apache-2.0
//
// Contributors:
//   ZettaScale Zenoh Team, <zenoh@zettascale.tech>
//
use uhlc::Timestamp;

/// # Err message
///
/// ```text
/// Flags:
/// - T: Timestamp      If T==1 then the timestamp if present
/// - I: Infrastructure If I==1 then the error is related to the infrastructure else to the user
/// - Z: Extension      If Z==1 then at least one extension is present
///
///   7 6 5 4 3 2 1 0
///  +-+-+-+-+-+-+-+-+
///  |Z|I|T|   ERR   |
///  +-+-+-+---------+
///  %   code:z16    %
///  +---------------+
///  ~ ts: <u8;z16>  ~  if T==1
///  +---------------+
///  ~  [err_exts]   ~  if Z==1
///  +---------------+
/// ```
pub mod flag {
    pub const T: u8 = 1 << 5; // 0x20 Timestamp         if T==0 then the timestamp if present
    pub const I: u8 = 1 << 6; // 0x40 Infrastructure    if I==1 then the error is related to the infrastructure else to the user
    pub const Z: u8 = 1 << 7; // 0x80 Extensions        if Z==1 then an extension will follow
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Err {
    pub code: u16,
    pub is_infrastructure: bool,
    pub timestamp: Option<Timestamp>,
    pub ext_sinfo: Option<ext::SourceInfoType>,
    pub ext_body: Option<ext::ErrBodyType>,
}

pub mod ext {
    use crate::{common::ZExtZBuf, core::Encoding, zextzbuf};
    use zenoh_buffers::ZBuf;

    /// # SourceInfo extension
    /// Used to carry additional information about the source of data
    pub type SourceInfo = crate::zenoh_new::put::ext::SourceInfo;
    pub type SourceInfoType = crate::zenoh_new::put::ext::SourceInfoType;

    /// # ErrBody extension
    /// Used to carry a body attached to the error
    pub type ErrBody = zextzbuf!(0x02, false);

    ///   7 6 5 4 3 2 1 0
    ///  +-+-+-+-+-+-+-+-+
    ///  ~   encoding    ~
    ///  +---------------+
    ///  ~ pl: [u8;z32]  ~  -- Payload
    ///  +---------------+
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct ErrBodyType {
        pub encoding: Encoding,
        pub payload: ZBuf,
    }

    impl ErrBodyType {
        #[cfg(feature = "test")]
        pub fn rand() -> Self {
            use rand::Rng;
            let mut rng = rand::thread_rng();

            let encoding = Encoding::rand();
            let payload = ZBuf::rand(rng.gen_range(1..=64));

            Self { encoding, payload }
        }
    }
}

impl Err {
    #[cfg(feature = "test")]
    pub fn rand() -> Self {
        use crate::core::ZenohId;
        use core::convert::TryFrom;
        use rand::Rng;
        let mut rng = rand::thread_rng();

        let code: u16 = rng.gen();
        let is_infrastructure = rng.gen_bool(0.5);
        let timestamp = rng.gen_bool(0.5).then_some({
            let time = uhlc::NTP64(rng.gen());
            let id = uhlc::ID::try_from(ZenohId::rand().as_slice()).unwrap();
            Timestamp::new(time, id)
        });
        let ext_sinfo = rng.gen_bool(0.5).then_some(ext::SourceInfoType::rand());
        let ext_body = rng.gen_bool(0.5).then_some(ext::ErrBodyType::rand());

        Self {
            code,
            is_infrastructure,
            timestamp,
            ext_sinfo,
            ext_body,
        }
    }
}
