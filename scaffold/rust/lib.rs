pub mod proto {
    pub mod oryon {
        pub mod authorization {
            pub mod v1 {
                include!("proto/oryon.authorization.v1.rs");
            }
        }
        pub mod identity {
            pub mod v1 {
                include!("proto/oryon.identity.v1.rs");
            }
        }
    }
}

pub mod oryon_authorization_v1 {
    include!("oryon.authorization.v1.rs");
}

pub mod oryon_identity_v1 {
    include!("oryon.identity.v1.rs");
}
