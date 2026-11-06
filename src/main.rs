use std::{fs, io::Write};

use anyhow::bail;
use sysinfo::Disks;

const CLIENT_UI_NAME: &str = "SPSSClientUI.jar";
const CLIENT_UI_RES_NAME: &str = "SPSSClientUIRes.jar";
const LIC_NAME: &str = "ibmidlic.jar";
const CLIENT_UI: &[u8] = include_bytes!("SPSSClientUI.jar");
const CLIENT_UI_RES: &[u8] = include_bytes!("SPSSClientUIRes.jar");
const LIC: &[u8] = include_bytes!("ibmidlic.jar");

fn main() {
    println!("[*] SPSS 32 Cracker - nofyso");
    println!("[*] 本补丁仅供学习使用，如有经济能力，请支持正版");
    println!("[*] 这个简单的程序将会替换 SPSSClientUI.jar SPSSClientUIRes.jar ibmidlic.jar 从而使SPSS的许可认证检查无效化");
    println!("[*] 不想写UI……就这样吧");
    println!("[*] 如有需要，您可以在 https://github.com/nofyso/coloratum 上找到这个程序的源码、补丁文件和额外信息");
    println!("[*] 如果准备好了的话，就按Enter开始吧~");
    wait_line();
    let path = match find_dic() {
        Ok(x) => x,
        Err(e) => {
            println!("[!] 错误：{e}");
            println!("[!] 额抱歉，我没有找到这个文件夹，请你帮我找一下……好吗？");
            println!(r"[!] 它大概长这样：C:\Program Files\IBM\SPSS Statistics");
            println!("[?] 如果找到的话，就输入在下面，然后回车：");
            print!(" >  ");
            let _ = std::io::stdout().flush();
            let mut str = String::new();
            let _ = std::io::stdin().read_line(&mut str);
            str = str.trim_end().to_string();
            if !str.ends_with('\\') {
                str.extend(['\\']);
            }
            str
        }
    };
    if !fs::exists(&path).unwrap_or_default() {
        println!("[-] 没有找到目录，程序将退出……");
        wait_line();
        return;
    }
    if let Err(e) = backup(&path) {
        println!("[-] {e}");
        println!("[-] 备份失败，程序将退出……");
        wait_line();
        return;
    }
    if let Err(e) = over_write(&path) {
        println!("[-] {e}");
        println!("[-] 覆写失败，程序将退出，你需要手动还原配置……");
        wait_line();
        return;
    }
    println!("[+] 操作成功！感谢您的使用！^v^");
    println!("[?] 按任意键结束程序");
    wait_line();
}

fn wait_line() {
    let _ = std::io::stdin().read_line(&mut String::new());
}

fn over_write(path: &str) -> anyhow::Result<()> {
    println!("[*] 正在覆写");
    fs::write(format!("{path}{CLIENT_UI_NAME}"), CLIENT_UI)?;
    fs::write(format!("{path}{CLIENT_UI_RES_NAME}"), CLIENT_UI_RES)?;
    fs::write(format!("{path}{LIC_NAME}"), LIC)?;
    Ok(())
}

fn backup(path: &str) -> anyhow::Result<()> {
    let backup_dir = &format!("{path}backup\\");
    println!("[*] 正在备份至: {backup_dir}");
    let ui_path = format!("{backup_dir}{CLIENT_UI_NAME}");
    let ui_res_path = format!("{backup_dir}{CLIENT_UI_RES_NAME}");
    let lic_path = format!("{backup_dir}{LIC_NAME}");
    if fs::exists(backup_dir).unwrap_or_default() {
        if fs::exists(&ui_path).unwrap_or_default()
            || fs::exists(&ui_res_path).unwrap_or_default()
            || fs::exists(&lic_path).unwrap_or_default()
        {
            println!("[*] 已有备份文件，跳过步骤");
            return Ok(());
        }
    } else {
        fs::create_dir(backup_dir)?;
    }
    copy(&format!("{path}{CLIENT_UI_NAME}"), &ui_path)?;
    copy(&format!("{path}{CLIENT_UI_RES_NAME}"), &ui_res_path)?;
    copy(&format!("{path}{LIC_NAME}"), &lic_path)?;
    Ok(())
}

fn copy(src: &str, dst: &str) -> anyhow::Result<()> {
    println!("[*] 正在复制 {src} -> {dst}");
    let _ = fs::copy(src, dst)?;
    Ok(())
}

fn find_dic() -> anyhow::Result<String> {
    let disks = Disks::new_with_refreshed_list()?;
    let program_files = &["Program Files", "Program Files (x86)"];
    let file_path = "\\IBM\\SPSS Statistics\\stats.exe";
    let Some(path) = disks
        .iter()
        .map(|it| it.mount_point().to_str().unwrap())
        .flat_map(|root| {
            program_files
                .iter()
                .map(move |sub_path| format!("{root}{sub_path}{file_path}"))
        })
        .find(|it| fs::exists(it).unwrap_or_default())
    else {
        bail!("file not found")
    };
    Ok(format!("{path}\\..\\"))
}
