use axum::body::{Body, Bytes};
use axum::http::{HeaderValue, header};
use axum::response::Response;
use coco_protocol::{CocoError, CocoResult, ResponseMeta, ResponseStatus, SearchHit};
use futures::stream;

struct StreamState {
    head: Option<Bytes>,
    results: std::vec::IntoIter<SearchHit>,
    first: bool,
    tail_sent: bool,
}

pub(crate) fn stream_results(
    results: Vec<SearchHit>,
    status: ResponseStatus,
) -> CocoResult<Response> {
    ensure_serializable_hits(&results)?;
    let meta = ResponseMeta { status };
    let meta_json = serde_json::to_vec(&meta).map_err(CocoError::system)?;
    let mut head = Vec::with_capacity(meta_json.len() + 24);
    head.extend_from_slice(b"{\"meta\":");
    head.extend_from_slice(&meta_json);
    head.extend_from_slice(b",\"data\":{\"results\":[");
    let state = StreamState {
        head: Some(Bytes::from(head)),
        results: results.into_iter(),
        first: true,
        tail_sent: false,
    };
    let stream = stream::unfold(state, |mut state| async move {
        if let Some(head) = state.head.take() {
            return Some((Ok::<Bytes, std::convert::Infallible>(head), state));
        }
        if let Some(item) = state.results.next() {
            let payload = serde_json::to_vec(&item)
                .expect("search hit serialization should be infallible after validation");
            let bytes = if state.first {
                state.first = false;
                Bytes::from(payload)
            } else {
                let mut combined = Vec::with_capacity(payload.len() + 1);
                combined.push(b',');
                combined.extend_from_slice(&payload);
                Bytes::from(combined)
            };
            return Some((Ok::<Bytes, std::convert::Infallible>(bytes), state));
        }
        if !state.tail_sent {
            state.tail_sent = true;
            return Some((
                Ok::<Bytes, std::convert::Infallible>(Bytes::from_static(b"]}}")),
                state,
            ));
        }
        None
    });
    let body = Body::from_stream(stream);
    let mut response = Response::new(body);
    response.headers_mut().insert(
        header::CONTENT_TYPE,
        HeaderValue::from_static("application/json"),
    );
    Ok(response)
}

fn ensure_serializable_hits(results: &[SearchHit]) -> CocoResult<()> {
    for hit in results {
        if !hit.meta.score.is_finite() {
            return Err(CocoError::compute("search hit score must be finite"));
        }
        if let Some(quality) = hit.meta.quality {
            if !quality.is_finite() {
                return Err(CocoError::compute("search hit quality must be finite"));
            }
        }
        if let Some(score) = hit.chunk.quality_score {
            if !score.is_finite() {
                return Err(CocoError::compute("chunk quality_score must be finite"));
            }
        }
        if let Some(embedding) = hit.chunk.embedding.as_ref() {
            for value in embedding {
                if !value.is_finite() {
                    return Err(CocoError::compute("chunk embedding must be finite"));
                }
            }
        }
    }
    Ok(())
}
