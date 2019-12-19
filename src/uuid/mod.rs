//use data_encoding::BASE64;
use data_encoding::HEXUPPER;
use rand::prelude::ThreadRng;
use rand::Rng;
use std::cmp::Ordering;
use std::fmt;
pub const UUID_LEN: usize = 256 / 8;

#[derive(Copy, PartialEq, Eq, Debug, Hash)]
pub struct Uuid {
    value: [u8; UUID_LEN],
}
impl AsRef<[u8]> for Uuid {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        &self.value[..]
    }
}

impl AsMut<[u8]> for Uuid {
    #[inline]
    fn as_mut(&mut self) -> &mut [u8] {
        &mut self.value[..]
    }
}

impl fmt::Display for Uuid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", HEXUPPER.encode(&self.value).to_string())
    }
}

impl Uuid {
    /// Creates a new Point.
    pub fn new(uuid: [u8; UUID_LEN]) -> Uuid {
        Uuid { value: uuid }
    }

    pub fn random_uuid(mut rng: ThreadRng) -> Uuid {
        let uuid_bytes: &mut [u8; UUID_LEN] = &mut [0; UUID_LEN];
        for item in uuid_bytes.iter_mut() {
            *item = rng.gen();
        }
        Uuid::new(*uuid_bytes)
    }

    pub fn from_vec(vector: Vec<u8>) -> [u8; UUID_LEN] {
            let mut array = [0; UUID_LEN];
            for (position,byte) in vector.iter().enumerate() {
                array[position] = *byte;
            }
            array
        }
    #[allow(dead_code)]
    pub fn from_string(string: String) -> Uuid {
        Uuid::new(Uuid::from_vec(HEXUPPER.decode(string.as_ref()).unwrap()))
    }

    #[allow(dead_code)]
    pub fn to_array(&self) -> [u8; UUID_LEN] {
        self.value
    }
}

impl Clone for Uuid {
    fn clone(&self) -> Uuid {
        let output: Uuid = Uuid::new(self.value);
        output
    }
}

impl Ord for Uuid {
    fn cmp(&self, other: &Self) -> Ordering {
        let mut answer: Ordering = Ordering::Equal;
        for i in 0..UUID_LEN {
            answer = self.value[i].cmp(&other.value[i]);
            if answer != Ordering::Equal {
                break;
            }
        }
        answer
    }
}

impl PartialOrd for Uuid {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
