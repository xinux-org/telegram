use crate::utils::inlines::*;
use libxinux::pkgs::any::{Any as Pkgs, Data};
use std::error::Error;
use teloxide::{prelude::*, types::*};

macro_rules! return_err_answer {
    ($bot:ident, $q:ident, $title:expr, $msg:expr) => {
        return {
            $bot.answer_inline_query(
                $q.id,
                vec![InlineQueryResultArticle::new(
                    uuid::Uuid::new_v4(),
                    $title,
                    InputMessageContent::Text(
                        InputMessageContentText::new($msg)
                            .parse_mode(ParseMode::Html)
                            .link_preview_options(LinkPreviewOptions {
                                is_disabled: true,
                                url: None,
                                prefer_small_media: false,
                                prefer_large_media: false,
                                show_above_text: false,
                            })
                    ),
                )
                .reply_markup(err_keyboard())
                .into()],
            )
            .await?;
            Ok(())
        }
    };
}

pub async fn inline(
    bot: Bot,
    pkgs: Pkgs,
    q: InlineQuery,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let parsed: String = q.query.clone();

    let parsed = parsed.split_whitespace().collect::<Vec<&str>>();

    match parsed.len() {
        0 => return_err_answer!(bot, q, "Qidirishni boshlang!", NO_INPUT),
        1 => return_err_answer!(bot, q, "Parametrlar yetarli emas!", NOT_ENOUGH),
        2 => {}
        3.. => return_err_answer!(bot, q, "Parametrlar haddan ko'p!", TOO_MANY),
    }

    match parsed[0] {
        "arch" => {
            let request = pkgs.search(parsed[1]).await;

            let request: Vec<Data> = match request {
                Ok(v) => v,
                Err(_) => return_err_answer!(bot, q, "Xatolik yuz berdi!", INTERNAL_ERROR),
            };

            let request: Vec<&Data> = request.iter().take(49).collect();

            if request.is_empty() {
                return_err_answer!(bot, q, "Hech narsa topilmadi!", NOT_FOUND)
            }

            let request: Vec<InlineQueryResult> = request
                .iter()
                .map(|d: &&Data| {
                    InlineQueryResult::Article(
                        InlineQueryResultArticle::new(
                            uuid::Uuid::new_v4(),
                            d.name.clone(),
                            InputMessageContent::Text(
                                InputMessageContentText::new(view_generate(d))
                                    .parse_mode(ParseMode::Html)
                                    .link_preview_options(LinkPreviewOptions {
                                        is_disabled: true,
                                        url: None,
                                        prefer_small_media: false,
                                        prefer_large_media: false,
                                        show_above_text: false,
                                    })
                            ),
                        )
                            .description(d.description.clone().unwrap())
                            .reply_markup(kb_generate(d)),
                    )
                })
                .collect();

            bot.answer_inline_query(q.id, request).send().await?;
        }
        "nixos" => {
            return_err_answer!(bot, q, "NixOS registri ustida ishlanmoqda!", NOT_FOUND)
        }
        _ => return_err_answer!(bot, q, "Noto'g'ri distributiv!", NOT_FOUND),
    }

    Ok(())
}
