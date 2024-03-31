use crate::{
    nodes::lxc::{self, CloneLxcRequest, Interfaces},
    VmId,
};
use serde::{de::DeserializeOwned, Serialize};

use super::{Client, Error};

macro_rules! sub_client {
    ($ty:ident) => {
        impl<'a> $ty<'a> {
            fn get<O>(&self, path: &str) -> Result<O, Error>
            where
                O: DeserializeOwned,
            {
                self.client.get(&format!("{}/{}", self.path, path), &())
            }

            fn post<I, O>(&self, path: &str, request: &I) -> Result<O, Error>
            where
                I: Serialize,
                O: DeserializeOwned,
            {
                self.client
                    .post(&format!("{}/{}", self.path, path), request)
            }
        }
    };
}

sub_client!(NodeClient);

#[derive(Debug, Clone)]
pub struct NodeClient<'a> {
    client: &'a super::Client,
    path: String,
}

impl<'a> NodeClient<'a> {
    pub(crate) fn new<T>(client: &'a Client, node: T) -> Self
    where
        T: AsRef<str>,
    {
        Self {
            client,
            path: format!("/nodes/{}", node.as_ref()),
        }
    }

    pub fn lxc(&self, vm: VmId) -> LxcClient<'a> {
        LxcClient::new(self.client, &self.path, vm)
    }

    pub fn hosts(&self) -> Result<serde_json::Value, Error> {
        self.get("hosts")
    }

    pub fn start_all(&self) -> Result<String, Error> {
        self.post("startall", &())
    }
}

// LXC
#[derive(Debug, Clone)]
pub struct LxcClient<'a> {
    client: &'a super::Client,
    path: String,
}

sub_client!(LxcClient);

impl<'a> LxcClient<'a> {
    pub(crate) fn new(client: &'a crate::client::Client, path: &str, vm: VmId) -> Self {
        let path = format!("{}/lxc/{}", path, vm);
        Self { client, path }
    }

    pub fn clone(&self, request: &CloneLxcRequest) -> Result<String, Error> {
        self.post("clone", request)
    }

    pub fn status(&self) -> Result<lxc::Status, Error> {
        self.get("status/current")
    }

    pub fn start(&self) -> Result<String, Error> {
        self.post("status/start", &())
    }

    pub fn stop(&self) -> Result<String, Error> {
        self.post("status/stop", &())
    }

    pub fn interfaces(&self) -> Result<Interfaces, Error> {
        self.get("interfaces")
    }
}
