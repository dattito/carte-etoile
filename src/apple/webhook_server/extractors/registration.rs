use aide::{
    gen::GenContext,
    openapi::{MediaType, Operation, RequestBody, SchemaObject},
    operation::set_body,
    OperationInput,
};
use axum::{
    async_trait,
    extract::{FromRequest, Request},
    Json, RequestExt,
};
use indexmap::IndexMap;
use schemars::JsonSchema;

use crate::error::Error;

#[derive(serde::Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct DeviceRegistrationPushToken {
    pub push_token: String,
}

#[async_trait]
impl<S> FromRequest<S> for DeviceRegistrationPushToken
where
    S: Send + Sync,
{
    type Rejection = Error;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let Json(s): Json<DeviceRegistrationPushToken> = req.extract_with_state(state).await?;
        Ok(s)
    }
}

impl OperationInput for DeviceRegistrationPushToken {
    fn operation_input(ctx: &mut GenContext, operation: &mut Operation) {
        let schema = ctx.schema.subschema_for::<Self>().into_object();
        let resolved_schema = ctx.resolve_schema(&schema);

        set_body(
            ctx,
            operation,
            RequestBody {
                description: resolved_schema
                    .metadata
                    .as_ref()
                    .and_then(|m| m.description.clone()),
                content: IndexMap::from_iter([(
                    "application/json".into(),
                    MediaType {
                        schema: Some(SchemaObject {
                            json_schema: schema.into(),
                            example: None,
                            external_docs: None,
                        }),
                        ..Default::default()
                    },
                )]),
                required: true,
                extensions: IndexMap::default(),
            },
        );
    }
}
