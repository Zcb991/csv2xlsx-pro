fn main() {
    let mut res = winres::WindowsResource::new();
    // res.set_icon("../assets/icon.ico"); // 指定你的 .ico 文件路径
    res.set_icon("./assets/csv2xlsx.ico"); // 指定你的 .ico 文件路径
    res.compile().expect("资源编译失败");
}