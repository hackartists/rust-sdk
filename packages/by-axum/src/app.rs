use std::{
    cell::RefCell,
    error::Error,
    future::Future,
    pin::Pin,
    rc::Rc,
    sync::{Arc, Mutex},
    thread::spawn,
};

use crate::lambda_adapter;
use axum::{routing::get, Router};
use tokio_cron_scheduler::{Job, JobScheduler};

pub struct App {
    pub router: Router,
    pub scheduler: Option<JobScheduler>,
}

impl App {
    pub fn new() -> Self {
        Self {
            router: Router::new().route("/version", get(version)),
            scheduler: None,
        }
    }

    pub async fn with_scheduler<S: ToString, Fut>(
        mut self,
        schedule: S,
        handler: fn() -> Fut,
    ) -> Result<Self, Box<dyn Error>>
    where
        Fut: Future<Output = ()> + Send + Sync + 'static,
    {
        if self.scheduler.is_none() {
            self.scheduler = Some(JobScheduler::new().await?);
        }

        let _ = self
            .scheduler
            .as_mut()
            .unwrap()
            .add(
                Job::new_async(
                    schedule,
                    move |_uuid, _lock| -> Pin<Box<dyn Future<Output = ()> + Send>> {
                        Box::pin(async move {
                            handler().await;
                        })
                    },
                )
                .expect("failed to create a new job"),
            )
            .await?;

        Ok(self)
    }

    pub fn nest(mut self, path: &str, router: Router) -> Self {
        self.router = self.router.nest(path, router);

        self
    }

    pub async fn serve(
        self,
        _tcp_listener: tokio::net::TcpListener,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let app = self.router.layer(tower_http::cors::CorsLayer::permissive());

        #[cfg(not(feature = "lambda"))]
        axum::serve(_tcp_listener, app).await?;

        #[cfg(feature = "lambda")]
        {
            lambda_runtime::run(lambda_adapter::LambdaAdapter::from(app.into_service()))
                .await
                .unwrap();
        }

        Ok(())
    }
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
