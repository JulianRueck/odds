use crate::{
    discovery::DiscoveryCandidate,
    picker::{SelectionStrategy, confident_pick, select_index},
    ranking::{ConfidenceRules, RankedCandidate},
};

fn rc(score: f32) -> RankedCandidate {
    RankedCandidate {
        candidate: DiscoveryCandidate::default(),
        ml_score: score,
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
        min_score: 0.8,
        min_gap: 0.3,
    };

    let index = select_index(&candidates, SelectionStrategy::Confident { rules });

    assert_eq!(index, Some(0));
}

#[test]
fn confident_pick_returns_top_candidate() {
    let candidates = vec![rc(0.9), rc(0.5)];

    let rules = ConfidenceRules {
        min_score: 0.8,
        min_gap: 0.3,
    };

    let result = confident_pick(&candidates, rules);

    assert!(result.is_some());
    assert_eq!(result.unwrap().ml_score, 0.9);
}
