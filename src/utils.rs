use std::iter::repeat;

use crypto::{digest::Digest, hmac::Hmac, mac::Mac, md5, sha1::Sha1, sha2::Sha256};
use rand::Rng;

static RANDOM_STR: &str = "1234567890abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

/// 使用md5加密字符串
pub fn encrypt_with_md5(content: &str) -> String {
    let mut md5 = md5::Md5::new();
    md5.input(content.as_bytes());
    md5.result_str()
}

/// 使用sha1加密字符并返回u8数组
pub fn encrypt_with_sha1(content: &str) -> Vec<u8> {
    let mut sha1 = Sha1::new();
    sha1.input_str(content);
    let mut out = get_output_vec(sha1.output_bits());
    sha1.result(&mut out);
    out.to_vec()
}

/// u8数组转base64字符串操作
pub fn encode_to_base64(bytes: &[u8]) -> String {
    base64::encode(bytes)
}

/// 字符串转u8数组
pub fn decode_to_base64_vec(str: &str) -> Vec<u8> {
    base64::decode(str).unwrap_or(vec![])
}

/// 获取长度为count的随机字符串
pub fn get_random_string(count: usize) -> String {
    let mut str = String::new();
    let mut rng = rand::thread_rng();
    for _ in 0..count {
        let index = rng.gen_range(0..RANDOM_STR.len());
        str.push_str(&RANDOM_STR[index..index + 1]);
    }
    str
}

/// 获取nonce
pub fn generate_nonce() -> String {
    get_random_string(16)
}

/// 获取签名的nonce
pub fn generate_signed_nonce(secret: &str, nonce: &str) -> String {
    let mut sha256 = Sha256::new();
    sha256.input(&decode_to_base64_vec(secret));
    sha256.input(&decode_to_base64_vec(nonce));
    let mut out: Vec<u8> = get_output_vec(sha256.output_bits());
    sha256.result(&mut out);
    encode_to_base64(&out)
}

/// 获取执行命令时的签名
pub fn generate_command_signature(
    url: &str,
    signed_nonce: &str,
    nonce: &str,
    data: &str,
) -> String {
    let sign = format!("{}&{}&{}&data={}", url, signed_nonce, nonce, data);
    let mut hmac = Hmac::new(Sha256::new(), &decode_to_base64_vec(signed_nonce));
    hmac.input(sign.as_bytes());
    let result = hmac.result();
    let code = result.code().clone();
    encode_to_base64(code)
}

/// 获取一个指定长度的vec
fn get_output_vec(size: usize) -> Vec<u8> {
    repeat(0).take((size + 7) / 8).collect()
}

#[cfg(test)]
mod test {
    use super::{
        decode_to_base64_vec, encode_to_base64, encrypt_with_md5, encrypt_with_sha1,
        generate_command_signature, generate_signed_nonce, get_random_string,
    };

    #[test]
    fn test_md5() {
        let result = encrypt_with_md5("test");
        assert_eq!("098f6bcd4621d373cade4e832627b4f6", result)
    }

    #[test]
    fn test_sha1() {
        let result = encrypt_with_sha1("test");
        let str = encode_to_base64(&result);
        assert_eq!("qUqP5cyxm6YcTAhz05Hph5gvu9M=", str)
    }

    #[test]
    fn test_base64() {
        assert_eq!("test", encode_to_base64(&decode_to_base64_vec("test")))
    }

    #[test]
    fn test_random_string() {
        let random = get_random_string(16);
        println!("{}", random);
    }

    #[test]
    fn test_signed_nonce() {
        let result = generate_signed_nonce("test", "1234");
        println!("{}", result);
    }

    #[test]
    fn test_generate_signature() {
        let result = generate_command_signature("test", "test", "test", "test");
        assert_eq!("IOSP119Hekgo9THjxG7OvJDpaiRwOMVsL05krsJqG/4=", result)
    }
}
