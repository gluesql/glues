use {
    crate::proxy_access::{
        GuardEvaluation, PendingMutation, PostPlan, ProxyAccess, apply_post_plan,
        collect_removal_plan, load_proxy_access,
    },
    color_eyre::Result,
    glues_core::backend::proxy::{
        ProxyServer,
        request::ProxyRequest,
        response::{ProxyResponse, ResultPayload},
    },
    glues_core::types::DirectoryId,
    std::sync::Arc,
    tokio::sync::{Mutex as AsyncMutex, RwLock},
};

pub struct ServerState {
    server: AsyncMutex<ProxyServer>,
    access: Option<Arc<RwLock<ProxyAccess>>>,
}

impl ServerState {
    pub async fn new(
        mut server: ProxyServer,
        allowed_directory: Option<DirectoryId>,
    ) -> Result<Self> {
        let access = initialize_proxy_access(allowed_directory, &mut server).await?;
        Ok(Self {
            server: AsyncMutex::new(server),
            access,
        })
    }

    pub async fn handle(&self, request: ProxyRequest) -> ProxyResponse {
        let mut post_plan = if self.access.is_some() {
            PostPlan::from_request(&request)
        } else {
            PostPlan::None
        };

        let mut pending = None;
        let access_arc = if let Some(access) = &self.access {
            let evaluation = {
                let guard = access.read().await;
                guard.evaluate(&request)
            };
            match evaluation {
                GuardEvaluation::ReturnRoot { root } => {
                    return ProxyResponse::Ok(ResultPayload::Id(root));
                }
                GuardEvaluation::Deny { message } => {
                    return ProxyResponse::Err(message);
                }
                GuardEvaluation::Allow { pending: p } => {
                    pending = p;
                    Some(Arc::clone(access))
                }
            }
        } else {
            None
        };

        let mut server = self.server.lock().await;

        if let Some(PendingMutation::CollectRemoval { directory_id }) = pending {
            match collect_removal_plan(&mut server, directory_id).await {
                Ok(plan) => {
                    post_plan = PostPlan::RemoveDirectory(plan);
                }
                Err(err) => {
                    return ProxyResponse::Err(err.to_string());
                }
            }
        }

        let response = server.handle(request).await;

        drop(server);

        if let Some(access) = access_arc.as_ref() {
            apply_post_plan(access, post_plan, &response).await;
        }

        response
    }
}

async fn initialize_proxy_access(
    allowed_directory: Option<DirectoryId>,
    server: &mut ProxyServer,
) -> Result<Option<Arc<RwLock<ProxyAccess>>>> {
    match allowed_directory {
        Some(root) => {
            let access = load_proxy_access(server, root).await?;
            Ok(Some(Arc::new(RwLock::new(access))))
        }
        None => Ok(None),
    }
}
