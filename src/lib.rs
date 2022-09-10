use actix_middleware_ed25519_authentication::{authenticate_request, MiddlewareData};
use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    error::ErrorUnauthorized,
    Error,
};
use futures_util::future::LocalBoxFuture;
use futures_util::FutureExt;
use std::{future::Ready, pin::Pin, rc::Rc};

type Snowflake = String;
type Timestamp = String;

pub struct Application {
    pub id: Snowflake,
    pub name: String,
    pub icon: Option<String>,
    pub description: String,
    pub rpc_origins: Option<Vec<String>>,
    pub bot_public: bool,
    pub bot_require_code_grant: bool,
    pub terms_of_service_url: Option<String>,
    pub privacy_policy_url: Option<String>,
    pub owner: Option<User>,
    pub verify_key: String,
    pub team: Option<Team>,
    pub guild_id: Option<Snowflake>,
    pub primary_sku_id: Option<Snowflake>,
    pub slug: Option<String>,
    pub cover_image: Option<String>,
    pub flags: Option<u32>,
    pub tags: Option<Vec<String>>,
    pub install_params: Option<InstallParams>,
    pub custom_install_url: Option<String>,
}

pub struct Team {
    pub icon: Option<String>,
    pub id: Snowflake,
    pub members: Vec<TeamMember>,
    pub name: String,
    pub owner_user_id: Snowflake,
}

pub struct TeamMember {
    pub membership_state: u32,
    pub permissions: Vec<String>,
    pub team_id: Snowflake,
    pub user: User,
}

pub struct InstallParams {
    pub scopes: Vec<String>,
    pub permissions: String,
}

pub struct Interaction {
    pub id: Snowflake,
    pub application_id: Snowflake,
    pub interaction_type: u8,
    pub data: Option<Rc<dyn InteractionData>>,
    pub guild_id: Option<Snowflake>,
    pub channel_id: Option<Snowflake>,
    pub member: Option<GuildMember>,
    pub user: Option<User>,
    pub token: String,
    pub version: u8,
    pub mesage: Option<Message>,
    pub app_permissions: Option<String>,
    pub locale: Option<String>,
    pub guild_locale: Option<String>,
}

pub trait InteractionData {}
pub struct ApplicationCommandData {
    pub id: Snowflake,
    pub name: String,
    pub command_type: u8,
    pub resolved: Option<Resolved>,
    pub options: Option<Vec<CommandOptions>>,
    pub guild_id: Option<Snowflake>,
    pub target_id: Option<Snowflake>,
}
impl InteractionData for ApplicationCommandData {}

pub struct ModalSubmitData {
    pub custom_id: String,
    pub components: Vec<MessageComponent>,
}
impl InteractionData for ModalSubmitData {}

pub struct Resolved {
    pub users: Option<Vec<Snowflake>>,
    pub members: Option<Vec<Snowflake>>,
    pub roles: Option<Vec<Snowflake>>,
    pub channels: Option<Vec<Snowflake>>,
    pub messages: Option<Vec<Snowflake>>,
    pub attachments: Option<Vec<Snowflake>>,
}

pub struct CommandOptions {
    pub name: String,
    pub command_type: u8,
    pub value: Option<String>,
    pub options: Option<Vec<CommandOptions>>,
    pub focused: Option<bool>,
}

pub struct GuildMember {
    pub user: Option<User>,
    pub nick: Option<String>,
    pub avatar: Option<String>,
    pub roles: Vec<Snowflake>,
    pub joined_at: Timestamp,
    pub premium_since: Option<Timestamp>,
    pub deaf: bool,
    pub mute: bool,
    pub pending: Option<bool>,
    pub permissions: Option<String>,
    pub communication_disabled_until: Option<Timestamp>,
}

