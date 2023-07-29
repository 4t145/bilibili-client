use rand::Rng;

/// see https://github.com/SocialSisterYi/bilibili-API-collect/blob/master/grpc_api/readme.md
pub fn gen_aurora_eid(uid: u64) -> Option<String> {
    if uid == 0 {
        return None;
    }
    let mut result_byte = Vec::with_capacity(64);
    // 1. 将 UID 字符串转为字节数组.
    let mid_byte = uid.to_string().into_bytes();
    // 2. 将字节数组逐位(记为第 i 位)与 b"ad1va46a7lza" 中第 (i % 12) 位进行异或操作, 作为结果数组第 i 位.
    mid_byte
        .iter()
        .enumerate()
        .for_each(|(i, v)| result_byte.push(v ^ (b"ad1va46a7lza"[i % 12])));
    // 3. 对字节数组执行 Base64 编码, 注意 no padding, 即得到 x-bili-aurora-eid.
    Some(base64::Engine::encode(
        &base64::engine::general_purpose::STANDARD_NO_PAD,
        result_byte,
    ))
}

pub fn random_alphanumeric_string(len: usize) -> String {
    rand::thread_rng()
        .sample_iter(&rand::distributions::Alphanumeric)
        .take(len)
        .map(char::from)
        .collect::<String>()
}

pub fn random_hex_string(len: usize) -> String {
    (0..len)
        .map(|_| format!("{:x}", rand::thread_rng().gen::<u8>()))
        .collect()
}

/// see https://github.com/SocialSisterYi/bilibili-API-collect/blob/master/grpc_api/readme.md
pub fn gen_trace_id() -> String {
    // 1. 生成 32 位随机字符串 random_id , Charset 为 0~9, a~z.
    let random_id = random_alphanumeric_string(32).to_lowercase();
    let mut random_trace_id = String::with_capacity(40);
    // 2. 取 random_id 前 24 位, 作为 random_trace_id.
    random_trace_id.push_str(&random_id[0..24]);
    // 3. 初始化一个长度为 3 的数组 b_arr, 初始值都为 0.
    let mut b_arr: [i8; 3] = [0i8; 3];
    // 并获取当前时间戳
    let mut ts = chrono::Local::now().timestamp();
    // 使用循环从高位到低位遍历 b_arr 数组, 循环体内执行以下逻辑:
    //  - 首先将 ts 右移 8 位
    //  - 然后根据条件向 b_arr 的第 i 位赋值:
    //    - 如果 (ts / 128) % 2的结果为0, 则 b_arr[i] = ts % 256
    //    - 否则 b_arr[i] = ts % 256 - 256
    for i in (0..3).rev() {
        ts >>= 8;
        b_arr[i] = {
            if ((ts / 128) % 2) == 0 {
                (ts % 256) as i8
            } else {
                (ts % 256 - 256) as i8
            }
        }
    }
    // 4. 将数组 b_arr 中的每个元素逐个转换为两位的十六进制字符串并追加到 random_trace_id 中.
    for i in 0..3 {
        random_trace_id.push_str(&format!("{:0>2x}", b_arr[i]))
    }
    // 5. 将 random_id 的第 31, 32 个字符追加到 random_trace_id 中, 此时 random_trace_id 生成完毕, 应当为 32 位长度.
    random_trace_id.push_str(&random_id[30..32]);
    // 6. 最后, 按 `{random_trace_id}:{random_trace_id[16..32]}:0:0` 的顺序拼接起来, 即为 x-bili-trace-id
    let mut random_trace_id_final = String::with_capacity(64);
    random_trace_id_final.push_str(&random_trace_id);
    random_trace_id_final.push(':');
    random_trace_id_final.push_str(&random_trace_id[16..32]);
    random_trace_id_final.push_str(":0:0");
    random_trace_id_final
}

/// see https://github.com/SocialSisterYi/bilibili-API-collect/blob/master/docs/misc/device_identity.md
pub fn device_id(fp_raw: &str) -> String {
    let mut veri_code = 0;
    // 有点像 HEX 的操作
    let fp_raw_sub_str = fp_raw
        .as_bytes() // 将字符串 fp_raw 转换为字节数组
        .chunks(2) // 按每两个字节一组进行切分
        .map(|s| unsafe { ::std::str::from_utf8_unchecked(s) }) // 对每一组解析作为 UTF-8 字符串
        .collect::<Vec<_>>(); // 将结果收集到 Vec 中
                              // 如果 fp_raw 的长度小于 62, 则向下取偶数减半作为循环终止条件, 否则终止条件为31
    for i in 0..({
        if fp_raw.len() < 62 {
            fp_raw.len() - fp_raw.len() % 2 // 取偶数
        } else {
            62
        }
    } / 2)
    {
        // 将每组字符串转换为对应的 16 进制整数, 将转换得到的整数加到 veri_code 上.
        veri_code += i32::from_str_radix(fp_raw_sub_str[i], 16).unwrap_or(0);
    }
    // 最后将 veri_code 对 256 取余, 格式化为两位的 16 进制字符串
    let veri_code = format!("{:0>2x}", veri_code % 256);
    veri_code
}
