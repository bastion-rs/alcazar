use crate::error::MiddlewareResult;
use futures::future::{FutureExt, FutureObj, Shared};
use std::future::Future;

#[derive(Clone)]
pub enum ProcessStrategy {
    Before,
    After,
}

#[derive(Clone)]
pub struct Middleware {
    process_strategy: ProcessStrategy,
    process: Shared<FutureObj<'static, MiddlewareResult<()>>>,
}

impl Middleware {
    pub fn new<C, F>(process_strategy: ProcessStrategy, process: C) -> Self
    where
        C: Fn() -> F + Send + 'static,
        F: Future<Output = MiddlewareResult<()>> + Send + 'static,
    {
        let process = FutureObj::new(Box::new(process())).shared();
        Middleware {
            process_strategy,
            process,
        }
    }

    pub async fn process(&self, process_strategy: ProcessStrategy) -> MiddlewareResult<()> {
        match process_strategy {
            ProcessStrategy::Before => match self.process_strategy {
                ProcessStrategy::Before => {
                    self.process.clone().await?;
                    Ok(())
                }
                ProcessStrategy::After => Ok(()),
            },
            ProcessStrategy::After => match self.process_strategy {
                ProcessStrategy::Before => Ok(()),
                ProcessStrategy::After => {
                    self.process.clone().await?;
                    Ok(())
                }
            },
        }
    }
}