pub struct User {
    pub id: Snowflake,
    pub username: String,
    pub discriminator: String,
    pub avatar: Option<String>,
    pub bot: Option<bool>,
    pub system: Option<bool>,
    pub mfa_enabled: Option<bool>,
    pub banner: Option<String>,
    pub accent_color: Option<u32>,
    pub locale: Option<String>,
    pub verified: Option<bool>,
    pub email: Option<String>,
    pub flags: Option<u32>,
    pub premium_type: Option<u8>,
    pub public_flags: Option<u32>,
}

pub struct Message {
    pub id: Snowflake,
    pub channel_id: Snowflake,
    pub author: User,
    pub content: String,
    pub timestamp: Timestamp,
    pub edited_timestamp: Option<Timestamp>,
    pub tts: bool,
    pub mention_everyone: bool,
    pub mentions: Vec<User>,
    pub mention_roles: Vec<Snowflake>,
    pub mention_channels: Option<Vec<ChannelMention>>,
    pub attachments: Vec<Attachment>,
    pub embeds: Vec<Embed>,
    pub reactions: Option<Vec<Reaction>>,
    pub nonce: Option<String>,
    pub webhook_id: Option<Snowflake>,
    pub message_type: u8,
    pub activity: Option<MessageActivity>,
    pub application: Option<Application>,
    pub application_id: Option<Snowflake>,
    pub message_reference: Option<MessageReference>,
    pub flags: Option<u32>,
    pub referenced_message: Option<Box<Message>>,
    pub interaction: Option<MessageInteraction>,
    pub thread: Option<Channel>,
    pub components: Option<Vec<MessageComponent>>,
    pub sticker_items: Option<Vec<StickerItem>>,
    pub position: Option<u32>,
}

pub struct MessageReference {
    pub message_id: Option<Snowflake>,
    pub channel_id: Option<Snowflake>,
    pub guild_id: Option<Snowflake>,
    pub fail_if_not_exists: Option<bool>,
}

pub struct MessageInteraction {
    pub id: Snowflake,
    pub interaction_type: u8,
    pub name: String,
    pub user: User,
    pub member: Option<GuildMember>,
}

pub struct MessageComponent {
    pub custom_id: String,
    pub component_type: u8,
    pub values: Option<Vec<String>>,
}
impl InteractionData for MessageComponent {}

pub struct MessageActivity {
    pub message_type: u8,
    pub party_id: Option<String>,
}

pub struct StickerItem {
    pub id: Snowflake,
    pub name: String,
    pub format_type: u8,
}

pub struct Channel {
    pub id: Snowflake,
    pub channel_type: u8,
    pub guild_id: Option<Snowflake>,
    pub position: Option<u32>,
    pub permission_overwrites: Option<Vec<Overwrite>>,
    pub name: Option<String>,
    pub topic: Option<String>,
    pub nsfw: Option<bool>,
    pub last_message_id: Option<Snowflake>,
    pub bitrate: Option<u32>,
    pub user_limit: Option<u32>,
    pub rate_limit_per_user: Option<u32>,
    pub recipients: Option<Vec<User>>,
    pub icon: Option<String>,
    pub owner_id: Option<Snowflake>,
    pub application_id: Option<Snowflake>,
    pub parent_id: Option<Snowflake>,
    pub last_pin_timestamp: Option<Timestamp>,
    pub rtc_region: Option<String>,
    pub video_quality_mode: Option<u8>,
    pub message_count: Option<u32>,
    pub member_count: Option<u32>,
    pub thread_metadata: Option<ThreadMetadata>,
    pub member: Option<ThreadMember>,
    pub default_auto_archive_duration: Option<u32>,
    pub permissions: Option<String>,
    pub flags: Option<u32>,
    pub total_messages_sent: Option<u32>,
    pub available_tags: Option<Vec<Tag>>,
    pub applied_tags: Option<Vec<Snowflake>>,
    pub default_reaction_emoji: Option<DefaultReaction>,
    pub default_thread_rate_limit_per_user: Option<u32>,
}
pub struct ChannelMention {
    pub id: Snowflake,
    pub guild_id: Snowflake,
    pub channel_type: u8,
    pub name: String,
}

