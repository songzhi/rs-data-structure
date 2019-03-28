use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug, Eq, PartialEq)]
pub enum Action {
    Enter(usize),
    Leave(usize),
}

impl Action {
    pub fn as_usize(&self) -> usize {
        match self {
            Action::Enter(time) => *time,
            Action::Leave(time) => *time
        }
    }
}

impl FromStr for Action {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, ()> {
        let s = s.trim().to_ascii_lowercase();
        if s.starts_with("enter") {
            let s = s.replace("enter", "")
                .replace("(", "")
                .replace(")", "");
            Ok(Action::Enter(s.trim().parse().map_err(|_| ())?))
        } else if s.starts_with("leave") {
            let s = s.replace("leave", "")
                .replace("(", "")
                .replace(")", "");
            Ok(Action::Leave(s.trim().parse().map_err(|_| ())?))
        } else {
            Err(())
        }
    }
}

#[derive(Debug)]
pub struct Car {
    plate_number: String,
}

impl Car {
    pub fn new(plate_number: &str) -> Self {
        Self {
            plate_number: String::from(plate_number)
        }
    }
}

impl PartialEq for Car {
    fn eq(&self, other: &Self) -> bool {
        self.plate_number == other.plate_number
    }
}

#[derive(Debug)]
struct ParkingFee(usize);


impl From<usize> for ParkingFee {
    fn from(fee: usize) -> Self {
        ParkingFee(fee)
    }
}

type ChargeRecord = (Action, Action, Car, ParkingFee);

#[derive(Debug)]
pub struct ParkingLot {
    cars: HashMap<String, (Car, usize)>,
    history: Vec<ChargeRecord>,
}

impl ParkingLot {
    pub fn new() -> Self {
        Self {
            cars: HashMap::new(),
            history: vec![],
        }
    }
    pub fn add_car(&mut self, car: Car, enter_time: usize) {
        self.cars.insert(car.plate_number.clone(), (car, enter_time));
    }
    pub fn remove_car(&mut self, plate_number: &str, leave_time: usize) -> Option<usize> {
        let (car, enter_time) = self.cars.remove(plate_number)?;
        let parking_fee = leave_time - enter_time;
        self.history.push((Action::Enter(enter_time), Action::Leave(leave_time), car, parking_fee.into()));
        Some(parking_fee)
    }

    /// line1: Enter(12) 京B8888
    /// line2: Leave(13) 京B8888
    pub fn parse_line(&mut self, line: &str) -> Result<(), ()> {
        let mut split = line.split(" ");
        let action = Action::from_str(split.next().ok_or(())?)?;
        let plate_number = split.next().ok_or(())?;
        match action {
            Action::Enter(time) => self.add_car(Car::new(plate_number), time),
            Action::Leave(time) => { self.remove_car(plate_number, time); }
        }
        Ok(())
    }
}

pub fn test() {
    let lines = vec![
        "Enter(1) 京A0001",
        "Enter(2) 京A8283",
        "Leave(3) 京A0001"
    ];
    let mut parking_lot = ParkingLot::new();
    for line in lines {
        parking_lot.parse_line(line);
        println!("{:?}", parking_lot);
    }
}
