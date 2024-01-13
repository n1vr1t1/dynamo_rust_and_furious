use std::collections::HashMap;
use rand::Rng;
use robotics_lib::interface::{craft, destroy, Direction, discover_tiles, look_at_sky, put, teleport, Tools};
use robotics_lib::runner::Runnable;
use robotics_lib::utils::LibError;
use robotics_lib::world::environmental_conditions::EnvironmentalConditions;
use robotics_lib::world::tile::{Content,Tile};
use robotics_lib::world::World;

type TileMatrix=(Vec<Vec<Option<Tile>>>,(usize,usize));

/// Rizzler is a struct with 0 fields and 7 methods which are modified versions of the interfaces inside the common crate.
/// Each method charges a vector of Strings with irresistible pick up lines...
///
///
/// *Sigh ... what am i doing with my life?
///
///  Once the Vector is all charged up, the correspondent interface of the common crate is called and only after the execution of the same
/// a (terrible) random  pick up line is printed on the terminal! the methods return both the standard interface result and a sentence selected according to the result of the same.
///
/// Seriously, why did you buy this?


pub struct Rizzler{}
impl Tools for Rizzler{}
#[allow(unused_assignments)]
impl Rizzler {
    ///# Init_error_rizz()
    /// it initializes the string vector of errors
    ///
    ///  # Syntax
    /// init_error_rizz()->'Vec<String>':
    ///

    fn init_error_rizz()->Vec<String>{
        vec!["Are you a computer program? Because every time I'm around you, my heart encounters a delightful syntax error".to_string(),
             "Are you an algorithm? Because meeting you seems to be a perfect match without any errors".to_string(),
             "If you were a program, I'd hope there's no runtime error when we run it together".to_string(),
             "If we were code, I'd try to be your catch, hoping we never encounter any errors".to_string(),
             "Are you a debugger? Because meeting you feels like fixing all the errors in my day".to_string(),
             "If errors were flowers, you'd be the exception in my garden. *leans in for kiss".to_string(),
             "Can I debug you for a day? Because making you smile would be my favourite error correction".to_string(),
             "Immanuel Kant, but at least Immanuel tried".to_string(),
             "If making mistakes were a sport, I'd be a professional athlete. Luckily, I'm a quick learner, especially when it comes to you".to_string(),
             "They say everyone makes mistakes, but I think meeting you was my best decision yet".to_string(),
            "They say you learn from your mistakes, but meeting you feels more like a happy accident than a lesson learned".to_string(),
            "If life were a test, meeting you would be the best mistake I've ever made".to_string(),
            "I thought I made a mistake, but then I realized it led me to you, and I can't help but be grateful for that".to_string(),
            "They say diamonds are made under pressure, but I believe great connections are made through adorable mistakes. Case in point: us.".to_string(),
            "If making mistakes were a skill, I'd be a master. Fortunately, my favorite mistake is meeting you".to_string(),
            "I may not be perfect, but my mistakes seem to disappear when I'm with you. Maybe it's magic, or maybe it's just us".to_string(),]
    }
    ///# Destroy_with_rizz:
    /// Calls "destroy" interface
    ///
    ///  # Syntax
    /// destroy_with_rizz(robot:  & mut impl Runnable, world: &mut World, direction:Direction)-> (String,Result<usize, LibError>):
    ///
    /// # Usage
    ///println!("{}",Rizzler::init_error_rizz()(*parameters))

