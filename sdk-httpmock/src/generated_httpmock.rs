// The contents of this file are generated; do not modify them.

pub mod operations {
    //! [`When`](httpmock::When) and [`Then`](httpmock::Then)
    //! wrappers for each operation. Each can be converted to
    //! its inner type with a call to `into_inner()`. This can
    //! be used to explicitly deviate from permitted values.
    use srvn::*;
    pub struct ServerListWhen(httpmock::When);
    impl ServerListWhen {
        pub fn new(inner: httpmock::When) -> Self {
            Self(
                inner
                    .method(httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/dedicated/server$").unwrap()),
            )
        }

        pub fn into_inner(self) -> httpmock::When {
            self.0
        }
    }

    pub struct ServerListThen(httpmock::Then);
    impl ServerListThen {
        pub fn new(inner: httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::ServerListResponse) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct ServerViewWhen(httpmock::When);
    impl ServerViewWhen {
        pub fn new(inner: httpmock::When) -> Self {
            Self(
                inner
                    .method(httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/dedicated/server/[^/]*$").unwrap()),
            )
        }

        pub fn into_inner(self) -> httpmock::When {
            self.0
        }

        pub fn server(self, value: &types::ServerId) -> Self {
            let re =
                regex::Regex::new(&format!("^/dedicated/server/{}$", value.to_string())).unwrap();
            Self(self.0.path_matches(re))
        }
    }

    pub struct ServerViewThen(httpmock::Then);
    impl ServerViewThen {
        pub fn new(inner: httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::Server) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }
}

/// An extension trait for [`MockServer`](httpmock::MockServer) that
/// adds a method for each operation. These are the equivalent of
/// type-checked [`mock()`](httpmock::MockServer::mock) calls.
pub trait MockServerExt {
    fn server_list<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::ServerListWhen, operations::ServerListThen);
    fn server_view<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::ServerViewWhen, operations::ServerViewThen);
}

impl MockServerExt for httpmock::MockServer {
    fn server_list<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::ServerListWhen, operations::ServerListThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::ServerListWhen::new(when),
                operations::ServerListThen::new(then),
            )
        })
    }

    fn server_view<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::ServerViewWhen, operations::ServerViewThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::ServerViewWhen::new(when),
                operations::ServerViewThen::new(then),
            )
        })
    }
}
