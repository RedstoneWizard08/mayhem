use std::sync::Mutex;

use oxide_auth::endpoint::{Authorizer, Issuer, Registrar};
use oxide_auth::frontends::simple::endpoint::{Generic, Vacant};
use oxide_auth::primitives::prelude::{AuthMap, ClientMap, RandomGenerator, TokenMap};
use oxide_auth::primitives::registrar::{Client, RegisteredUrl};

pub struct OAuthAppState {
    registrar: Mutex<ClientMap>,
    authorizer: Mutex<AuthMap<RandomGenerator>>,
    issuer: Mutex<TokenMap<RandomGenerator>>,
}

impl OAuthAppState {
    pub fn preconfigured() -> Self {
        OAuthAppState {
            registrar: Mutex::new(
                vec![Client::public(
                    "LocalClient",
                    RegisteredUrl::Semantic(
                        "https://dev-backend.kadaroja.com/api/v1/oauth/clientside/endpoint"
                            .parse()
                            .unwrap(),
                    ),
                    "default-scope".parse().unwrap(),
                )]
                .into_iter()
                .collect(),
            ),

            authorizer: Mutex::new(AuthMap::new(RandomGenerator::new(16))),
            issuer: Mutex::new(TokenMap::new(RandomGenerator::new(16))),
        }
    }

    pub fn endpoint(&self) -> Generic<impl Registrar + '_, impl Authorizer + '_, impl Issuer + '_> {
        Generic {
            registrar: self.registrar.lock().unwrap(),
            authorizer: self.authorizer.lock().unwrap(),
            issuer: self.issuer.lock().unwrap(),
            solicitor: Vacant,
            scopes: Vacant,
            response: Vacant,
        }
    }
}
