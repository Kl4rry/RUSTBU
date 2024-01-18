#![allow(unused)]
use rocket::{*, response::content::*, http::{hyper::Response, Cookie}};
use crate::rules::{game_board::{Color, Tile}, game_state::{Game, GameHodler}};

//Skapa en ny game lobby med en random url
//t.ex. rsIa8ZVuA
//Och redirectar direkt till den.
#[get("/new")]
pub fn new_game_instance<'r>(shared: &State<GameHodler>) -> response::Redirect
{
    let g = Game::new_game();
    let mut games_map = shared.games.lock().expect("FAILED TO LOCk");

    let url = Game::generate_url();
    games_map.insert(url.to_owned(), g);

    return response::Redirect::to(format!("/api/game/{}", url));
}

//Fetcha en gamestate med associated URL
#[get("/state/<url>")]
pub fn get_game_instance<'r>(url: String, shared: &State<GameHodler>) -> String
{
    let games_map = shared.games.lock().expect("FAILED TO LOCk");
    let g: &Game = games_map.get(&url).unwrap();
    return format!("{:#?}", g);
}

#[post("/make_move/<url>/<p>/<a>")]
pub fn make_move(url: String, p: String, a: String, shared: &State<GameHodler>) -> RawJson<&'static str>
{
    if parse_move(&url, &p, &shared) && parse_move(&url, &a, &shared)
    {
        return RawJson("we good");
    }
    let mut board = shared.games.lock().expect("FAILED TO LOCk").get(&url).unwrap().get_boards();
    todo!();

    RawJson("true")
}

pub fn parse_move(url: &String, m: &String, shared: &&State<GameHodler>) -> bool
{
    let list: Vec<char> = m.chars().collect();
    if list.len() != 7
    {
        return false;
    }
    /*
    movestringen ser ut såhär:

    BW1131A

    0: Home_size : B
    1: Board_Colour : W

    2: POS X1 : 1
    3: POS Y1 : 1

    4: POS x2 : 3
    5: POS Y2 : 3

    6: MOVETYPE: A(ggressive)

     */

    let homeside = match list[0]
    {
        b => Color::Black,
        w => Color::White,
        _=> return false,
    };

    let colour = match list[0]
    {
        b => Color::Black,
        w => Color::White,
        _=> return false,
    };

    let game = shared.games.lock().expect("Failed to lock in parse moves");
    let board = game.get(url).unwrap().get_board(homeside, colour).unwrap();

    let x1 = list[2] as i8;
    let y1 = list[3] as i8;
    let x2 = list[4] as i8;
    let y2 = list[5] as i8;

    let delta_x = (x2 - x1).abs();
    let delta_y = (y2 - y1).abs();

    let aggr = match list[6]
    {
        a => true,
        p => false,
        _=> return false,
    };

    //Det här är så crummy.
    return Tile::is_valid(board.get_state(), (x1, y1), (x2, y2), &delta_x.max(delta_y), aggr, (&delta_x, &delta_y));

}

//Kolla vilken spelare du är i ett game, om rollen är tom, blir du den rollen.
//I guess used for debugging
#[get("/whoami/<url>")]
pub fn who_am_i(url: String, shared: &State<GameHodler>) -> String
{
    let (mut b, mut w) = shared.games.lock().expect("Idk who you are").get(&url).unwrap().get_players();
    //Check black player
    match b
    {
        Some(p) => (),
        None =>
        {
            b = Some(String::new());
            return format!("You are Black, my condolences.");
        } 
    }

    //Check white player
    match w
    {
        Some(p) => (),
        None =>
        {
            w = Some(String::new());
            return format!("You are White");
        } 
    }
    return String::new();
}