// サッカーの試合のスコアをリスト型で取り扱います。
// それぞれの行では"<team_1_name>,<team_2_name>,<team_1_goals>,<team_2_goals>"という形式を取ります。
// チームの名前と取得点、失点の合計点を格納するスコアテーブルを作る必要があります。

use std::collections::HashMap;

// チームの点数の詳細を格納する構造体。
#[derive(Default)]
struct TeamScores {
    goals_scored: u8,
    goals_conceded: u8,
}

fn build_scores_table(results: &str) -> HashMap<&str, TeamScores> {
    // チームの名前はキー、構造体はバリューに格納します。
    let mut scores: HashMap<&str, TeamScores> = HashMap::new();

    for line in results.lines() {
        let mut split_iterator = line.split(',');
        // NOTE: エラーハンドリングについてまだ取り扱っていないので、一旦`unwrap`を使います。
        let team_1_name = split_iterator.next().unwrap();
        let team_2_name = split_iterator.next().unwrap();
        let team_1_score: u8 = split_iterator.next().unwrap().parse().unwrap();
        let team_2_score: u8 = split_iterator.next().unwrap().parse().unwrap();

        // TODO: スコアテーブルに引数resultsから取得したデータを格納しましょう。
        // チーム1の取得点はチーム2の失点であることに留意しましょう。
        let team_1_scores = TeamScores {
            goals_scored: team_1_score + scores.get(&team_1_name).unwrap_or(&TeamScores { goals_scored:0, goals_conceded: 0 }).goals_scored,
            goals_conceded: team_2_score + scores.get(&team_1_name).unwrap_or(&TeamScores { goals_scored:0, goals_conceded: 0 }).goals_conceded
        };
        let team_2_scores = TeamScores {
            goals_scored: team_2_score + scores.get(&team_2_name).unwrap_or(&TeamScores { goals_scored:0, goals_conceded: 0 }).goals_scored,
            goals_conceded: team_1_score + scores.get(&team_2_name).unwrap_or(&TeamScores { goals_scored:0, goals_conceded: 0 }).goals_conceded
        };
        scores.insert(team_1_name, team_1_scores);
        scores.insert(team_2_name, team_2_scores);
    }

    scores
}

fn main() {
    // この行で関数のテストができます。
}

#[cfg(test)]
mod tests {
    use super::*;

    const RESULTS: &str = "England,France,4,2
France,Italy,3,1
Poland,Spain,2,0
Germany,England,2,1
England,Spain,1,0";

    #[test]
    fn build_scores() {
        let scores = build_scores_table(RESULTS);

        assert!(["England", "France", "Germany", "Italy", "Poland", "Spain"]
            .into_iter()
            .all(|team_name| scores.contains_key(team_name)));
    }

    #[test]
    fn validate_team_score_1() {
        let scores = build_scores_table(RESULTS);
        let team = scores.get("England").unwrap();
        assert_eq!(team.goals_scored, 6);
        assert_eq!(team.goals_conceded, 4);
    }

    #[test]
    fn validate_team_score_2() {
        let scores = build_scores_table(RESULTS);
        let team = scores.get("Spain").unwrap();
        assert_eq!(team.goals_scored, 0);
        assert_eq!(team.goals_conceded, 3);
    }
}
