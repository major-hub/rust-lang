mod found_the_secret_number;


enum IpAddrType {
    V4(u8, u8, u8, u8),
    V6(String),
}

struct IpAddr {
    ip_type: IpAddrType,
    address: String,
}

impl IpAddrType {
    fn check_v4(&self) {

    }
}


fn main() {
    let main_ip = IpAddr {
        ip_type: IpAddrType::V4(192, 168, 43, 1),
        address: String::from("localhost"),
    };

    let last_ip = IpAddr {}
}