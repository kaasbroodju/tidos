use tidos::{scoped_css, view, Component, Page};

pub struct PlayerRow {
    pub rank: usize,
    pub name: String,
    pub score: u32,
    pub online: bool,
}

impl Component for PlayerRow {
    fn to_render(&self, page: &mut Page) {
        view! {
            <tr class={scoped_css!("./player_row.css")}>
                <td>
                    {#match self.rank}
                        {:case 1}
                            {"🥇"}
                        {:case 2}
                            {"🥈"}
                        {:case 3}
                            {"🥉"}
                        {:case _}
                            <span>{"#{}", self.rank}</span>
                    {/match}
                </td>
                <td>
                    <span>{&self.name}</span>
                    {#if self.online}
                        <span data-status="online">{"online"}</span>
                    {:else}
                        <span data-status="offline">{"offline"}</span>
                    {/if}
                </td>
                <td>
                    {self.score.to_string()}
                </td>
                <td>
                    {#if self.score >= 9000}
                        <span data-tier="legendary">{"LEGENDARY"}</span>
                    {:else if self.score >= 5000}
                        <span data-tier="expert">{"EXPERT"}</span>
                    {:else if self.score >= 3000}
                        <span data-tier="pro">{"PRO"}</span>
                    {:else}
                        <span data-tier="newbie">{"NEWBIE"}</span>
                    {/if}
                </td>
            </tr>
        }
    }
}
