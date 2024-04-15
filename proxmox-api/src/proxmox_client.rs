use serde::{de::DeserializeOwned, Serialize};

/// A proxmox client, with a specific path.
pub trait ProxmoxClient {
    type Path: AsRef<str>;

    fn path(self) -> Self::Path;
}

pub trait ProxmoxClientAction<P, O, E>: ProxmoxClient
where
    P: Serialize,
    O: DeserializeOwned,
    E: Debug,
{
    fn exec(&self, p: &P) -> Result<O, E>;
}

#[cfg(test)]
mod test {
    use super::ProxmoxClient;

    #[derive(Default)]
    struct TestClient {
        path: String,
    }

    impl<'a> ProxmoxClient for &'a TestClient {
        type Path = String;

        fn path(self) -> Self::Path {
            self.path.to_string()
        }
    }

    /// Just to validate that we can get the path for all relevant
    /// combinations.
    #[allow(unused)]
    fn try_all_combos() {
        let mut client = TestClient::default();
        let path = client.path();

        let client_mut_ref = &mut client;
        let path = client_mut_ref.path();

        let client_ref = &client;
        let path = client_ref.path();
    }
}
