// Code generated by sails-client-gen. DO NOT EDIT.
#[allow(unused_imports)]
use sails_rs::collections::BTreeMap;
#[allow(unused_imports)]
use sails_rs::{
    calls::{Activation, Call, Query, Remoting, RemotingAction},
    prelude::*,
    String,
};
pub struct AppFactory<R> {
    #[allow(dead_code)]
    remoting: R,
}
impl<R> AppFactory<R> {
    #[allow(unused)]
    pub fn new(remoting: R) -> Self {
        Self { remoting }
    }
}
impl<R: Remoting + Clone> traits::AppFactory for AppFactory<R> {
    type Args = R::Args;
    fn new(&self) -> impl Activation<Args = R::Args> {
        RemotingAction::<_, app_factory::io::New>::new(self.remoting.clone(), ())
    }
}
pub mod app_factory {
    use super::*;
    pub mod io {
        use super::*;
        use sails_rs::calls::ActionIo;
        pub struct New(());
        impl New {
            #[allow(dead_code)]
            pub fn encode_call() -> Vec<u8> {
                <New as ActionIo>::encode_call(&())
            }
        }
        impl ActionIo for New {
            const ROUTE: &'static [u8] = &[12, 78, 101, 119];
            type Params = ();
            type Reply = ();
        }
    }
}
pub struct QuerySvc<R> {
    remoting: R,
}
impl<R> QuerySvc<R> {
    pub fn new(remoting: R) -> Self {
        Self { remoting }
    }
}
impl<R: Remoting + Clone> traits::QuerySvc for QuerySvc<R> {
    type Args = R::Args;
    fn all_callers(&self) -> impl Query<Output = Vec<ActorId>, Args = R::Args> {
        RemotingAction::<_, query_svc::io::AllCallers>::new(self.remoting.clone(), ())
    }
    fn state_num_value(&self) -> impl Query<Output = u64, Args = R::Args> {
        RemotingAction::<_, query_svc::io::StateNumValue>::new(self.remoting.clone(), ())
    }
    fn state_string_value(&self) -> impl Query<Output = String, Args = R::Args> {
        RemotingAction::<_, query_svc::io::StateStringValue>::new(self.remoting.clone(), ())
    }
    fn vec_string_value(&self) -> impl Query<Output = Vec<String>, Args = R::Args> {
        RemotingAction::<_, query_svc::io::VecStringValue>::new(self.remoting.clone(), ())
    }
}
pub mod query_svc {
    use super::*;
    pub mod io {
        use super::*;
        use sails_rs::calls::ActionIo;
        pub struct AllCallers(());
        impl AllCallers {
            #[allow(dead_code)]
            pub fn encode_call() -> Vec<u8> {
                <AllCallers as ActionIo>::encode_call(&())
            }
        }
        impl ActionIo for AllCallers {
            const ROUTE: &'static [u8] = &[
                32, 81, 117, 101, 114, 121, 83, 118, 99, 40, 65, 108, 108, 67, 97, 108, 108, 101,
                114, 115,
            ];
            type Params = ();
            type Reply = Vec<ActorId>;
        }
        pub struct StateNumValue(());
        impl StateNumValue {
            #[allow(dead_code)]
            pub fn encode_call() -> Vec<u8> {
                <StateNumValue as ActionIo>::encode_call(&())
            }
        }
        impl ActionIo for StateNumValue {
            const ROUTE: &'static [u8] = &[
                32, 81, 117, 101, 114, 121, 83, 118, 99, 52, 83, 116, 97, 116, 101, 78, 117, 109,
                86, 97, 108, 117, 101,
            ];
            type Params = ();
            type Reply = u64;
        }
        pub struct StateStringValue(());
        impl StateStringValue {
            #[allow(dead_code)]
            pub fn encode_call() -> Vec<u8> {
                <StateStringValue as ActionIo>::encode_call(&())
            }
        }
        impl ActionIo for StateStringValue {
            const ROUTE: &'static [u8] = &[
                32, 81, 117, 101, 114, 121, 83, 118, 99, 64, 83, 116, 97, 116, 101, 83, 116, 114,
                105, 110, 103, 86, 97, 108, 117, 101,
            ];
            type Params = ();
            type Reply = String;
        }
        pub struct VecStringValue(());
        impl VecStringValue {
            #[allow(dead_code)]
            pub fn encode_call() -> Vec<u8> {
                <VecStringValue as ActionIo>::encode_call(&())
            }
        }
        impl ActionIo for VecStringValue {
            const ROUTE: &'static [u8] = &[
                32, 81, 117, 101, 114, 121, 83, 118, 99, 56, 86, 101, 99, 83, 116, 114, 105, 110,
                103, 86, 97, 108, 117, 101,
            ];
            type Params = ();
            type Reply = Vec<String>;
        }
    }
}
pub struct Receiver<R> {
    remoting: R,
}
impl<R> Receiver<R> {
    pub fn new(remoting: R) -> Self {
        Self { remoting }
    }
}
impl<R: Remoting + Clone> traits::Receiver for Receiver<R> {
    type Args = R::Args;
    fn add_string_to_vec(
        &mut self,
        value: String,
    ) -> impl Call<Output = ReceiverEvents, Args = R::Args> {
        RemotingAction::<_, receiver::io::AddStringToVec>::new(self.remoting.clone(), value)
    }
    fn set_num_value(
        &mut self,
        new_value: u64,
    ) -> impl Call<Output = ReceiverEvents, Args = R::Args> {
        RemotingAction::<_, receiver::io::SetNumValue>::new(self.remoting.clone(), new_value)
    }
    fn set_string_value(
        &mut self,
        new_value: String,
    ) -> impl Call<Output = ReceiverEvents, Args = R::Args> {
        RemotingAction::<_, receiver::io::SetStringValue>::new(self.remoting.clone(), new_value)
    }
}
pub mod receiver {
    use super::*;
    pub mod io {
        use super::*;
        use sails_rs::calls::ActionIo;
        pub struct AddStringToVec(());
        impl AddStringToVec {
            #[allow(dead_code)]
            pub fn encode_call(value: String) -> Vec<u8> {
                <AddStringToVec as ActionIo>::encode_call(&value)
            }
        }
        impl ActionIo for AddStringToVec {
            const ROUTE: &'static [u8] = &[
                32, 82, 101, 99, 101, 105, 118, 101, 114, 56, 65, 100, 100, 83, 116, 114, 105, 110,
                103, 84, 111, 86, 101, 99,
            ];
            type Params = String;
            type Reply = super::ReceiverEvents;
        }
        pub struct SetNumValue(());
        impl SetNumValue {
            #[allow(dead_code)]
            pub fn encode_call(new_value: u64) -> Vec<u8> {
                <SetNumValue as ActionIo>::encode_call(&new_value)
            }
        }
        impl ActionIo for SetNumValue {
            const ROUTE: &'static [u8] = &[
                32, 82, 101, 99, 101, 105, 118, 101, 114, 44, 83, 101, 116, 78, 117, 109, 86, 97,
                108, 117, 101,
            ];
            type Params = u64;
            type Reply = super::ReceiverEvents;
        }
        pub struct SetStringValue(());
        impl SetStringValue {
            #[allow(dead_code)]
            pub fn encode_call(new_value: String) -> Vec<u8> {
                <SetStringValue as ActionIo>::encode_call(&new_value)
            }
        }
        impl ActionIo for SetStringValue {
            const ROUTE: &'static [u8] = &[
                32, 82, 101, 99, 101, 105, 118, 101, 114, 56, 83, 101, 116, 83, 116, 114, 105, 110,
                103, 86, 97, 108, 117, 101,
            ];
            type Params = String;
            type Reply = super::ReceiverEvents;
        }
    }
}
#[derive(PartialEq, Debug, Encode, Decode, TypeInfo)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub enum ReceiverEvents {
    StringValueChanged { new: String, old: String },
    NumValueChange { new: u64, old: u64 },
    StringAddedToVec { added: String },
}
pub mod traits {
    use super::*;
    #[allow(dead_code)]
    pub trait AppFactory {
        type Args;
        #[allow(clippy::new_ret_no_self)]
        #[allow(clippy::wrong_self_convention)]
        fn new(&self) -> impl Activation<Args = Self::Args>;
    }
    #[allow(clippy::type_complexity)]
    pub trait QuerySvc {
        type Args;
        fn all_callers(&self) -> impl Query<Output = Vec<ActorId>, Args = Self::Args>;
        fn state_num_value(&self) -> impl Query<Output = u64, Args = Self::Args>;
        fn state_string_value(&self) -> impl Query<Output = String, Args = Self::Args>;
        fn vec_string_value(&self) -> impl Query<Output = Vec<String>, Args = Self::Args>;
    }
    #[allow(clippy::type_complexity)]
    pub trait Receiver {
        type Args;
        fn add_string_to_vec(
            &mut self,
            value: String,
        ) -> impl Call<Output = ReceiverEvents, Args = Self::Args>;
        fn set_num_value(
            &mut self,
            new_value: u64,
        ) -> impl Call<Output = ReceiverEvents, Args = Self::Args>;
        fn set_string_value(
            &mut self,
            new_value: String,
        ) -> impl Call<Output = ReceiverEvents, Args = Self::Args>;
    }
}
#[cfg(feature = "with_mocks")]
#[cfg(not(target_arch = "wasm32"))]
extern crate std;
#[cfg(feature = "with_mocks")]
#[cfg(not(target_arch = "wasm32"))]
pub mod mockall {
    use super::*;
    use sails_rs::mockall::*;
    mock! { pub QuerySvc<A> {} #[allow(refining_impl_trait)] #[allow(clippy::type_complexity)] impl<A> traits::QuerySvc for QuerySvc<A> { type Args = A; fn all_callers (& self, ) -> MockQuery<A, Vec<ActorId>>;fn state_num_value (& self, ) -> MockQuery<A, u64>;fn state_string_value (& self, ) -> MockQuery<A, String>;fn vec_string_value (& self, ) -> MockQuery<A, Vec<String>>; } }
    mock! { pub Receiver<A> {} #[allow(refining_impl_trait)] #[allow(clippy::type_complexity)] impl<A> traits::Receiver for Receiver<A> { type Args = A; fn add_string_to_vec (&mut self, value: String,) -> MockCall<A, ReceiverEvents>;fn set_num_value (&mut self, new_value: u64,) -> MockCall<A, ReceiverEvents>;fn set_string_value (&mut self, new_value: String,) -> MockCall<A, ReceiverEvents>; } }
}
