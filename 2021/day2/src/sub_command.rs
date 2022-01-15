use utils::app_errors::AppError;

#[derive(Debug)]
pub enum Command {
    Up(u64),
    Down(u64),
    Forward(u64),
}

#[derive(Debug)]
pub struct SubCommand(pub Command);

impl SubCommand {
    pub(crate) fn new(direction: &str, value: u64) -> Self {
        let cmd = if direction == "down" {
            Command::Down(value)
        } else if direction == "forward" {
            Command::Forward(value)
        } else if direction == "up" {
            Command::Up(value)
        } else {
            panic!("Invalid direction")
        };

        SubCommand(cmd)
    }
}

impl TryFrom<Vec<&str>> for SubCommand {
    type Error = AppError;

    fn try_from(parts: Vec<&str>) -> Result<Self, Self::Error> {
        let convert = || -> Result<Self, Self::Error> {
            Ok(SubCommand::new(
                parts.get(0).unwrap().to_owned(),
                parts.get(1).unwrap().to_owned().parse().unwrap(),
            ))
        };

        let res = convert();

        if res.is_err() {
            let vec_as_string: String = parts.iter().fold(String::from(""), |acc, val| -> String {
                format!("{:?} {:?}", acc, val.to_string())
            });
            let message = format!("Error converting {:#?} into SubCommand", vec_as_string);

            return Err(AppError::GenericError(message));
        }

        res
    }
}
