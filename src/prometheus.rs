use lazy_static::lazy_static;
use prometheus::{Gauge, Opts, Registry};

lazy_static! {
    // Root

    pub static ref WINS_METRIC: Opts =
        Opts::new("team_wins", "Number of wins").variable_label("team_number");
    pub static ref RANK_METRIC: Opts =
        Opts::new("team_rank", "The rank of a specified team").variable_label("team_number");
    pub static ref LOSS_METRIC: Opts =
        Opts::new("team_loss", "Number of losses").variable_label("team_number");
    pub static ref QUAL_MATCHES_PLAYED_METRIC: Opts =
        Opts::new("team_matches_played", "Number of matches played").variable_label("team_number");

    // Avg

    pub static ref AVG_AUTO: Opts =
        Opts::new("team_average_auto", "Number of average points scored in auto").variable_label("team_number");
    pub static ref AVG_TOTAL: Opts =
        Opts::new("team_average_total", "Number of average total points scored per match").variable_label("team_number");

    // OPR

    pub static ref OPR: Opts =
        Opts::new("team_opr", "A given team's OPR").variable_label("team_number");
}

pub fn register_metrics() {
    let register = Registry::new();

    let wins_counter = Gauge::with_opts(WINS_METRIC.clone()).expect("error");
    let rank_counter = Gauge::with_opts(RANK_METRIC.clone()).expect("error");
    let loss_counter = Gauge::with_opts(LOSS_METRIC.clone()).expect("error");
    let qual_matches_played_counter =
        Gauge::with_opts(QUAL_MATCHES_PLAYED_METRIC.clone()).expect("error");

    let avg_auto_counter = Gauge::with_opts(AVG_AUTO.clone()).expect("error");
    let avg_total_counter = Gauge::with_opts(AVG_TOTAL.clone()).expect("error");

    let opr_counter = Gauge::with_opts(OPR.clone()).expect("error");

    let _ = register.register(Box::new(wins_counter));
    let _ = register.register(Box::new(rank_counter));
    let _ = register.register(Box::new(loss_counter));
    let _ = register.register(Box::new(qual_matches_played_counter));

    let _ = register.register(Box::new(avg_auto_counter));
    let _ = register.register(Box::new(avg_total_counter));

    let _ = register.register(Box::new(opr_counter));
}

#[macro_export]
macro_rules! set_gauge_counter {
    ($gauge:expr, $input:expr) => {{
        $gauge.set($input);
    }};
}
