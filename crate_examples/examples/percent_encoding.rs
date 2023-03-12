fn main() {
    let result = percent_encoding::percent_decode_str("chrome-extension://ahmkjjgdligadogjedmnogbpbcpofeeo/html/suspended.html#ttl=%E9%97%B9%E9%92%9F%E7%BD%91%20-%20%E9%97%B9%E9%92%9F%E5%9C%A8%E7%BA%BF%20-%20%E9%97%B9%E9%92%9F%20-%20%E9%97%B9%E8%A1%A8&pos=0&uri=https://naozhong.net.cn/#time=13:59&title=%E9%97%B9%E9%92%9F&enabled=0&sound=musicbox&loop=1").decode_utf8_lossy();

    println!("{:?}", result);
}
