use std::{cmp::Ord, f32};

use super::game_board::Board;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Tile {
    Empty,
    Black,
    White,
}

impl Tile {
    pub fn is_empty(t: Tile) -> bool {
        t == Tile::Empty
    }
    pub fn empty() -> Tile {
        return Tile::Empty;
    }

    pub fn white() -> Tile {
        return Tile::White;
    }

    pub fn black() -> Tile {
        return Tile::Black;
    }

    pub fn get_possible_moves(b: &Board, aggr: bool, cur_pos: (i8, i8)) -> Vec<(i8, i8)> {
        let boardstate = b.get_state();

        let mut movelist: Vec<(i8, i8)> = Vec::new();
        //0 = y, 1 = x
        let directions = [
            (0, -1),
            (0, 1),
            (-1, 0),
            (1, 0),
            (-1, -1),
            (-1, 1),
            (1, 1),
            (1, -1),
        ];
        //So a move to the left is [1, 0]

        for (dy, dx) in directions.iter() {
            for i in 1..3 as i8 {
                let new_pos = (cur_pos.0 + i * dy, cur_pos.1 + i * dx);

                if Tile::is_valid(boardstate, cur_pos, new_pos, &i, aggr, (&dy, &dx)) {
                    //println!("ADDED {} {}, DIRECTION: {} {}, DIFF: {} {}", new_pos.0, new_pos.1, dy, dx, (cur_pos.0 - new_pos.0).abs(), (cur_pos.1 - new_pos.1).abs());
                    movelist.push((new_pos.0, new_pos.1)); //this is so crummy.
                    continue;
                }
                break;
            }
        }
        return movelist;
    }

    pub fn is_valid(
        state: &[[Tile; 4]; 4],
        cur_pos: (i8, i8),
        new_pos: (i8, i8),
        i: &i8,
        aggr: bool,
        (dy, dx): (&i8, &i8),
    ) -> bool {
        //Check if in range.
        let newy = new_pos.0 as usize;
        let newx = new_pos.1 as usize;

        let stepy = (cur_pos.0 + dy * 1) as usize;
        let stepx = (cur_pos.1 + dx * 1) as usize;

        //println!("newy: {:?}\nnewx: {:?}\ndy: {}\ndx: {}\ni: {}", newy, newx, dy, dx, i);

        //println!("cur_pos: {:?}\nnew_pos: {:?}\nstepy: {}\nstepx: {}", cur_pos, new_pos, stepy, stepx);

        //If outta range
        if newx > 3 || newy > 3 || stepy > 3 || stepx > 3 {
            //println!("\n\nMove is out of range. \nCurpos: {:?}\nNewpos: {:?}\nNewx: {}\nNewy: {}\nStepx: {}\nStepy: {}\ndy: {}\ndx: {}\n\n", cur_pos, new_pos, newx, newy, stepx, stepy, dy, dx);
            return false;
        }

        //Passive
        if !aggr {
            if state[newy][newx] != Tile::Empty {
                println!("You tried to passive into a mf.");
                return false;
            }
        }

        if aggr
        //Det här är gigacrummy. TODO: Make less crummy.
        {
            //Knuffa ej våra egna stenar.
            if state[newy][newx] == state[cur_pos.0 as usize][cur_pos.1 as usize] {
                println!("Cannot push own rocks.");
                return false;
            }

            if *i == 1 && state[newy][newx] != Tile::Empty {
                //In case stenen vi puttar faller av boarden.
                if (cur_pos.0 + 2 * dy) > 3
                    || (cur_pos.1 + 2 * dx) > 3
                    || (cur_pos.0 + 2 * dy) < 0
                    || (cur_pos.1 + 2 * dx) < 0
                {
                    return true;
                }
                //Checka om det finns en sten bakom stenen vi puttar.
                //println!("(cur_pos.0 + 2 * dy) = {}\n(cur_pos.1 + 2 * dx) = {}\n(cur_pos.0 + 2 * dy) = {}\n(cur_pos.1 + 2 * dx) = {}", (cur_pos.0 + 2 * dy), (cur_pos.1 + 2 * dx), (cur_pos.0 + 2 * dy), (cur_pos.1 + 2 * dx));
                //println!("No rock is falling off. Returns true if valid: {}\nwtf is there: {:?}", (state[(cur_pos.0 + 2 * dy) as usize][(cur_pos.1 + 2 * dx) as usize] == Tile::Empty), state[(cur_pos.0 + 2 * dy) as usize][(cur_pos.1 + 2 * dx) as usize]);
                return state[(cur_pos.0 + 2 * dy) as usize][(cur_pos.1 + 2 * dx) as usize]
                    == Tile::Empty;
            } else if *i == 2 && state[stepy][stepx] != Tile::Empty {
                //Checka om det finns en sten bakom stenen vi puttar.
                //println!("Rock 1 step ahead: {:?}\nRock 2 step ahead: {:?}", state[stepx][stepy], state[newx][newy]);
                return state[newy][newx] == Tile::Empty;
            }
        }
        return true;
    }

