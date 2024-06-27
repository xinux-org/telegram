use libxinux::pkgs::any::{Data, Type};
use orzklv::telegram::keyboard::Keyboard;
use teloxide::types::*;

pub static NO_INPUT: &str = r#"
<b>Salom foydalanuvchi!</b>

Siz hozir inlayn rejim ishga tushurdingiz. Bu xususiyat yordamida siz Arch Linux yoki NixOS paketlarini qidirishingiz va chatda ulashishingiz mumkin. Qidirishni boshlash uchun, quyida keltirilgan misol tariqasida yozing:

<code>@xinuxmgr &lt;distributiv&gt; &lt;nomi&gt;</code>
"#;

pub static NOT_ENOUGH: &str = r#"
<b>Foydalanuvchi, siz yetarlicha parametrlar kiritmadingiz!</b>

Iltimos, quyidagi parametrlarni kiriting:
&lt;distributiv&gt; &lt;paket nomi&gt;
"#;

pub static TOO_MANY: &str = r#"
<b>Foydalanuvchi, siz keraklidan ko'p parametrlar kiritdingiz!</b>
"#;

pub static INTERNAL_ERROR: &str = r#"
<b>API dan ma'lumot olishda xatolik yuz berdi!</b>
Iltimos, qayta yoki keyinroq urinib ko'ring!
"#;

pub static NOT_FOUND: &str = r#"
<b>There are no results related to {}!</b>\nPlease, Try to search with other names or parameters!
"#;

pub fn view_generate(d: &Data) -> String {
    let d = d.clone();
    let mut result = String::new();

    result.push_str(&format!("<b>Nomi:</b> {}\n", d.name));

    result.push_str(&format!("<b>Versiyasi:</b> <code>{}</code>\n", d.version));

    result.push_str(&format!(
        "<b>Ma'lutot:</b> {}\n",
        d.description.unwrap_or("Ma'lumot yo'q".to_string())
    ));

    if d.repo.is_some() {
        result.push_str(&format!(
            "<b>Repozitoriya:</b> {}\n",
            d.repo.clone().unwrap()
        ));
    }

    result.push_str("🔌 <b>O'rnatish uchun:</b> \n");

    match d.repo {
        Some(_) => {
            result.push_str(&format!("<code>pacman -S {}</code>\n", d.name));
        }
        None => {
            result.push_str(&format!("<code>paru -S {}</code>\n", d.name));
        }
    }

    result
}

pub fn kb_generate(d: &Data) -> InlineKeyboardMarkup {
    let d = d.clone();
    let mut keyboard = Keyboard::new();

    keyboard
        .url(
            "Web sahifasi",
            match d.types {
                Type::Aur => format!("https://aur.archlinux.org/packages/{}", d.name),
                Type::Std => format!(
                    "https://archlinux.org/packages/{}/{}/{}",
                    d.repo.unwrap(),
                    d.arch,
                    d.name
                ),
            }
            .as_str(),
        )
        .unwrap()
}

pub fn err_keyboard() -> InlineKeyboardMarkup {
    let mut keyboard = Keyboard::new();
    keyboard.switch_inline_current("Qayta urinib ko'ramizmi?", "arch linux")
}
