use std::convert::Infallible;

use aide::{
    axum::{
        routing::{get, ApiMethodRouter},
        ApiRouter, IntoApiResponse,
    },
    openapi::{Info, OpenApi},
};
use axum::{body::Body, extract::Request, response::IntoResponse, routing::Route, Extension, Json};
use tower::{Layer, Service};

pub struct BiyardRouter<S = ()> {
    pub open_api: OpenApi,
    pub inner: ApiRouter<S>,
}

impl<S> BiyardRouter<S>
where
    S: Clone + Send + Sync + 'static,
{
    pub fn new() -> Self {
        Self {
            open_api: OpenApi {
                info: Info {
                    description: Some("an example API".to_string()),
                    ..Info::default()
                },
                ..OpenApi::default()
            },
            inner: ApiRouter::new()
                .api_route("/version", get(version))
                .route("/api", get(serve_api)),
        }
    }

    pub fn route(self, path: &str, method_router: ApiMethodRouter<S>) -> Self {
        let path = path
            .split("/")
            .map(|s| {
                if s.starts_with(":") {
                    format!("{{{}}}", s.replace(":", ""))
                } else {
                    s.to_string()
                }
            })
            .collect::<Vec<String>>()
            .join("/");

        Self {
            inner: self.inner.api_route(&path, method_router),
            ..self
        }
    }

    pub fn route_service<T>(self, path: &str, service: T) -> Self
    where
        T: Service<Request<Body>, Error = Infallible> + Clone + Send + Sync + 'static,
        T::Response: IntoResponse,
        T::Future: Send + 'static,
    {
        Self {
            inner: self.inner.route_service(path, service),
            ..self
        }
    }

    pub fn nest<R>(self, path: &str, router: R) -> Self
    where
        R: Into<BiyardRouter<S>>,
    {
        Self {
            inner: self.inner.nest(path, router.into().inner),
            ..self
        }
    }

    pub fn nest_service<T>(self, path: &str, service: T) -> Self
    where
        T: Service<Request<Body>, Error = Infallible> + Clone + Send + Sync + 'static,
        T::Response: IntoResponse,
        T::Future: Send + 'static,
    {
        Self {
            inner: self.inner.nest_service(path, service),
            ..self
        }
    }

    pub fn merge<R>(self, other: R) -> Self
    where
        R: Into<ApiRouter<S>>,
    {
        Self {
            inner: self.inner.merge(other.into()),
            ..self
        }
    }

    pub fn layer<L>(self, layer: L) -> Self
    where
        L: Layer<Route> + Clone + Send + Sync + 'static,
        L::Service: Service<Request<Body>> + Clone + Send + Sync + 'static,
        <L::Service as Service<Request<Body>>>::Response: IntoResponse + 'static,
        <L::Service as Service<Request<Body>>>::Error: Into<Infallible> + 'static,
        <L::Service as Service<Request<Body>>>::Future: Send + 'static,
    {
        Self {
            inner: self.inner.layer(layer),
            ..self
        }
    }

    pub fn with_state<S2>(self, state: S) -> BiyardRouter<S2> {
        BiyardRouter {
            inner: self.inner.with_state(state),
            open_api: self.open_api,
        }
    }
}

impl Into<ApiRouter> for BiyardRouter {
    fn into(self) -> ApiRouter {
        self.inner
    }
}

async fn serve_api(Extension(api): Extension<OpenApi>) -> impl IntoApiResponse {
    Json(api)
}

async fn version() -> String {
    match option_env!("VERSION") {
        Some(version) => match option_env!("COMMIT") {
            Some(commit) => format!("{}-{}", version, commit),
            None => version.to_string(),
        },
        None => match option_env!("DATE") {
            Some(date) => date.to_string(),
            None => "unknown".to_string(),
        },
    }
    .to_string()
}
