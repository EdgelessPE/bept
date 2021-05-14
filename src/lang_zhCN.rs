#![allow(dead_code)]
pub mod formatter {
    use crate::search;
    use colorful::Colorful;
    pub fn startup(ver: &str) -> String {
        format!("Better Edgeless Plugin Tool, 版本 {}", ver)
    }
    pub fn gotosc(sc: &str) -> String {
        format!("跳转到子命令 `{}`", sc)
    }
    pub fn nosc() -> String {
        "没有找到子命令".to_string()
    }
    pub fn found_file(file: &str) -> String {
        format!("找到文件 `{}`, Ok", file)
    }
    pub fn found_folder(folder: &str) -> String {
        format!("找到文件夹 `{}`, Ok", folder)
    }
    pub fn not_found_file_and_creating(file: &str) -> String {
        format!("未找到文件 `{}`, 正在创建...", file)
    }
    pub fn not_found_folder_and_creating(folder: &str) -> String {
        format!("未找到文件夹 `{}`, 正在创建...", folder)
    }
    pub fn writing_indexes() -> String {
        "正在写入索引".to_string()
    }
    pub fn exit_with_code(code: i32) -> String {
        format!("退出, 返回代码 = {}", code)
    }
    pub fn skip_check() -> String {
        "已跳过系统检查".to_string()
    }
    pub fn check_ok() -> String {
        "系统检查成功".to_string()
    }
    pub fn sea_found_kw(kw: &search::SearchKeywords) -> String {
        format!(
            "{} - {}.",
            "关键字".cyan().bold(),
            format!("{:?}", kw).dark_gray()
        )
    }
    pub fn sea_unknown_id() -> String {
        "未知_未知_未知_未知".to_string()
    }
    pub fn sea_invalid_id(kw: &str) -> String {
        format!("无效的索引ID, {:?}", kw)
    }
    pub fn sea_invalid_arg(kw: &str) -> String {
        format!("无效的搜索参数, {:?}", kw)
    }
    pub fn sea_invalid_name(kw: &str) -> String {
        format!("无效的绝对名称, {:?}", kw)
    }
    pub fn sea_invalid_regex(kw: &str) -> String {
        format!("无效的正则表达式, {:?}", kw)
    }
    pub fn sea_searching() -> String {
        "正在搜索...".to_string()
    }
    pub fn sea_result() -> String {
        "搜索结果: ".to_string()
    }
    pub fn write_default() -> String {
        "正在写入默认配置...".to_string()
    }
    pub fn search_nothing() -> String {
        "难道你什么也不搜吗？".to_string()
    }
    pub fn reading_indexes() -> String {
        "正在读取索引...".to_string()
    }
    pub fn updating_indexes() -> String {
        "正在更新索引...".to_string()
    }
    pub fn sea_more_kw(kw: &search::SearchKeywords) -> String {
        format!("关键字 {} 匹配到多个包", format!("{:?}", kw).dark_gray())
    }
    pub fn selected_pkg(pkg: &str) -> String {
        format!("已选择包 {:?}", pkg)
    }
    pub fn choose_one() -> String {
        format!("{}", "请选择一个 > ".bold())
    }
    pub fn start_dl_pkg() -> String {
        format!("{}", "开始下载包".bold())
    }
    pub fn dl_pkg_ok() -> String {
        format!("{}", "包下载完成".bold())
    }
}
