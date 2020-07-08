use std::collections::HashMap;

pub fn tally(match_results: &str) -> String {
    let mut teams: HashMap<String, TeamStats> = HashMap::new();
    for x in match_results.lines() {
        let (team1_name, team2_name, result) = get_match_result(x);
        add_stats(team1_name.to_string(), get_stats(result), &mut teams);
        add_stats(team2_name.to_string(), get_stats(result.for_opposition()), &mut teams);
    }
    let mut team_array: Vec<&TeamStats> = teams.iter().map(|(_name, team)| team).collect();
    get_tally(&mut team_array)
}

fn add_stats(team_name: String, stats: Stats, teams: &mut HashMap<String, TeamStats>) {
    match teams.get_mut(&team_name) {
        None => {
            let stats = Stats::default().add(stats);
            let team_stats = TeamStats::new(team_name.clone(), Stats::default().add(stats));
            teams.insert(team_name.clone(), team_stats);
        }
        Some(team_stats) => {
            team_stats.stats = team_stats.stats.add(stats);
        }
    }
}

fn get_match_result(match_str: &str) -> (&str, &str, MatchResult) {
    let a: Vec<&str> = match_str.split(';').collect();
    (a[0], a[1], MatchResult::from(a[2]))
}

fn get_stats(result: MatchResult) -> Stats {
    match result {
        MatchResult::Win => Stats {
            mp: 1,
            win: 1,
            l: 0,
            d: 0,
            p: 3,
        },
        MatchResult::Loss => Stats {
            mp: 1,
            win: 0,
            l: 1,
            d: 0,
            p: 0,
        },
        MatchResult::Draw => Stats {
            mp: 1,
            win: 0,
            l: 0,
            d: 1,
            p: 1,
        },
    }
}

#[derive(Default, Clone, Debug)]
pub struct TeamStats {
    name: String,
    stats: Stats,
}

impl TeamStats {
    fn new(name: String, stats: Stats) -> Self {
        TeamStats { name, stats }
    }
}

#[derive(Default, Clone, Copy, Debug)]
struct Stats {
    mp: i8,
    win: i8,
    l: i8,
    d: i8,
    p: i8,
}

impl Stats {
    fn add(mut self, rhs: Self) -> Self {
        self.mp += rhs.mp;
        self.win += rhs.win;
        self.l += rhs.l;
        self.d += rhs.d;
        self.p += rhs.p;
        self
    }
}

#[derive(Copy, Clone)]
enum MatchResult {
    Win,
    Draw,
    Loss,
}

impl From<&str> for MatchResult {
    fn from(result_str: &str) -> Self {
        match result_str {
            "win" => MatchResult::Win,
            "loss" => MatchResult::Loss,
            "draw" => MatchResult::Draw,
            _ => unreachable!(),
        }
    }
}

impl MatchResult {
    fn for_opposition(self) -> MatchResult {
        match self {
            MatchResult::Win => MatchResult::Loss,
            MatchResult::Loss => MatchResult::Win,
            MatchResult::Draw => MatchResult::Draw,
        }
    }
}

pub fn get_tally(teams: &mut [&TeamStats]) -> String {
    let mut tally = "Team                           | MP |  W |  D |  L |  P".to_string();
    teams.sort_by(|a, b| b.stats.p.cmp(&a.stats.p).then(a.name.cmp(&b.name)));
    for team in teams.iter() {
        tally += get_team_tally_string(team).as_str()
    }
    tally
}
pub fn get_team_tally_string(team: &TeamStats) -> String {
    format!(
        "\n{:<width$}|  {} |  {} |  {} |  {} |  {}",
        team.name,
        team.stats.mp,
        team.stats.win,
        team.stats.d,
        team.stats.l,
        team.stats.p,
        width = 31
    )
}
