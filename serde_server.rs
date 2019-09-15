use std::net::{TcpListener, TcpStream};
use std::thread;
use std::collections::HashMap;
use std::str;
use std::str::FromStr;
use std::sync::{Arc, RwLock};
use std::fmt;
use std::fs::File;
use std::io::Read;
use std::io::{BufRead, BufReader};
use std::io::Write;
use std::time::Duration;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct Address {
    street: String,
    city: String,
    country: String,
}

#[derive(Serialize, Deserialize)]
enum Gender {
    Male,
    Female,
}

/***********
impl FromStr for Gender {
    type Err = String;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        match string {
            "male" => Ok(Gender::Male),
            "female" => Ok(Gender::Female),
            another => Err(another.to_owned()),
        }
    }
}

impl ToString for Gender {
    fn to_string(&self) -> String {
        match self {
            Gender::Male => "male",
            Gender::Female => "female",
        }
        .to_owned()
    }
}
************/

// #[derive(Debug)] does this internally.
impl fmt::Debug for Gender {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Gender::Male => write!(f, "M"),
            Gender::Female => write!(f, "F"),
            _ => write!(f, "U"),
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Hash)]
enum Level {
    SSC,
    HSC,
    Graduation,
}

/**********
impl FromStr for Level {
    type Err = String;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        match string {
            "ssc" => Ok(Level::SSC),
            "hsc" => Ok(Level::HSC),
            "graduation" => Ok(Level::Graduation),
            another => Err(another.to_owned()),
        }
    }
}
***********/

impl fmt::Debug for Level {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Level::SSC => write!(f, "SSC"),
            Level::HSC => write!(f, "HSC"),
            Level::Graduation => write!(f, "Graduation"),
            _ => write!(f, "Unknown"),
        }
    }
}

#[derive(Debug)]
enum Message {
    FetchRequest(Identifier),
    ModifyRequest(Person),
    InvalidRequest(String),
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
struct Identifier {
    ssn: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
struct Person {
    ssn: String,
    age: u16,
    name: String,
    gender: Gender,
    contacts: Vec<String>, 
    address: Address,
    education: HashMap<Level, i32>,
}

fn handle_client(mut stream: TcpStream, person: Arc<RwLock<Person>>) {

    loop {
        let mut line = String::new();
        // make sure to use references here as BufReader takes 
        // the ownership.
        let num_bytes = BufReader::new(&stream).read_line(&mut line).unwrap();
        // removing this will infinitely print 'invalid message'. Why????
        if num_bytes == 0 {
            println!("connection closed");
            break;
        }
        /**********   
        let data = match serde_json::from_str::<Person>(&line) {
            Ok(data) => {
                println!("Received modify request {:?}", data);
                Message::ModifyRequest(data)
            },
            Err(err) => {
                match serde_json::from_str::<Identifier>(&line) {
                    Ok(data) => {
                        println!("Received fetch request");
                        Message::FetchRequest(data)
                    },
                    Err(err) => {
                        println!("Received invalid request {}", err);
                        Message::InvalidRequest("Invalid data".to_string())        
                    }
                }
            }
        };
        */ 
        // sequence here will matter if we remove 'deny_unknown_fields'.
        // in that case uncomment the above code.
        let data = match serde_json::from_str::<Identifier>(&line) {
            Ok(data) => {
                println!("Received default request {:?}", data);
                Message::FetchRequest(data)
            },
            Err(err) => {
                match serde_json::from_str::<Person>(&line) {
                    Ok(data) => {
                        println!("Received custom request");
                        Message::ModifyRequest(data)
                    },
                    Err(err) => {
                        println!("Received invalid request {}", err);
                        Message::InvalidRequest("Invalid data".to_string())        
                    }
                }
            }
        };
        // to unwrap an enum either use 'match clause' OR 'if let'
        if let Message::FetchRequest(request) = data {
            if let Ok(person) = person.read() {
                // de-referencing is important here.
                let mut buffer = serde_json::to_vec(&*person).expect("serialization bug");
                buffer.push(b'\n');
                stream.write_all(&buffer);
            }
        } else if let Message::ModifyRequest(request) = data {
            if let Ok(mut person) = person.write() {
                // actually no need for explicit de-referencing.
                (*person).ssn = request.ssn;
                (*person).age = request.age;
                (*person).name = request.name;
                (*person).gender = request.gender;
                (*person).contacts = request.contacts;
                (*person).address = request.address;
                (*person).education = request.education;

                let mut buffer = serde_json::to_vec("Success").expect("serialization bug");
                buffer.push(b'\n');
                stream.write_all(&buffer);
            }
        } else if let Message::InvalidRequest(message) = data {
            let mut buffer = serde_json::to_vec("Invalid Request").expect("serialization bug");
            buffer.push(b'\n');
            stream.write_all(&buffer);
        }
    }
}

fn main() {

    let listener = TcpListener::bind("127.0.0.1:12345").unwrap();
    let person = Arc::new(RwLock::new(initialize()));
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("Accepted connection from {}", stream.peer_addr().unwrap());
                let person = person.clone();
                thread::spawn(move || {
                    handle_client(stream, person);
                });
            }
            Err(_)=> {
                println!("Error");
            }
        }
    }
}

fn initialize() -> Person {
    let mut contacts = Vec::new();
    contacts.push("8819927600".to_string());
    contacts.push("9188711602".to_string());

    let mut education = HashMap::new();
    education.entry(Level::SSC).or_insert(78);
    education.entry(Level::HSC).or_insert(77);
    education.entry(Level::Graduation).or_insert(63);

    let address = Address {
        street: "New Lane".to_string(),
        city: "Washington".to_string(),
        country: "United States".to_string(),
    };
    let person = Person {
        ssn: "150DS120".to_string(),
        age: 30,
        name: "Stewart".to_string(),
        gender: Gender::Male,
        contacts: contacts,
        address: address,
        education: education,
    };
    person
}
