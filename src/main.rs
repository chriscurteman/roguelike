use tcod::colors::*;
use tcod::console::*;

// size of the window
const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;

// size of the map
const MAP_WIDTH: i32 = 80;
const MAP_HEIGHT: i32 = 45;

// tile colors
const COLOR_DARk_WALL: Color = Color { 
    r: 0, 
    g: 0, 
    b: 100,
};
const COLOR_DARK_GROUND: Color = Color {
    r: 50,
    g: 50,
    b: 150,
};

// 20 frames-per-second maximum
const LIMIT_FPS: i32 = 20; 

type Map = Vec<Vec<Tile>>;

struct Game {
    map: Map,
}

#[derive(Clone, Copy, Debug)]
struct Tile {
    blocked: bool,
    block_sight: bool,
}

impl Tile {
    pub fn empty() -> Self {
        Tile { 
            blocked: false, 
            block_sight: false 
        }
    }

    pub fn wall() -> Self {
        Tile { 
            blocked: true, 
            block_sight: true 
        }
    }
}

struct Tcod {
    root: Root,
    con: Offscreen,
}

// Generic Entity
/// contains entity metadata
#[derive(Debug)]
struct Entity {
    x: i32,
    y: i32,
    char: char,
    color: Color,
}

// contains the code for entity functions
impl Entity {
    pub fn new(
        x: i32, 
        y: i32, 
        char: char, 
        color: Color
    ) -> Self {
        Entity { x, y, char, color }
    }

    /// move by the given input amount
    pub fn move_by(
        &mut self, 
        dx: i32, 
        dy: i32,
        game: &Game
    ) {
        if !game.map[(self.x + dx) as usize][(self.y + dy) as usize].blocked {
            self.x += dx;
            self.y += dy;
        }
    }

    /// set the color and draw the ascii character
    pub fn draw(
        &self, 
        con: &mut dyn Console
    ) {
        con.set_default_foreground(self.color);
        con.put_char(self.x, self.y, self.char, BackgroundFlag::None);
    }
}

fn make_map() -> Map {
    // fill the map with unblocked tiles
    let mut map = vec![vec![
        Tile::empty(); 
        MAP_HEIGHT as usize]; 
        MAP_WIDTH as usize];

        map[30][22] = Tile::wall();
        map[50][22] = Tile::wall();

        map
}

fn render_all(tcod: &mut Tcod, game: &Game, entities: &[Entity])
{
    // draw all entities in the list
    for entity in entities {
        entity.draw(&mut tcod.con);
    }

    // go through all tiles and set their background color
    for y in 0..MAP_HEIGHT {
        for x in 0..MAP_WIDTH {
            let wall = game.map[x as usize][y as usize].block_sight;
            if wall {
                tcod.con
                    .set_char_background(
                        x, 
                        y, 
                        COLOR_DARk_WALL, 
                        BackgroundFlag::Set
                    );
            } else {
                tcod.con
                    .set_char_background(
                        x, 
                        y, 
                        COLOR_DARK_GROUND, 
                        BackgroundFlag::Set
                    );
            }
        }
    }

    // blit the contents of "con" to the root console
    blit(
        &tcod.con,
        (0, 0),
        (MAP_WIDTH, MAP_HEIGHT),
        &mut tcod.root,
        (0, 0),
        1.0,
        1.0,
    );
}

fn handle_keys(tcod: &mut Tcod, game: &Game, player: &mut Entity) -> bool {
    use tcod::input::Key;
    use tcod::input::KeyCode::*;

    let key = tcod.root.wait_for_keypress(true);
    match key {
        Key {
            code: Enter,
            alt: true,
            ..
        } => {
            // Alt+Enter: toggle fullscreen
            let fullscreen = tcod.root.is_fullscreen();
            tcod.root.set_fullscreen(!fullscreen);
        }
        Key { code: Escape, .. } => return true, // exit game

        // movement keys
        Key { code: Up, .. } => player.move_by(0, -1, game),
        Key { code: Down, .. } => player.move_by(0, 1, game),
        Key { code: Left, .. } => player.move_by(-1, 0, game),
        Key { code: Right, .. } => player.move_by(1, 0, game),

        _ => {}
    }

    false
}

fn main() {
    tcod::system::set_fps(
        LIMIT_FPS
    );

    let root = Root::initializer()
        .font(
            "arial10x10.png", 
            FontLayout::Tcod)
        .font_type(
            FontType::Greyscale
        )
        .size(
            SCREEN_WIDTH, 
            SCREEN_HEIGHT
        )
        .title(
            "Rust/libtcod tutorial"
        )
        .init();

    let con = Offscreen::new(
        MAP_WIDTH, 
        MAP_HEIGHT
    );

    let mut tcod = Tcod { 
        root, 
        con,
    };

    let player = Entity::new(
        SCREEN_WIDTH / 2, 
        SCREEN_HEIGHT / 2, 
        '@', 
        WHITE
    );

    let npc = Entity::new(
        SCREEN_WIDTH / 2 - 5,
        SCREEN_HEIGHT / 2,
        '@',
        YELLOW
    );

    let mut entities = [
        player,
        npc
    ];

    let game = Game {
        map: make_map(),
    };

    // Main Game Loop
    while !tcod.root.window_closed() {
        // clears the previous frame
        tcod.con.clear();

        for entity in &entities {
            entity.draw(
                &mut tcod.con
            );
        }

        // render the game with entities
        render_all(
            &mut tcod, 
            &game, 
            &entities
        );

        // draw everything on the console at the same time
        tcod.root.flush();

        // handle keys and exit game if needed
        let player = &mut entities[0];
        let exit = handle_keys(
            &mut tcod, 
            &game,
            player
        );
        if exit {
            break;
        }
    }
}