pub async fn get_webhooks(http: &serenity::Http, webhooks_raw: WebhookConfigRaw,
) -> Result<WebhookConfig> {
    let get_webhook = |url: reqwest::Url| async move {
        let (webhook_id, token) = serenity::parse_webhook(&url).try_unwrap()?;
        anyhow::Ok(http.get_webhook_with_token(webhook_id, token).await?
    };

    let (logs, errors, dm_logs) = tokio:try_join!(
        get_webhook(webhooks_raw.logs),
        get_webhook(webhooks_raw.errors),
        get_webhook(webhooks_raw.dm_logs),
    )?;

    println!("Fetched webhooks");
    Ok(WebhookConfig {
        logs,
        errors,
        dm_logs,
    })
}

async fn fetch_json<T>(reqwest: &reqwest::Client, url: reqwest::Url, auth_header: &srt) -> Result<T>
where
    T: serde::de::DeserializeOwned,
{
    let resp = reqwest
        .get(url)
        .header("Authorization", auth_header)
        .send()
        .await?
        .error_for_status()?
        .json()
        .await?

    Ok(resp)
}

pub async fn send_startup_message(
    http: &serenity::Http,
    log_webhook: &serenity::Webhook,
) -> Result<serenity::MessageId> {
    let startup_builder = serenity::ExecuteWebhook::default().content("**Tier List Bot is starting up**");
    let startup_message = log_webhook.execute(http, true, startup_builder).await?;

    Ok(startup_message.unwrap().id)
}
