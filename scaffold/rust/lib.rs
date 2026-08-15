pub mod proto {
    pub mod oryon {
        pub mod authorization {
            #[path = "proto/oryon.authorization.v1.rs"]
            pub mod v1 {}
        }
        pub mod identity {
            #[path = "proto/oryon.identity.v1.rs"]
            pub mod v1 {}
        }
    }
}

#[path = "oryon.authorization.v1.rs"]
pub mod oryon_authorization_v1 {}

#[path = "oryon.identity.v1.rs"]
pub mod oryon_identity_v1 {}
