use crate::client_error::ClientError;
use sdk::client::ConnectedClient;
use shared::streams::create_stream::CreateStream;

pub async fn handle(
    command: CreateStream,
    client: &mut ConnectedClient,
) -> Result<(), ClientError> {
    client.create_stream(&command).await?;
    Ok(())
}
