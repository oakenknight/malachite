use crate::handle::vote::on_vote;
use crate::input::RequestId;
use crate::prelude::*;
#[cfg(not(feature = "std"))]
use crate::types::Metrics;

pub async fn on_vote_set_request<Ctx>(
    co: &Co<Ctx>,
    state: &mut State<Ctx>,
    _metrics: Option<&Metrics>,
    request_id: RequestId,
    height: Ctx::Height,
    round: Round,
) -> Result<(), Error<Ctx>>
where
    Ctx: Context,
{
    debug!(%height, %round, %request_id, "Received vote set request, retrieve the votes and send response if set is not empty");

    let votes = state.restore_votes(height, round);

    if !votes.is_empty() {
        let vote_set = VoteSet::new(votes);

        perform!(
            co,
            Effect::SendVoteSetResponse(request_id, height, round, vote_set, Default::default())
        );
    }

    Ok(())
}

pub async fn on_vote_set_response<Ctx>(
    co: &Co<Ctx>,
    state: &mut State<Ctx>,
    metrics: Option<&Metrics>,
    response: VoteSet<Ctx>,
) -> Result<(), Error<Ctx>>
where
    Ctx: Context,
{
    debug!(
        height = %state.height(), round = %state.round(), votes.count = %response.len(),
        "Received vote set response"
    );

    for vote in response.votes {
        let _ = on_vote(co, state, metrics, vote).await;
    }

    Ok(())
}
