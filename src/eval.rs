use serde::Serialize;

/// Direction of deviation from an optimal band.
#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Direction {
    Under,
    Within,
    Over,
}

/// The type of an eval and its scoring parameters.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum EvalType {
    /// Pass/fail: true → 1.0, false → 0.0
    Boolean,
    /// Likert scale mapped to [0.0, 1.0]
    Ordinal { min: u32, max: u32 },
    /// Raw ratio in [0.0, 1.0]
    Continuous,
    /// Optimal band with directional deviation
    Range { low: f64, high: f64 },
}

/// Result of scoring an eval.
#[derive(Debug, Clone, Serialize)]
pub struct EvalResult {
    pub name: String,
    pub eval_type: EvalType,
    pub score: f64,
    pub deviation: f64,
    pub direction: Direction,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw_value: Option<f64>,
}

impl EvalType {
    /// Score a raw value according to this eval type.
    pub fn score(&self, value: f64) -> EvalResult {
        match self {
            EvalType::Boolean => {
                let score = if value >= 0.5 { 1.0 } else { 0.0 };
                EvalResult {
                    name: String::new(),
                    eval_type: self.clone(),
                    score,
                    deviation: 0.0,
                    direction: if score >= 1.0 {
                        Direction::Within
                    } else {
                        Direction::Under
                    },
                    raw_value: Some(value),
                }
            }
            EvalType::Ordinal { min, max } => {
                let min_f = *min as f64;
                let max_f = *max as f64;
                let range = max_f - min_f;
                let score = if range > 0.0 {
                    ((value - min_f) / range).clamp(0.0, 1.0)
                } else {
                    1.0
                };
                EvalResult {
                    name: String::new(),
                    eval_type: self.clone(),
                    score,
                    deviation: 0.0,
                    direction: Direction::Within,
                    raw_value: Some(value),
                }
            }
            EvalType::Continuous => {
                let score = value.clamp(0.0, 1.0);
                EvalResult {
                    name: String::new(),
                    eval_type: self.clone(),
                    score,
                    deviation: 0.0,
                    direction: Direction::Within,
                    raw_value: Some(value),
                }
            }
            EvalType::Range { low, high } => {
                let center = (low + high) / 2.0;
                let half_band = (high - low) / 2.0;

                let (score, deviation, direction) = if value >= *low && value <= *high {
                    (1.0, 0.0, Direction::Within)
                } else if value < *low {
                    let dist = low - value;
                    let s = if half_band > 0.0 {
                        (1.0 - dist / (center.max(half_band))).max(0.0)
                    } else {
                        0.0
                    };
                    (s, -dist, Direction::Under)
                } else {
                    let dist = value - high;
                    let s = if half_band > 0.0 {
                        (1.0 - dist / (center.max(half_band))).max(0.0)
                    } else {
                        0.0
                    };
                    (s, dist, Direction::Over)
                };

                EvalResult {
                    name: String::new(),
                    eval_type: self.clone(),
                    score,
                    deviation,
                    direction,
                    raw_value: Some(value),
                }
            }
        }
    }
}

/// Parse an eval type annotation from bracket syntax: `[boolean]`, `[range: 5..10]`, etc.
pub fn parse_eval_type(annotation: &str) -> Option<EvalType> {
    let inner = annotation.trim();
    if inner.eq_ignore_ascii_case("boolean") {
        return Some(EvalType::Boolean);
    }
    if inner.eq_ignore_ascii_case("continuous") {
        return Some(EvalType::Continuous);
    }
    if let Some(rest) = inner.strip_prefix("ordinal") {
        let rest = rest.trim().strip_prefix(':').unwrap_or(rest.trim()).trim();
        if let Some((min_s, max_s)) = rest.split_once("..")
            && let (Ok(min), Ok(max)) = (min_s.trim().parse(), max_s.trim().parse())
        {
            return Some(EvalType::Ordinal { min, max });
        }
        return Some(EvalType::Ordinal { min: 1, max: 5 });
    }
    if let Some(rest) = inner.strip_prefix("range") {
        let rest = rest.trim().strip_prefix(':').unwrap_or(rest.trim()).trim();
        if let Some((low_s, high_s)) = rest.split_once("..")
            && let (Ok(low), Ok(high)) = (low_s.trim().parse(), high_s.trim().parse())
        {
            return Some(EvalType::Range { low, high });
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn boolean_scoring() {
        let eval = EvalType::Boolean;
        let result = eval.score(1.0);
        assert_eq!(result.score, 1.0);
        assert_eq!(result.direction, Direction::Within);

        let result = eval.score(0.0);
        assert_eq!(result.score, 0.0);
        assert_eq!(result.direction, Direction::Under);
    }

    #[test]
    fn ordinal_scoring() {
        let eval = EvalType::Ordinal { min: 1, max: 5 };
        let result = eval.score(3.0);
        assert_eq!(result.score, 0.5);

        let result = eval.score(5.0);
        assert_eq!(result.score, 1.0);

        let result = eval.score(1.0);
        assert_eq!(result.score, 0.0);
    }

    #[test]
    fn continuous_scoring() {
        let eval = EvalType::Continuous;
        let result = eval.score(0.75);
        assert_eq!(result.score, 0.75);

        let result = eval.score(1.5);
        assert_eq!(result.score, 1.0);
    }

    #[test]
    fn range_scoring_within() {
        let eval = EvalType::Range {
            low: 5.0,
            high: 10.0,
        };
        let result = eval.score(7.0);
        assert_eq!(result.score, 1.0);
        assert_eq!(result.deviation, 0.0);
        assert_eq!(result.direction, Direction::Within);
    }

    #[test]
    fn range_scoring_over() {
        let eval = EvalType::Range {
            low: 5.0,
            high: 10.0,
        };
        let result = eval.score(12.0);
        assert!(result.score < 1.0);
        assert!(result.deviation > 0.0);
        assert_eq!(result.direction, Direction::Over);
    }

    #[test]
    fn range_scoring_under() {
        let eval = EvalType::Range {
            low: 5.0,
            high: 10.0,
        };
        let result = eval.score(3.0);
        assert!(result.score < 1.0);
        assert!(result.deviation < 0.0);
        assert_eq!(result.direction, Direction::Under);
    }

    #[test]
    fn parse_eval_type_boolean() {
        assert_eq!(parse_eval_type("boolean"), Some(EvalType::Boolean));
        assert_eq!(parse_eval_type("Boolean"), Some(EvalType::Boolean));
    }

    #[test]
    fn parse_eval_type_continuous() {
        assert_eq!(parse_eval_type("continuous"), Some(EvalType::Continuous));
    }

    #[test]
    fn parse_eval_type_ordinal() {
        assert_eq!(
            parse_eval_type("ordinal: 1..5"),
            Some(EvalType::Ordinal { min: 1, max: 5 })
        );
        assert_eq!(
            parse_eval_type("ordinal"),
            Some(EvalType::Ordinal { min: 1, max: 5 })
        );
    }

    #[test]
    fn parse_eval_type_range() {
        assert_eq!(
            parse_eval_type("range: 5..10"),
            Some(EvalType::Range {
                low: 5.0,
                high: 10.0
            })
        );
    }
}