    pub fn destroy_with_rizz(robot:  & mut impl Runnable, world: &mut World, direction:Direction) -> (String,Result<usize, LibError>) {
        let mut output_string=String::new();
        
        let destroy_vec : Vec<String> = vec!["Can i catch you? You look like an exception".to_string(),
                                             "Baby, if they made you in java you'd be the object of my desire".to_string(),
                                             "Are you a borrow checker? Because you are making sure there is no ownership conflicts in my heart".to_string(),
                                             "Are you a wrecking ball? Because you just knocked down all my defenses".to_string(),
                                             "Are you a demolition expert? Because you just demolished the walls around my heart".to_string(),
                                             "Did you play Jenga as a kid? Because you just pulled out the last piece, and now I'm falling for you".to_string(),
                                             "Are you a superhero? Because you just crashed through the walls of my ordinary life".to_string(),
                                             "Is your name Dynamite? Because you've just blasted your way into my heart".to_string(),
                                             "Are you a supernova? Because you just exploded into my universe, and now everything revolves around you".to_string()];
        let errors_vec : Vec<String> = Rizzler::init_error_rizz();
        let x = destroy(robot,world,direction );
        match x{
            Ok(_) => {
                output_string =  destroy_vec[rand::thread_rng().gen_range(0..destroy_vec.len())].clone();
                println!("{}",output_string);
            }
            Err(_) => { 
                output_string=errors_vec[rand::thread_rng().gen_range(0..errors_vec.len())].clone();
                println!("{}",output_string);
            }

        }
        (output_string,x)
    }
    ///# Put_with_rizz:
    ///
    /// Calls "put" interface
    ///
    /// # Syntax
    /// put_with_rizz(robot: & mut impl Runnable, world: &mut World, content: Content, quantity:usize, direction: Direction)-> (String,Result<usize, LibError>)
    ///
    /// # Usage
    ///println!("{}",Rizzler::put_with_rizz(*parameters))
    pub fn put_with_rizz(robot: & mut impl Runnable, world: &mut World, content: Content, quantity:usize, direction: Direction ) -> (String,Result<usize, LibError>) {
        let mut output_string=String::new();

        let put_vec : Vec<String> = vec!["Excuse me, but I think you dropped something: my jaw".to_string(),
                                          "Are you a coin? Because wherever you are, that's where I want to put my luck".to_string(),
                                          "If I had a coin for every time I thought about you, I'd have put a fortune in your name".to_string(),
                                          "Are you a power source? Because whenever you are around, you put a spark in my energy level".to_string(),
                                          "If you were a battery, I'd put you in my heart because you charge me up".to_string(),
                                         "Are you a puzzle? Because it feels like you're the missing piece I've been putting off finding".to_string(),
                                         "Did it hurt when you fell from the vending machine? Because you're a snack I wouldn't mind putting in my pocket".to_string(),
                                         "Are you a book? Because every time I see you, I feel like I'm putting another chapter in the story of us".to_string(),
                                         "Do you have a map? I keep getting lost in your eyes, and I need help putting my way back to reality".to_string(),
                                         "Are you a camera? Because every time I see you, I can't help but think about putting you in the picture of my life".to_string(),
                                         "Are you a recipe? Because I can't wait to start putting the ingredients of a great relationship together with you".to_string(), ];

        let errors_vec : Vec<String> = Rizzler::init_error_rizz();

        let x = put(robot, world, content, quantity, direction );
        match x{
            Ok(_) => {
                output_string =  put_vec[rand::thread_rng().gen_range(0..put_vec.len())].clone();
                println!("{}",output_string);
            }
            Err(_) => { 
                output_string=errors_vec[rand::thread_rng().gen_range(0..errors_vec.len())].clone();

                println!("{}",output_string);
            }

        }
        (output_string,x)
    }
    ///# Discover_tiles_with_rizz:
    /// Calls "discover tile" interface
    ///
    ///  # Syntax
    /// discover_tiles_with_rizz(robot: &mut impl Runnable, world: &mut World, to_discover: &[(usize, usize)])->(String, Result<HashMap<(usize, usize), Option<Tile>>, LibError>)
    ///
    /// # Usage
    ///Rizzler::println!("{}",Rizzler::discover_tiles_with_rizz(*parameters))
    pub fn discover_tiles_with_rizz(robot: &mut impl Runnable, world: &mut World, to_discover: &[(usize, usize)]) 
    -> (String, Result<HashMap<(usize, usize), Option<Tile>>, LibError>){

        let mut output_string=String::new();

        let discover_tiles_vec : Vec<String> = vec!["Do you have a map? Cause I'm lost in your eyes and i need directions to your heart".to_string(),
                                                    "Are you a treasure map? Because every step I take with you feels like uncovering a new tile of excitement".to_string(),
                                                    "Are you a game of dominoes? Because falling for you seems inevitable, just like the tiles".to_string(),
                                                    "If you were a tile in a game of Mahjong, you'd be the on I'd want to pick up and keep close".to_string(),
                                                    "Is your name Columbus? Because you've just discovered a whole new world in my heart".to_string(),
                                                    "Is your laughter a hidden oasis? Because discovering it feels like finding a refreshing spring in the desert of my day".to_string(),
                                                    "Your love is just like an unexplored island, makes me feel like an adventurer discovering new wonders in your heart".to_string()];
    
        let errors_vec : Vec<String> = Rizzler::init_error_rizz();

        let x = discover_tiles(robot, world, to_discover);
        match x{
            Ok(_) => {
                output_string =  discover_tiles_vec[rand::thread_rng().gen_range(0..discover_tiles_vec.len())].clone();
                println!("{}",output_string);
            }
            Err(_) => { 
                output_string=errors_vec[rand::thread_rng().gen_range(0..errors_vec.len())].clone();
                println!("{}",output_string);
            }
        }
        (output_string,x)
    }
    ///# Teleport_with_rizz:
    /// Calls "teleport" interface
    ///
    ///  # Syntax
    /// teleport_with_rizz(robot: &mut impl Runnable, world: &mut World, coordinates: (usize, usize))-> (String, Result<TileMatrix, LibError>)
    ///
    /// # Usage
    ///println!("{}",Rizzler::teleport_with_rizz(*parameters))

