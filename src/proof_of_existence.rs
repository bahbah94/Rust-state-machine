use core::fmt::Debug;
use std::collections::BTreeMap;
use crate::support::DispatchResult;

pub trait Config: crate::system::Config {

    type Content: Debug + Ord;
}


#[derive(Debug)]
pub struct Pallet<T: Config> {
    claims: BTreeMap<T::Content, T::AccountId>,
}

impl<T: Config> Pallet<T> {

    pub fn new() -> Self {
        Self {
            claims: BTreeMap::new()
        }
    }

    pub fn get_claim(&self, claim: &T::Content) -> Option<&T::AccountId> {
        self.claims.get(claim)
    }
    pub fn create_claim(&mut self, caller: T::AccountId, claim: T::Content) -> DispatchResult {
        match self.get_claim(&claim) {
            Some(_) => Err("Claim already exists"),
            None => {
                self.claims.insert(claim,caller);
                Ok(())
            }
        }
    }

    pub fn revoke_claim(&mut self, caller: T::AccountId, claim: T::Content) -> DispatchResult {
        let claim_owner = self.get_claim(&claim).ok_or("claim does not exists")?;

        if claim_owner != &caller {
            return Err("Caller is not the owner of the claim");
        }

        self.claims.remove(&claim);
        Ok(())
    }
}
pub enum Call<T: Config> {
    CreateClaim {claim: T::Content},
    RevokeClaim {claim: T::Content},
}
impl<T: Config> crate::support::Dispatch for Pallet<T> {
    type Call = Call<T>;
    type Caller = T::AccountId;

    fn dispatch(&mut self, caller: Self::Caller, call: Self::Call) -> DispatchResult {
        match call {
            Call::CreateClaim { claim } => {
                // Call create_claim and return its result
                self.create_claim(caller, claim)?;
            },
            Call::RevokeClaim { claim } => {
                // Call revoke_claim and return its result
                self.revoke_claim(caller, claim)?;
            }
        }
        Ok(())
    }
}
#[cfg(test)]
mod test {
    struct TestConfig;

    impl super::Config for TestConfig{
        type Content = &'static str;
    }
    impl crate::system::Config for TestConfig {
        type AccountId = &'static str;
        type BlockNumber = u32;
        type Nonce = u32;

    }

    #[test]
    fn basic_proof_of_existence() {
        let mut poe = super::Pallet::<TestConfig>::new();

        let _ = poe.create_claim("alice", "alice is good");

        assert_eq!(poe.get_claim(&"alice is good"), Some(&"alice"));

        let res = poe.revoke_claim("bob", "my_document");

        assert_eq!(res, Err("claim does not exists"));

        let res2 = poe.create_claim("alice", "my document");

        assert_eq!(res2,Err("Claim already exists"));
    }
}