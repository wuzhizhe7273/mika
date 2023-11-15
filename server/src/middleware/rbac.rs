use axum::{body::Body, http::Request, response::Response};
use futures_util::Future;
use http::{header::AUTHORIZATION, HeaderValue};
use pin_project::pin_project;
use std::{
    convert::Infallible,
    pin::Pin,
    task::{Context, Poll},
};
use tower::{BoxError, Layer, Service};

#[derive(Clone)]
struct RBACLayer;

impl<S> Layer<S> for RBACLayer {
    type Service = RBAC<S>;

    fn layer(&self, inner: S) -> Self::Service {
        RBAC { inner }
    }
}

#[derive(Clone)]
struct RBAC<S> {
    inner: S,
}

impl<S> RBAC<S> {
    pub fn new(inner: S) -> Self {
        Self { inner: inner }
    }
}

impl<S> Service<Request<Body>> for RBAC<S>
where
    S: Service<Request<Body>, Response = Response, Error = Infallible> + Send + Clone + 'static,
    S::Future: Send + 'static,
{
    type Response = S::Response;
    type Error = BoxError;
    type Future = ResponseFuture<S::Future>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx).map_err(Into::into)
    }

    fn call(&mut self, request: Request<Body>) -> Self::Future {
        let clone = self.inner.clone();
        let mut inner = std::mem::replace(&mut self.inner, clone);
        let path = request.uri().path().to_string();
        let token = request.headers().get(AUTHORIZATION).cloned();
        ResponseFuture {
            inner: inner.call(request),
            path: path,
            token: token,
        }
    }
}

#[pin_project]
pub struct ResponseFuture<F> {
    #[pin]
    pub inner: F,
    pub path: String,
    pub token: Option<HeaderValue>,
}

impl<F, Response, Error> Future for ResponseFuture<F>
where
    F: Future<Output = Result<Response, Error>>,
    Error: Into<BoxError>,
{
    type Output = Result<Response, BoxError>;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.project();
        match this.inner.poll(cx) {
            Poll::Ready(result) => {
                let result = result.map_err(Into::into);
                return Poll::Ready(result);
            }
            Poll::Pending => {}
        }
        Poll::Pending
    }
}

fn get_token(str: Option<HeaderValue>) -> Result<String, BoxError> {
    let mut token_str = String::new();
    match str {
        Some(header) => {
            let t = header.to_str().map_err(|e| RBACError::ParsedError)?;
            let t=t.split_whitespace().collect::<Vec<&str>>();
            if t.len() < 2{
                return Err(Box::new(RBACError::ParsedError));
            }
            token_str=t[1].to_string();
        }
        None => return Err(Box::new(RBACError::ParsedError))
    }
    
    
    todo!()
}

#[derive(thiserror::Error, Debug, Clone)]
enum RBACError {
    #[error("token parsed error")]
    ParsedError,
}
