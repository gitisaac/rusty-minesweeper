use rand::Rng;
use yew::prelude::*;

struct Cell {
    is_mine: bool,
    is_revealed: bool,
    is_flagged: bool,
    adjacent_mines: u8,
}

pub struct GameBoard {
    cells: Vec<Cell>,
    size: u8,  // size of the board (size x size)
    num_mines: u8,
}

impl GameBoard {
    fn new(size: u8, num_mines: u8) -> Self {
        let mut cells = Vec::with_capacity((size * size) as usize);
        for _ in 0..(size * size) {
            cells.push(Cell {
                is_mine: false,
                is_revealed: false,
                is_flagged: false,
                adjacent_mines: 0,
            });
        }

        let mut rng = rand::thread_rng();
        for _ in 0..num_mines {
            let index = rng.gen_range(0..cells.len());
            cells[index].is_mine = true;
        }

        // Compute the adjacent mines
        for row in 0..size {
            for col in 0..size {
                let i = (row * size + col) as usize;
                let mut adjacent_mines = 0;

                // Check the row above
                if row > 0 {
                    if col > 0 && cells[i - size as usize - 1].is_mine {
                        adjacent_mines += 1;
                    }
                    if cells[i - size as usize].is_mine {
                        adjacent_mines += 1;
                    }
                    if col < size - 1 && cells[i - size as usize + 1].is_mine {
                        adjacent_mines += 1;
                    }
                }

                // Check the current row
                if col > 0 && cells[i - 1].is_mine {
                    adjacent_mines += 1;
                }
                if col < size - 1 && cells[i + 1].is_mine {
                    adjacent_mines += 1;
                }

                // Check the row below
                if row < size - 1 {
                    if col > 0 && cells[i + size as usize - 1].is_mine {
                        adjacent_mines += 1;
                    }
                    if cells[i + size as usize].is_mine {
                        adjacent_mines += 1;
                    }
                    if col < size - 1 && cells[i + size as usize + 1].is_mine {
                        adjacent_mines += 1;
                    }
                }

                cells[i].adjacent_mines = adjacent_mines;
            }
        }

        Self {
            cells,
            size,
            num_mines,
        }
    }
}


pub enum Msg {
    CellClick(usize, usize),
}

#[derive(Properties, Clone, PartialEq)]
pub struct GameBoardProps {
    pub rows: u8,
    pub cols: u8,
}

impl Default for GameBoardProps {
    fn default() -> Self {
        Self {
            rows: 0,
            cols: 0,
        }
    }
}


impl Component for GameBoard {
    type Message = Msg;
    type Properties = GameBoardProps;

    fn create(_ctx: &Context<Self>) -> Self {
        let props = _ctx.props();
        Self::new(props.rows, props.cols)
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::CellClick(x, y) => {
                self.cells[x * self.size as usize + y].is_revealed = true;
                true  // re-renders the component
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="grid">
                {
                    for (0..self.size).map(|row| {
                        html! {
                            <div class="row">
                                {
                                    for (0..self.size).map(|col| {
                                        let index = row * self.size + col;
                                        let cell = &self.cells[index as usize];
                                        let class = if cell.is_revealed {
                                            "revealed"
                                        } else {
                                            "hidden"
                                        };
                                        html! {
                                            <div class="cell">
                                                {" "}
                                            </div>
                                        }
                                    })
                                }
                            </div>
                        }
                    })
                }
            </div>
        }
    }
}

