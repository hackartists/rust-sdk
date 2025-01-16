use std::convert::Infallible;

use axum::{
    extract::Request,
    response::IntoResponse,
    routing::{MethodRouter, Route, Router, RouterAsService, RouterIntoService},
};
use tower::{Layer, Service};

pub struct BiyardRouter<S = ()> {
    pub inner: Router<S>,
}

impl<S> BiyardRouter<S>
where
    S: Clone + Send + Sync + 'static,
{
    pub fn new() -> Self {
        Self {
            inner: Router::new(),
        }
    }

    pub fn route(self, path: &str, method_router: MethodRouter<S>) -> Self {
        Self {
            inner: self.inner.route(path, method_router),
        }
    }

    pub fn route_service<T>(self, path: &str, service: T) -> Self
    where
        T: Service<Request, Error = Infallible> + Clone + Send + 'static,
        T::Response: IntoResponse,
        T::Future: Send + 'static,
    {
        Self {
            inner: self.inner.route_service(path, service),
        }
    }

    pub fn nest<R>(self, path: &str, router: R) -> Self
    where
        R: Into<BiyardRouter<S>>,
    {
        Self {
            inner: self.inner.nest(path, router.into().inner),
        }
    }

    pub fn nest_service<T>(self, path: &str, service: T) -> Self
    where
        T: Service<Request, Error = Infallible> + Clone + Send + 'static,
        T::Response: IntoResponse,
        T::Future: Send + 'static,
    {
        Self {
            inner: self.inner.nest_service(path, service),
        }
    }

    pub fn merge<R>(self, other: R) -> Self
    where
        R: Into<Router<S>>,
    {
        Self {
            inner: self.inner.merge(other.into()),
        }
    }

    pub fn layer<L>(self, layer: L) -> Self
    where
        L: Layer<Route> + Clone + Send + 'static,
        L::Service: Service<Request> + Clone + Send + 'static,
        <L::Service as Service<Request>>::Response: IntoResponse + 'static,
        <L::Service as Service<Request>>::Error: Into<Infallible> + 'static,
        <L::Service as Service<Request>>::Future: Send + 'static,
    {
        Self {
            inner: self.inner.layer(layer),
        }
    }

    pub fn with_state(self, state: S) -> Self {
        Self {
            inner: self.inner.with_state(state),
        }
    }

    pub fn as_service<B>(&mut self) -> RouterAsService<'_, B, S> {
        self.inner.as_service()
    }

    pub fn into_service<B>(self) -> RouterIntoService<B, S> {
        self.inner.into_service()
    }
}

impl Into<Router> for BiyardRouter {
    fn into(self) -> Router {
        self.inner
    }
}

impl From<Router> for BiyardRouter {
    fn from(inner: Router) -> Self {
        Self { inner }
    }
}
