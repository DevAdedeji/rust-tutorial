fn main() {
    //
    enum IpAddrKind {
        V4,
        V6,
    }

    enum IpAddr {
        V4(String),
        V6(String),
    }

    let home = IpAddr::V4(String::from("127.0.0.02"));

    let _four = IpAddrKind::V4;
    let _six = IpAddrKind::V6;

    fn route(_ip_kind: IpAddrKind) {}
    route(IpAddrKind::V4);

    // struct IpAddr {
    //     kind: IpAddrKind,
    //     address: String,
    // }

    // let _home = IpAddr {
    //     kind: IpAddrKind::V6,
    //     address: String::from("127.0.0.1"),
    // };

    // let _loopback = IpAddr {
    //     kind: IpAddrKind::V4,
    //     address: String::from("::1"),
    // };
}
