use crate::utils::inlines::*;
use libxinux::pkgs::any::{Any as Pkgs, Data};
use std::error::Error;
use teloxide::{prelude::*, types::*};

pub async fn inline(
    bot: Bot,
    pkgs: Pkgs,
    q: InlineQuery,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let parsed: String = q.query.clone();

    let parsed = parsed.split_whitespace().collect::<Vec<&str>>();

    match parsed.len() {
        0 => {
            return {
                bot.answer_inline_query(
                    q.id,
                    vec![InlineQueryResultArticle::new(
                        "101",
                        "Qidirishni boshlang!",
                        InputMessageContent::Text(
                            InputMessageContentText::new(NO_INPUT)
                                .parse_mode(ParseMode::Html)
                                .disable_web_page_preview(true),
                        ),
                    )
                    .reply_markup(err_keyboard())
                    .into()],
                )
                .await?;
                Ok(())
            };
        }
        1 => {
            return {
                bot.answer_inline_query(
                    q.id,
                    vec![InlineQueryResultArticle::new(
                        "102",
                        "Parametrlar yetarli emas!",
                        InputMessageContent::Text(
                            InputMessageContentText::new(NOT_ENOUGH)
                                .parse_mode(ParseMode::Html)
                                .disable_web_page_preview(true),
                        ),
                    )
                    .reply_markup(err_keyboard())
                    .into()],
                )
                .await?;
                Ok(())
            };
        }
        2 => {}
        3.. => {
            return {
                bot.answer_inline_query(
                    q.id,
                    vec![InlineQueryResultArticle::new(
                        "103",
                        "Parametrlar haddan ko'p!",
                        InputMessageContent::Text(
                            InputMessageContentText::new(TOO_MANY)
                                .parse_mode(ParseMode::Html)
                                .disable_web_page_preview(true),
                        ),
                    )
                    .reply_markup(err_keyboard())
                    .into()],
                )
                .await?;
                Ok(())
            };
        }
    }

    let request = pkgs.search(parsed[1]).await;

    println!("{:?}", request);

    let request: Vec<Data> = match request {
        Ok(v) => v,
        Err(_) => {
            return {
                bot.answer_inline_query(
                    q.id,
                    vec![InlineQueryResultArticle::new(
                        "500",
                        "Xatolik yuz berdi!",
                        InputMessageContent::Text(
                            InputMessageContentText::new(
                                format!("<b>API dan ma'lumot olishda xatolik yuz berdi!</b>\nIltimos, qayta yoki keyinroq urinib ko'ring!")
                            )
                            .parse_mode(ParseMode::Html)
                            .disable_web_page_preview(true),
                        ),
                    )
                    .reply_markup(err_keyboard())
                    .into()],
                )
                .await?;
                Ok(())
            };
        }
    };

    // get only 50 results
    let request: Vec<&Data> = request.iter().take(49).collect();

    if request.is_empty() {
        return {
            bot.answer_inline_query(
                q.id,
                vec![InlineQueryResultArticle::new(
                    "404",
                    "Couldn't find!",
                    InputMessageContent::Text(
                        InputMessageContentText::new(
                            format!("<b>There are no results related to {}!</b>\nPlease, Try to search with other names or parameters!",
                            q.query.clone())
                        )
                            .parse_mode(ParseMode::Html)
                            .disable_web_page_preview(true),
                    ),
                )
                    .reply_markup(err_keyboard())
                .into()],
            )
            .await?;
            Ok(())
        };
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
                            .disable_web_page_preview(true),
                    ),
                )
                .description(d.description.clone().unwrap())
                .reply_markup(kb_generate(d)),
            )
        })
        .collect();

    bot.answer_inline_query(q.id, request).send().await?;
    Ok(())
}
