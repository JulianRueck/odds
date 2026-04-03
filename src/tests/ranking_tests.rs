use std::path::PathBuf;

use crate::{
    discovery::{DiscoveryCandidate, Matchkind},
    persistence::History,
    persistence::Session,
    ranking::ranker::{Ranker},
};

#[test]
fn rank_candidates_orders_by_match_kind_weight() {
    let history = History::default();
    let session = Session::default();
    let weights = Ranker::default();

    let exact = DiscoveryCandidate {
        path: PathBuf::new(),
        match_kind: Matchkind::Exact,
        score: 0.0,
    };

    let prefix = DiscoveryCandidate {
        path: PathBuf::new(),
        match_kind: Matchkind::Prefix,
        score: 0.0,
    };

    let ranked = Ranker::rank_candidates(vec![prefix, exact], &history, &session, &weights, 10);

    assert_eq!(ranked[0].match_kind, Matchkind::Exact);
}
