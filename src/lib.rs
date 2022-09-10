use actix_middleware_ed25519_authentication::{authenticate_request, MiddlewareData};
use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    error::ErrorUnauthorized,
    Error,
};
use futures_util::future::LocalBoxFuture;
use futures_util::FutureExt;
use std::{future::Ready, pin::Pin, rc::Rc};

pub struct DiscordInteractions {
    pub public_key: String,
}

impl<S, B> Transform<S, ServiceRequest> for DiscordInteractions
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = DiscordInteractionsMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        std::future::ready(Ok(DiscordInteractionsMiddleware {
            service: Rc::new(service),
            public_key: self.public_key.clone(),
        }))
    }
}

pub struct DiscordInteractionsMiddleware<S> {
    service: Rc<S>,
    public_key: String,
}

impl<S, B> Service<ServiceRequest> for DiscordInteractionsMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(
        &self,
        mut req: ServiceRequest,
    ) -> Pin<
        Box<
            (dyn futures_util::Future<Output = Result<ServiceResponse<B>, actix_web::Error>>
                 + 'static),
        >,
    > {
        let srv = self.service.clone();
        let key = self.public_key.clone();

        async move {
            // Authenticate
            let result = authenticate_request(&mut req, &MiddlewareData::new(&key)).await;
            if result.is_err() {
                return Err(ErrorUnauthorized("Unauthorized"));
            }

            let fut = srv.call(req);
            let res = fut.await?;
            Ok(res)
        }
        .boxed_local()
    }
}
