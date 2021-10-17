use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug)]
pub struct SlackReceivedForm {
  pub token: String,
  pub text: String,
  pub user_name: String,
  pub team_id: String,
  pub team_domain: String,
  pub enterprise_id: String,
  pub enterprise_name: String,
  pub channel_id: String,
  pub channel_name: String,
  pub user_id: String,
  pub command: String,
  pub response_url: String,
}

///Attachment is the internal structure that contains the information inside a @SlackCommandResponse
#[derive(Serialize, Deserialize, Debug)]
pub struct Attachment {
  pub fallback: String,
  pub color: String,
  pub author_name: String,
  pub image_url: String,
  pub ts: u64,
}

///SlackCommandResponse is the json structure that is required to send a slack '/' command response back to the requestor or channel
#[derive(Serialize, Deserialize, Debug)]
pub struct SlackCommandResponse {
  pub response_type: String, //in_channel for response to the full channel
  pub attachments: Vec<Attachment>,
}