    pub fn teleport_with_rizz(robot: &mut impl Runnable, world: &mut World, coordinates: (usize, usize)) -> (String, Result<TileMatrix, LibError>) {

        let mut output_string=String::new();

        let teleport_vec : Vec<String> = vec!["If I had a teleporter, I'd program it to bring me to you every time you crossed my mind".to_string(),
                                              "Are you a teleporter? Because meeting you instantly changed my coordinates".to_string(),
                                              "Are you a teleporter? Because everytime i see you my heart skips a dimension".to_string(),
                                              "Do you have a world map? Because i'm getting lost in the geography of your charm".to_string(),
                                              "If teleporting were an option, I'd choose to materialize right next to you".to_string(),
                                              "I'd love to be your personal teleporter, so I could whisk you away to a romantic destination whenever you desire".to_string(),
                                              "I don't need a teleporter to tell me you're the destination I've been searching for".to_string()];

        let errors_vec : Vec<String> = Rizzler::init_error_rizz();

        let x = teleport(robot, world, coordinates );
        match x{
            Ok(_) => {
                output_string =  teleport_vec[rand::thread_rng().gen_range(0..teleport_vec.len())].clone();
                println!("{}",output_string);
            }
            Err(_) => { 
                output_string=errors_vec[rand::thread_rng().gen_range(0..errors_vec.len())].clone();
                println!("{}",output_string);
            }
        }
        (output_string,x)
    }
    ///# Look_at_sky_with_rizz:
    ///  Calls "look_at_sky" interface
    ///
    ///  # Syntax
    /// look_at_sky_with_rizz(world: &World) -> (String, EnvironmentalConditions)
    ///
    /// # Usage
    ///println!("{}",Rizzler::look_at_sky_with_rizz(*parameters))

    pub fn look_at_sky_with_rizz(world: &World) -> (String, EnvironmentalConditions) {

        let look_at_sky_vec : Vec<String> = vec!["When I'm with you i have the lifetime of my life".to_string(),
                                                 "My scope only ends with you".to_string(),
                                                 "If beauty were time, you'd be an eternity of sunsets".to_string(),
                                                 "If you were a constellation, you'd be the one that guides me home".to_string(),
                                                 "Looking at the sky is beautiful, but not as breathtaking as catching your gaze".to_string(),
                                                 "If you were a constellation, you'd be the one that lights up my darkest nights".to_string(),
                                                 "The clouds might be up high, but your smile takes me even higher".to_string(),
                                                 "The stars must be jealous tonight because your eyes outshine them all".to_string()];
        let output_string =  look_at_sky_vec[rand::thread_rng().gen_range(0..look_at_sky_vec.len())].clone();
        println!("{}",output_string);

        (output_string,look_at_sky(world))
    }
    ///# Craft_with_rizz:
    /// Calls "craft" interface
    ///
    ///  # Syntax
    /// craft_with_rizz(robot: &mut impl Runnable, content: Content)-> (String, Result<Content, LibError>)
    ///
    /// # Usage
    ///println!("{}",Rizzler::craft_with_rizz(*parameters))


    pub fn craft_with_rizz(robot: &mut impl Runnable, content: Content) -> (String, Result<Content, LibError>) {

        let mut output_string=String::new();

        let craft_vec: Vec<String> = vec!["Are you a crafting recipe because you and I together make a perfect combination".to_string(),
                                          "Are you a sculptor? Because the way you've crafted your smile is a work of art".to_string(),
                                          "Do you believe in fate? Because it seems like someone crafted our paths to cross".to_string(),
                                          "Are you a poet? Because the words you craft leave a lasting impression on my heart".to_string(),
                                          "Do you work with leather? Because you've crafted something truly special in the way you carry yourself".to_string(),
                                          "If kisses were crafts, I'd spend a lifetime perfecting the art with you".to_string(),
                                          "Do you believe in love at first craft? Because the moment I met you, my heart started creating something beautiful".to_string(),
                                          "If our love were a sculpture, it would be a masterpiece that stands the test of time".to_string(),
                                          "You're the DIY dream I never knew I had, and I can't wait to craft a future with you".to_string(),
                                          "If love were a craft, you'd be the essential tool in my toolbox.".to_string()];
        let errors_vec : Vec<String> = Rizzler::init_error_rizz();

        let x = craft(robot,  content,);
        match x{
            Ok(_) => {
                output_string =  craft_vec[rand::thread_rng().gen_range(0..craft_vec.len())].clone();
                println!("{}",output_string);
            }
           Err(_) => { 
                output_string=errors_vec[rand::thread_rng().gen_range(0..errors_vec.len())].clone();
                println!("{}",output_string);
            }
        }
        (output_string,x)
    }
}