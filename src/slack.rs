use serde::Deserialize;
use serde::Serialize;

/// Slack form message: as documented at:
/// https://api.slack.com/interactivity/slash-commands#app_command_handling
#[derive(Serialize, Deserialize, Debug)]
pub struct SlackReceivedCommand {
    pub token: String,
    pub text: String,
    pub team_id: String,
    pub team_domain: String,
    pub enterprise_id: Option<String>,
    pub enterprise_name: Option<String>,
    pub channel_id: String,
    pub channel_name: String,
    pub user_id: String,
    pub user_name: String,
    pub command: String,
    pub response_url: String,
    pub trigger_id: String,
    pub api_app_id: String,
}

///Slack Command Error HttpResponse
#[derive(Serialize, Deserialize, Debug)]
pub struct SlackCommandErrorResponse {
    pub response_type: String,
    pub text: String,
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
