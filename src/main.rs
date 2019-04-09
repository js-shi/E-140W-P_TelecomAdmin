extern crate reqwest;

use std::{io, net, collections::HashMap, io::Read};

fn main() {
    println!("================================================================");
    println!("本工具为免费使用，若您为本工具支付了任何费用，请申请退款。");
    println!("================================================================");
    println!("请输入光猫的IP地址:");
    let mut modem_ip = String::new();
    io::stdin().read_line(&mut modem_ip).expect("无效输入。");
    let modem_ip: net::Ipv4Addr = modem_ip.trim().parse().expect("无效IP地址。");
    let psi_url = format!("http://{}/downloadFile?file=/var/config/psi", modem_ip);
    let mut resp = reqwest::get(&psi_url[..]).expect("无法访问光猫。");
    if !resp.status().is_success() {
        panic!("无法访问光猫。");
    }
    let mut psi_content: Vec<u8> = Vec::new();
    resp.read_to_end(&mut psi_content);
    println!("{}", psi_content.len());

    //let resp_content: HashMap<String, String> = resp.json().expect("读取光猫配置失败。");
    //println!("{:#?}", resp_content);
}
