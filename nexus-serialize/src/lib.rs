use serde::{Deserialize, Serialize};

pub fn to_bytes<T>(value: &T) -> Result<Vec<u8>, serde_json::Error>
where
    T: ?Sized + Serialize,
{
    serde_json::to_string(value).map(|s| s.as_bytes().to_vec())
}

pub fn from_bytes<'a, T>(v: &'a [u8]) -> Result<T, serde_json::Error>
where
    T: Deserialize<'a>,
{
    let s = std::str::from_utf8(v).unwrap_or("");
    serde_json::from_str(s)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Debug)]
    struct MyData {
        id: String,
        val: u16,
    }

    #[derive(Serialize, Deserialize, Debug)]
    #[serde(rename(serialize = "container"))]
    #[serde(rename(deserialize = "container"))]
    struct MyMessage {
        name: String,
        age: u8,
        val: Vec<MyData>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    enum Test {
        Ping(u8),
        Pong(String),
    }

    #[test]
    fn sd() {
        let mydata1 = MyData {
            id: "md1".to_string(),
            val: 10,
        };

        let mydata2 = MyData {
            id: "md2".to_string(),
            val: 20,
        };

        let mymsg = MyMessage {
            name: "test".to_string(),
            age: 100,
            val: vec![mydata1, mydata2],
        };

        let bs = to_bytes(&mymsg).unwrap();
        eprintln!("serialized len={}", bs.len());

        let deserialized: MyMessage = from_bytes(bs.as_slice()).unwrap();
        eprintln!("deserialized: {deserialized:?}");
    }

    #[test]
    fn ping_pong() {
        let msg = Test::Ping(10);

        let bs = to_bytes(&msg).unwrap();
        eprintln!("serialized ping: {}", bs.len());

        let des: Test = from_bytes(bs.as_slice()).unwrap();
        eprintln!("ping: {des:?}");

        let msg = Test::Pong("test".to_string());

        let bs = to_bytes(&msg).unwrap();
        eprintln!("serialized pong: {}", bs.len());

        let des: Test = from_bytes(bs.as_slice()).unwrap();
        eprintln!("ping: {des:?}");
    }
}
