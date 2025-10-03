use database::{
    Database,
    models::service::{Service, ServiceCreate, ServicePatch},
};
use serde::{Deserialize, Serialize};

use crate::error::AppError;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ServicePayloadAdmin {
    pub id: u32,
    pub name: String,
    pub api_key: String,
}

impl From<Service> for ServicePayloadAdmin {
    fn from(value: Service) -> Self {
        Self {
            id: value.id,
            name: value.name,
            api_key: value.api_key,
        }
    }
}

impl ServicePayloadAdmin {
    pub async fn all(db: &Database) -> Result<Vec<Self>, AppError> {
        Ok(db
            .services()
            .all()
            .await?
            .into_iter()
            .map(|service| service.into())
            .collect())
    }
}
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ServicePayloadUser {
    pub id: u32,
    pub name: String,
}

impl From<Service> for ServicePayloadUser {
    fn from(value: Service) -> Self {
        Self {
            id: value.id,
            name: value.name,
        }
    }
}

impl ServicePayloadUser {
    pub async fn all(db: &Database) -> Result<Vec<Self>, AppError> {
        Ok(db
            .services()
            .all()
            .await?
            .into_iter()
            .map(|service| service.into())
            .collect())
    }
}

#[derive(Serialize, Deserialize)]
pub struct ServiceCreatePayload {
    pub name: String,
}

impl ServiceCreatePayload {
    pub async fn create(self, db: &Database) -> Result<ServicePayloadAdmin, AppError> {
        let service = db.services().create(self.into()).await?;
        Ok(service.into())
    }
}

impl From<ServiceCreatePayload> for ServiceCreate {
    fn from(value: ServiceCreatePayload) -> Self {
        ServiceCreate { name: value.name }
    }
}

#[derive(Serialize, Deserialize)]
pub struct ServicePatchPayload {
    pub name: String,
}

impl ServicePatchPayload {
    pub async fn patch(
        self,
        service_id: u32,
        db: &Database,
    ) -> Result<ServicePayloadAdmin, AppError> {
        let service = db.services().patch(service_id, self.into()).await?;
        Ok(service.into())
    }
}

impl From<ServicePatchPayload> for ServicePatch {
    fn from(value: ServicePatchPayload) -> Self {
        Self { name: value.name }
    }
}
