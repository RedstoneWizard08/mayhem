pub mod password;
pub mod serde;
pub mod user;

pub fn parse_ip(ip: String) -> [u8; 4] {
    let split = ip.split('.');

    if split.clone().count() < 4 {
        panic!("Unknown IPv4 address!");
    }

    let octet_1 = split.clone().next().unwrap().parse::<u8>().unwrap_or(0);
    let octet_2 = split.clone().nth(1).unwrap().parse::<u8>().unwrap_or(0);
    let octet_3 = split.clone().nth(2).unwrap().parse::<u8>().unwrap_or(0);
    let octet_4 = split.clone().nth(3).unwrap().parse::<u8>().unwrap_or(0);

    return [octet_1, octet_2, octet_3, octet_4];
}
