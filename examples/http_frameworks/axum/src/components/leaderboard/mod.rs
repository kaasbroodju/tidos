use tidos::{scoped_css, view, Component, Page};

use crate::components::player_row::PlayerRow;

pub struct Player {
    pub name: String,
    pub score: u32,
    pub online: bool,
}

pub struct Leaderboard;

impl Component for Leaderboard {
    fn to_render(&self, page: &mut Page) -> String {
        let players = vec![
            Player { name: "ferris_the_crab".into(), score: 9999, online: true },
            Player { name: "rustacean42".into(), score: 8420, online: true },
            Player { name: "borrow_checker".into(), score: 7350, online: false },
            Player { name: "unsafe_wizard".into(), score: 5100, online: false },
            Player { name: "lifetime_larry".into(), score: 3200, online: true },
            Player { name: "newbie_carl".into(), score: 120, online: false },
        ];

        let headers = vec![
            String::from("Rank"),
            String::from("Player"),
            String::from("Score"),
            String::from("Tier"),
        ];
        let online_count = players.iter().filter(|p| p.online).count();
        let total = players.len();

        view! {
            <section class={scoped_css!("./leaderboard.css")}>
                <h2>{"🏆 Leaderboard"}</h2>
                <p>
                    {#if online_count == 0}
                        No players online right now.
                    {:else if online_count == 1}
                        {format!("1 of {} players currently online.", total)}
                    {:else}
                        {format!("{} of {} players currently online.", online_count, total)}
                    {/if}
                </p>
                <LeaderboardTable headers={headers}>
                    {#slot:body}
                        {#for (i, player) in players.iter().enumerate()}
                            <PlayerRow
                                rank={i + 1}
                                name={player.name.clone()}
                                score={player.score}
                                online={player.online}
                            />
                        {/for}
                    {/slot}
                </LeaderboardTable>
            </section>
        }
    }
}

pub struct LeaderboardTable {
    pub headers: Vec<String>,
    pub body: String,
}

impl Component for LeaderboardTable {
    fn to_render(&self, _page: &mut Page) -> String {
        view! {
            <table>
                <thead>
                    <tr>
                        {#for title in &self.headers}
                            <th>{title}</th>
                        {/for}
                    </tr>
                </thead>
                <tbody>
                    @html{&self.body}
                </tbody>
            </table>
        }
    }
}