    pub fn passive_move(b: &mut Board, cur_pos: (i8, i8), new_pos: (i8, i8)) -> bool {
        //TODO: get_possible_moves borde inte finnas.
        /* if !Tile::get_possible_moves(b, false, cur_pos).contains(&new_pos)
        {
            return false
        }*/

        let dx = new_pos.1 - cur_pos.1;
        let dy = new_pos.0 - cur_pos.0;

        //i = antal steps, 1 eller 2
        let i = dy.abs().max(dx.abs());

        if !Tile::is_valid(b.get_state(), cur_pos, new_pos, &i, false, (&dy, &dx)) {
            return false;
        }

        if dx == 0 && dy == 0 {
            return false;
        }

        let mut boardstate = *b.get_state();
        /*
        Detta kommer ge typ:
        [W][B][ ][B]
        [ ][W][ ][ ]
        [ ][ ][ ][ ]
        [ ][w][B][W]

        Så vi tar cur_pos och flyttar till new_pos
        */
        let rock_me = boardstate[cur_pos.0 as usize][cur_pos.1 as usize];

        //Old space is empty
        boardstate[cur_pos.0 as usize][cur_pos.1 as usize] = Tile::empty();

        //New space has the rock
        boardstate[new_pos.0 as usize][new_pos.1 as usize] = rock_me;

        b.set_state(&boardstate);

        //Får ut storleken flyttad så vi kan slänga in den i aggr.
        //let sizediff = ((cur_pos.0 - new_pos.0).abs(), (cur_pos.1 - new_pos.1).abs());
        return true;
    }

    pub fn aggressive_move(b: &mut Board, cur_pos: (i8, i8), new_pos: (i8, i8)) -> bool {
        if Tile::is_empty(b.to_owned().get_state()[cur_pos.0 as usize][cur_pos.1 as usize]) {
            panic!("wtf")
        }

        //TODO: get_possible_moves borde inte finnas.
        /*if !Tile::get_possible_moves(b, true, cur_pos).contains(&new_pos)
        {
            return false;
        }*/

        let dx = new_pos.1 - cur_pos.1;
        let dy = new_pos.0 - cur_pos.0;

        let dir = (
            (dy as f32 / 2.0).round() as i8,
            (dx as f32 / 2.0).round() as i8,
        );

        //i = antal steps, 1 eller 2
        let i = dy.abs().max(dx.abs());

        if !Tile::is_valid(b.get_state(), cur_pos, new_pos, &i, true, (&dir.0, &dir.1)) {
            println!("not valid.");
            return false;
        }

        let mut boardstate = *b.get_state();

        //let dir = ((diff.0 as f32 / 2.0).ceil() as i8, (diff.1 as f32 / 2.0).ceil() as i8);
        let dir = (
            (dy as f32 / 2.0).round() as i8,
            (dx as f32 / 2.0).round() as i8,
        );
        println!("{:?}", dir);
        //Linear size of diff
        let size = i;

        /*
        Detta kommer ge typ:
        [ ][ ][ ][ ]      [ ][ ][ ][ ]      [ ][ ][ ][ ]
        [ ][ ][ ][B]      [ ][ ][ ][W]      [ ][ ][ ][W]
        [ ][ ][ ][ ]  =>  [ ][ ][ ][ ]  =>  [ ][ ][ ][ ]
        [ ][w][ ][ ]      [ ][ ][ ][ ]      [ ][ ][ ][ ]

        [ ][ ][ ][ ]      [ ][ ][ ][ ]      [ ][B][ ][ ]
        [ ][ ][ ][ ]      [ ][W][ ][ ]      [ ][W][ ][ ]
        [ ][B][ ][ ]  =>  [ ][B][ ][ ]  =>  [ ][ ][ ][ ]
        [ ][w][ ][ ]      [ ][ ][ ][ ]      [ ][ ][ ][ ]

        [ ][W][B][ ]      [ ][ ][B][W]      [ ][ ][ ][W]
        [ ][ ][ ][ ]      [ ][ ][ ][ ]      [ ][ ][ ][ ]
        [ ][ ][ ][ ]  =>  [ ][ ][ ][ ]  =>  [ ][ ][ ][ ]
        [ ][ ][ ][ ]      [ ][ ][ ][ ]      [ ][ ][ ][ ]

        Hopefully
        */

        let push_pos: (usize, usize) = (
            (new_pos.0 + 1 * dir.0) as usize,
            (new_pos.1 + 1 * dir.1) as usize,
        );
        let still_on_board = Tile::is_valid(
            b.get_state(),
            new_pos,
            (push_pos.0 as i8, push_pos.1 as i8),
            &size,
            true,
            (&dir.0, &dir.1),
        );

        //Om den puttade stenen fortfarande är på brädet.
        if still_on_board {
            //Ta stenen
            let rocky = boardstate[(new_pos.0 + (size - 1) * dir.0) as usize]
                [(new_pos.1 + (size - 1) * dir.1) as usize];
            //Sätt nästa position till den stenen
            boardstate[push_pos.0][push_pos.1] = rocky;
        }

        //Sätt nya posen till vår färg
        boardstate[new_pos.0 as usize][new_pos.1 as usize] =
            boardstate[cur_pos.0 as usize][cur_pos.1 as usize];

        //Om vi hoppar framåt 2 steg, rensa platsen 1 steg bakom oss. (D'Lcrantz metoden)
        if size == 2 {
            boardstate[(new_pos.0 - 1 * dir.0) as usize][(new_pos.1 - 1 * dir.1) as usize] =
                Tile::empty();
        }

        //Rensa förra platsen.
        boardstate[cur_pos.0 as usize][cur_pos.1 as usize] = Tile::empty();

        //Uppdatera boardstate.
        b.set_state(&boardstate);

        return true;
    }
}