pub struct ThreadMetadata {
    pub archived: bool,
    pub auto_archive_duration: u32,
    pub archive_timestamp: Timestamp,
    pub locked: bool,
    pub invitable: Option<bool>,
    pub create_timestamp: Option<Timestamp>,
}

pub struct ThreadMember {
    pub id: Option<Snowflake>,
    pub user_id: Option<Snowflake>,
    pub join_timestamp: Timestamp,
    pub flags: u32,
}

pub struct DefaultReaction {
    pub emoji_id: Snowflake,
    pub emoji_name: Option<String>,
}

pub struct Tag {
    pub id: Snowflake,
    pub name: String,
    pub moderated: bool,
    pub emoji_id: Option<Snowflake>,
    pub emoji_name: Option<String>,
}

pub struct Overwrite {
    pub id: Snowflake,
    pub overwrite_type: u8,
    pub allow: String,
    pub deny: String,
}

pub struct Attachment {
    pub id: Snowflake,
    pub filename: String,
    pub description: Option<String>,
    pub content_type: Option<String>,
    pub size: u32,
    pub url: String,
    pub proxy_url: String,
    pub height: Option<u32>,
    pub width: Option<u32>,
    pub ephemeral: Option<bool>,
}

pub struct Embed {
    pub title: Option<String>,
    pub embed_type: Option<String>,
    pub description: Option<String>,
    pub url: Option<String>,
    pub timestamp: Option<Timestamp>,
    pub color: Option<u32>,
    pub footer: Option<EmbedFooter>,
    pub image: Option<EmbedImage>,
    pub thumbnail: Option<EmbedThumbnail>,
    pub video: Option<EmbedVideo>,
    pub provider: Option<EmbedProvider>,
    pub author: Option<EmbedAuthor>,
    pub fields: Option<Vec<EmbedField>>,
}

pub struct EmbedFooter {
    pub text: String,
    pub icon_url: Option<String>,
    pub proxy_icon_url: Option<String>,
}

pub struct EmbedImage {
    pub url: String,
    pub proxy_url: Option<String>,
    pub height: Option<u32>,
    pub width: Option<u32>,
}

type EmbedThumbnail = EmbedImage;

pub struct EmbedVideo {
    pub url: Option<String>,
    pub proxy_url: Option<String>,
    pub height: Option<u32>,
    pub width: Option<u32>,
}

pub struct EmbedProvider {
    pub name: Option<String>,
    pub url: Option<String>,
}

pub struct EmbedAuthor {
    pub name: String,
    pub url: Option<String>,
    pub icon_url: Option<String>,
    pub proxy_icon_url: Option<String>,
}

pub struct EmbedField {
    pub name: String,
    pub value: String,
    pub inline: Option<bool>,
}

pub struct Reaction {
    pub count: u32,
    pub me: bool,
    pub emoji: Emoji,
}

pub struct Emoji {
    pub id: Option<Snowflake>,
    pub name: Option<String>,
    pub roles: Option<Vec<Snowflake>>,
    pub user: Option<User>,
    pub require_colons: Option<bool>,
    pub managed: Option<bool>,
    pub animated: Option<bool>,
    pub available: Option<bool>,
}

pub struct DiscordInteractions {
    pub public_key: String,
}

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
            public_key: self.public_key.clone(),
        }))
    }
}

pub struct DiscordInteractionsMiddleware<S> {
    service: Rc<S>,
    public_key: String,
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
        let key = self.public_key.clone();

        async move {
            // Authenticate
            let result = authenticate_request(&mut req, &MiddlewareData::new(&key)).await;
            if result.is_err() {
                return Err(ErrorUnauthorized("Unauthorized"));
            }

            // Ping handler

            let fut = srv.call(req);
            let res = fut.await?;
            Ok(res)
        }
        .boxed_local()
    }
}
