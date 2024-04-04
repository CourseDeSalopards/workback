use log::{error, warn, info, debug, trace, LevelFilter};
use function_name::named;
use prost_types::Value;
use tonic::{metadata::{MetadataMap, MetadataValue}, service::interceptor, transport::ClientTlsConfig, Request, Response, Streaming};

use crate::local_env::*;

pub mod roomapi {
    include!(concat!(env!("OUT_DIR"), "/room_api.rs"));
    pub const FILE_DESCRIPTOR_SET: &[u8] = tonic::include_file_descriptor_set!("roomapi_descriptor");
}

use roomapi::{
    room_api_client::RoomApiClient,
    VariableRequest,
    SaveVariableRequest,
    EventRequest,
    EventResponse,
    DispatchEventRequest
};

#[named]
async fn get_client() -> Result<RoomApiClient<tonic::transport::Channel>, tonic::Status> {
    let dst = format!("https://{}:{}", ROOMAPI_HOST.to_string(), *ROOMAPI_PORT);

    let endpoint = tonic::transport::Channel::from_shared(dst).map_err(|e| {
        error!("[{}] - {e}", function_name!());
        tonic::Status::invalid_argument("Invalid url")
    })?
    .tls_config(
        ClientTlsConfig::new().domain_name("room-api.workadventu.re")
    ).map_err(|e| {
        tonic::Status::invalid_argument("Invalid url")
    })?;

    let channel = match endpoint.connect().await {
        Ok(channel) => Ok(channel),
        Err(e) => {
            error!("[{}] - {}", function_name!(), e);
            Err(tonic::Status::invalid_argument("Failed to connect"))
        },
    }?;

    Ok(RoomApiClient::new(channel))
}

#[named]
pub async fn read_variable(variable_name: &str) -> Result<Response<Value>, tonic::Status> {
    let mut client = get_client().await?;

    let message = VariableRequest {
        room: ROOM_URL.to_string(),
        name: variable_name.to_string(),
    };

    let mut request: Request<VariableRequest> = Request::new(message);
    request.metadata_mut().insert("x-api-key", X_API_KEY.to_string().parse().unwrap());

    match client.read_variable(request).await {
        Ok(response) => {
            info!("[{}] - Variable read", function_name!());
            Ok(response)
        },
        Err(e) => {
            error!("[{}] - Failed to read variable: {}", function_name!(), e);
            Err(e)
        }
    }
}

#[named]
pub async fn listen_variable(variable_name: &str, value: &str) -> Result<Response<Streaming<Value>>, tonic::Status> {
    let mut client = get_client().await?;

    let message = VariableRequest {
        room: ROOM_URL.to_string(),
        name: variable_name.to_string(),
    };

    let mut request: Request<VariableRequest> = Request::new(message);
    request.metadata_mut().insert("x-api-key", X_API_KEY.to_string().parse().unwrap());

    //TODO
    match client.listen_variable(request).await {
        Ok(response) => {
            info!("[{}] - Variable listen", function_name!());
            Ok(response)
        },
        Err(e) => {
            error!("[{}] - Failed to listen variable: {}", function_name!(), e);
            Err(e)
        }
    }
}


#[named]
pub async fn save_variable(variable_name: &str, value: Value) -> Result<Response<()>, tonic::Status> {
    let mut client = get_client().await?;

    let message = SaveVariableRequest {
        room: ROOM_URL.to_string(),
        name: variable_name.to_string(),
        value: Some(value),
    };

    let mut request: Request<SaveVariableRequest> = Request::new(message);
    request.metadata_mut().insert("x-api-key", X_API_KEY.to_string().parse().unwrap());

    match client.save_variable(request).await {
        Ok(response) => {
            info!("[{}] - Variable saved", function_name!());
            Ok(response)
        },
        Err(e) => {
            error!("[{}] - Failed to save variable: {}", function_name!(), e);
            Err(e)
        }
    }
}


#[named]
pub async fn broadcast_event(variable_name: &str, data: Value, event: &str) -> Result<Response<()>, tonic::Status> {
    let mut client = get_client().await?;

    let target_user_ids = vec![1, 2, 3];

    let message = DispatchEventRequest {
        room: ROOM_URL.to_string(),
        name: event.to_string(),
        data: Some(data),
        target_user_ids,
    };

    let mut request: Request<DispatchEventRequest> = Request::new(message);
    request.metadata_mut().insert("x-api-key", X_API_KEY.to_string().parse().unwrap());

    match client.broadcast_event(request).await {
        Ok(response) => {
            info!("[{}] - Broadcast {}", event, function_name!());
            Ok(response)
        },
        Err(e) => {
            error!("[{}] - Failed to broadcast event {}: {}", event, function_name!(), e);
            Err(e)
        }
    }
}


#[named]
pub async fn listen_to_event(event: &str) -> Result<tonic::Response<Streaming<EventResponse>>, tonic::Status> {
    let mut client = get_client().await?;

    let message = EventRequest {
        room: ROOM_URL.to_string(),
        name: event.to_string(),
    };

    let mut request: Request<EventRequest> = Request::new(message);
    request.metadata_mut().insert("x-api-key", X_API_KEY.to_string().parse().unwrap());

    match client.listen_to_event(request).await {
        Ok(response) => {
            info!("[{}] - Listen to event {}", event, function_name!());
            Ok(response)
        },
        Err(e) => {
            error!("[{}] - Failed to listen to event {}: {}", event, function_name!(), e);
            Err(e)
        }
    }
}


