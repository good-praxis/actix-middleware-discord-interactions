use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error,
};
use futures_util::future::LocalBoxFuture;
use futures_util::FutureExt;
use std::{future::Ready, pin::Pin, rc::Rc};

pub struct DiscordInteractions {}

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
        }))
    }
}

pub struct DiscordInteractionsMiddleware<S> {
    service: Rc<S>,
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
        async move {
            let fut = srv.call(req);
            let res = fut.await?;
            Ok(res)
        }
        .boxed_local()
    }
}
