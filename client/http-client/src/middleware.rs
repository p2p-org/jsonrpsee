use crate::HeaderMap;

/// When attached to a [`HttpClientBuilder`] (generally using [`with`]), middleware is run
/// whenever the client issues a request, in the order it was attached.
#[async_trait::async_trait]
pub trait Middleware: 'static + Send + Sync {
	/// Invoked with a request before sending it.
	async fn handle(&self, headers: &mut HeaderMap);
}
