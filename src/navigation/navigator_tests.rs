use crate::{
    discovery::DiscoveryCandidate,
    navigation::picker::{ConfidenceRules, SelectionStrategy, confident_pick, select_index},
    ranking::RankedCandidate,
};

fn rc(score: f32) -> RankedCandidate {
    RankedCandidate {
        candidate: DiscoveryCandidate::default(),
        ranked_score: score,
    }
}

#[test]
fn manual_valid_choice() {
    let candidates = vec![rc(0.0), rc(0.0)];

    let index = select_index(&candidates, SelectionStrategy::Manual { choice: 2 });

    assert_eq!(index, Some(1));
}

#[test]
fn manual_out_of_bounds() {
    let candidates = vec![rc(0.0)];

    let index = select_index(&candidates, SelectionStrategy::Manual { choice: 5 });

    assert_eq!(index, None);
}

#[test]
fn confident_success() {
    let candidates = vec![rc(0.9), rc(0.5)];

    let rules = ConfidenceRules {
        min_ranked_score: 0.8,
        min_score: 0.0,
    };

    let index = select_index(&candidates, SelectionStrategy::Confident { rules });

    assert_eq!(index, Some(0));
}

#[test]
fn confident_pick_returns_top_candidate() {
    let candidates = vec![rc(0.9), rc(0.5)];

    let rules = ConfidenceRules {
        min_ranked_score: 0.8,
        min_score: 0.0,
    };

    let result = confident_pick(&candidates, rules);

    assert!(result.is_some());
    assert_eq!(result.unwrap().ranked_score, 0.9);
}
