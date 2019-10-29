const MAX_HTTP_REDIRECTS_ALLOWED: usize = 10;

use crate::errors::{self, ErrorKind, Result, ResultExt};
use crate::status::StatusBackend;
use crate::tt_note;
use reqwest::RedirectPolicy;

pub use reqwest::{Client, Error, Response, StatusCode};

pub fn get(url: &str) -> Result<Response> {
    Ok(Client::new().get(url).send()?)
}

pub fn get_range_inclusive(
    client: &mut Client,
    url: &str,
    start: u64,
    end: u64,
) -> Result<Response> {
    Ok(client
        .get(url)
        .header("Range", &format!("bytes={}-{}", start, end))
        .send()?)
}

pub fn resolve_url(url: &str, status: &mut dyn StatusBackend) -> Result<String> {
    tt_note!(status, "connecting to {}", url);

    // First, we actually do a HEAD request on the URL for the data file.
    // If it's redirected, we update our URL to follow the redirects. If
    // we didn't do this separately, the index file would have to be the
    // one with the redirect setup, which would be confusing and annoying.

    let redirect_policy = RedirectPolicy::custom(|attempt| {
        // In the process of resolving the file url it might be neccesary
        // to stop at a certain level of redirection. This might be required
        // because some hosts might redirect to a version of the url where
        // it isn't possible to select the index file by appending .index.gz.
        // (This mostly happens because CDNs redirect to the file hash.)
        if attempt.previous().len() >= MAX_HTTP_REDIRECTS_ALLOWED {
            attempt.too_many_redirects()
        } else if let Some(segments) = attempt.url().clone().path_segments() {
            let follow = segments
                .last()
                .map(|file| file.contains('.'))
                .unwrap_or(true);
            if follow {
                attempt.follow()
            } else {
                attempt.stop()
            }
        } else {
            attempt.follow()
        }
    });
    let res = Client::builder()
        .redirect(redirect_policy)
        .build()?
        .head(url)
        .send()?;

    if !(res.status().is_success() || res.status() == StatusCode::FOUND) {
        return Err(errors::Error::from(ErrorKind::UnexpectedHttpResponse(
            url.to_string(),
            res.status(),
        )))
        .chain_err(|| "couldn\'t probe".to_string());
    }

    let final_url = res.url().clone().into_string();

    if final_url != url {
        tt_note!(status, "resolved to {}", final_url);
    }

    Ok(final_url)
}
