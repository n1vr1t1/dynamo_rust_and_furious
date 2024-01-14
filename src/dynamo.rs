use robotics_lib::runner::Robot;
use robotics_lib::interface::Tools;
use robotics_lib::energy::Energy;

/// Hi! Thank u so much, for choosing the Dynamo by Rust And Furious™.
/// dynamo is a struct that implements only the 'update_energy' function, which takes no parameters and always returns the maximum (1000) energy your robot can have.
///
/// The philosophy of our tool is to be user-friendly. You get to decide when, where and how many times you want to call the function.
/// Since, we haven't imposed any restrictions to the calling of this function. We leaving it to you to set appropriate limits. 
/// 
/// The dynamo will give back a new instance of 'Energy' without ever interacting with your robot or your world. 
/// In case you are worried, the function does not have the ability to mess up your robot's initial energy, backpack or coordinates. 
/// 
/// # Example
/// ```rust
/// use dynamo_rust_and_furious;
/// use robotics_lib::runner::Robot;
/// use robotics_lib::energy::Energy;
/// use robotics_lib::event::events::Event;
/// use robotics_lib::runner::Runnable;
/// use robotics_lib::world::World;
/// use robotics_lib::interface::Direction;
/// use robotics_lib::interface::destroy;
/// 
/// struct YourRobot(Robot);
/// impl Runnable for YourRobot{
///     fn process_tick(&mut self, world: &mut World) {
///
///         let _=destroy(self, world,Direction::Down);
///         //can be used anywhere and at any time
///         *self.get_energy_mut()=dynamo_rust_and_furious::Dynamo::update_energy();
///         *self.get_energy_mut()=dynamo_rust_and_furious::Dynamo::update_energy(); // gives back new Energy again even if the energy is already at max
///     }
///     fn handle_event(&mut self, _event: Event) {}
///     fn get_energy(&self) -> &Energy {
///         &self.0.energy
///     }
///     fn get_energy_mut(&mut self) -> &mut Energy {
///         &mut self.0.energy
///     }
///     fn get_coordinate(&self) -> &Coordinate {
///         &self.0.coordinate
///     }
///     fn get_coordinate_mut(&mut self) -> &mut Coordinate {
///         &mut self.0.coordinate
///     }
///     fn get_backpack(&self) -> &BackPack {
///         &self.0.backpack
///     }
///     fn get_backpack_mut(&mut self) -> &mut BackPack {
///         &mut self.0.backpack
///     }    
/// }
///```

pub struct Dynamo{}
impl Tools for Dynamo{}
impl Dynamo{
    
    /// Caution: Please use this feature responsibly. Just because you can call ther function whenever you want, do not mea you can abuse its functionality. 
    /// So remember:"with great power comes with great responsibility" (Yes, it's cringy but hopefully you will not forget it).
    /// # Returns
    /// - `Energy`: a new instance of Energy which can be assigned to the energy of your robot
    pub fn update_energy() -> Energy {
        let puppet=Robot::new();
        puppet.energy
    }
}