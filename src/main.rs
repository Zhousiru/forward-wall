mod cookie;
mod error;
mod page;

use anyhow::Context;
use axum::{
  extract::Path,
  http::{header::SET_COOKIE, HeaderMap, StatusCode},
  response::{IntoResponse, Response},
  routing::get,
  Router,
};
use axum_extra::extract::CookieJar;
use error::AppError;
use tokio::signal;

use crate::{cookie::build_cookie, page::AUTH_PAGE};

async fn auth(
  Path(passcode): Path<String>,
  headers: HeaderMap,
  jar: CookieJar,
) -> Result<Response, AppError> {
  let forwarded_method = headers
    .get("X-Forwarded-Method")
    .context("`X-Forwarded-Method` not found in the headers.")?
    .to_str()?
    .to_string()
    .to_uppercase();

  if let Some(cookie) = jar.get("forward_wall_passcode") {
    if cookie.value() == passcode {
      return Ok("ok".into_response());
    }
  }

  if let Some(input_passcode_header) = headers.get("Forward-Wall-Passcode") {
    if forwarded_method == "POST" {
      let input_passcode = input_passcode_header.to_str()?;
      if input_passcode == passcode {
        return Ok(
          Response::builder()
            .status(StatusCode::UNAUTHORIZED)
            .header(SET_COOKIE, build_cookie(passcode))
            .body("yes".to_string())?
            .into_response(),
        );
      } else {
        return Ok(
          Response::builder()
            .status(StatusCode::UNAUTHORIZED)
            .body("no".to_string())?
            .into_response(),
        );
      }
    }
  };

  Ok((StatusCode::UNAUTHORIZED, AUTH_PAGE).into_response())
}

#[tokio::main]
async fn main() {
  let app = Router::new().route("/:passcode", get(auth));

  let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
  axum::serve(listener, app)
    .with_graceful_shutdown(shutdown_signal())
    .await
    .unwrap();
}

async fn shutdown_signal() {
  let ctrl_c = async {
    signal::ctrl_c()
      .await
      .context("Failed to install Ctrl+C handler.")
      .unwrap();
  };

  #[cfg(unix)]
  let terminate = async {
    signal::unix::signal(signal::unix::SignalKind::terminate())
      .context("Failed to install Ctrl+C handler.")
      .unwrap()
      .recv()
      .await;
  };

  #[cfg(not(unix))]
  let terminate = std::future::pending::<()>();

  tokio::select! {
      _ = ctrl_c => {},
      _ = terminate => {},
  }
}
