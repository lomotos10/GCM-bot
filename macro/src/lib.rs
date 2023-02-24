use proc_macro::{TokenStream, TokenTree};

fn tt2literal(tt: &TokenTree) -> String {
    if let TokenTree::Literal(a) = tt {
        let s = a.to_string();
        let mut s = s.chars();
        s.next();
        s.next_back();
        s.collect()
    } else {
        panic!()
    }
}

#[proc_macro]
pub fn jacket_template(tokens: TokenStream) -> TokenStream {
    let tokens = tokens.into_iter().collect::<Vec<_>>();
    // println!("{:?}", tokens);

    let game = tt2literal(&tokens[0]);
    let mut jacket_url = tt2literal(&tokens[2]);
    // This is O(N) and I do not care
    let l = jacket_url.len();
    if jacket_url.starts_with('\\') {
        jacket_url.remove(l - 2);
        jacket_url.remove(0);
    }

    let code = format!("
    let actual_title = get_title(&title, &ctx.data().{}_aliases, ctx.guild_id().unwrap_or(poise::serenity_prelude::GuildId(0)));
    if actual_title == None {{
        let mut log = ctx.data().alias_log.lock().await;
        let closest = get_closest_title(&title, &ctx.data().{}_aliases, ctx.guild_id().unwrap_or(poise::serenity_prelude::GuildId(0)));
        writeln!(log, \"{{}}\\t{}\\t{{}}\\t{{}}\", title, closest.0, closest.1)?;
        log.sync_all()?;
        drop(log);
        let reply = format!(
            \"I couldn't find the results for **{{}}**;
Did you mean **{{}}** (for **{{}}**)?
(P.S. You can also use the `/add-alias` command to add this alias to the bot.)\",
            title, closest.0, closest.1
        );
        let sent = ctx
            .send(|f| {{
                let mut f = f.ephemeral(true).content(reply);
                if let Context::Application(_) = ctx {{
                    f = f.components(|c| {{
                        let mut button = CreateButton::default();
                        button.custom_id(closest.0);
                        button.label(format!(\"Yes (times out after {{}} seconds)\", 10));
                        let mut ar = CreateActionRow::default();
                        ar.add_button(button);
                        c.set_action_row(ar)
                    }})
                }}
                f
            }})
            .await?;
        if let ReplyHandle::Unknown {{ interaction, http }} = sent {{
            if let Context::Application(poise_ctx) = ctx {{
                let serenity_ctx = poise_ctx.discord;
                let m = interaction.get_interaction_response(http).await.unwrap();
                let mci = match m
                    .await_component_interaction(&serenity_ctx)
                    .timeout(Duration::from_secs(10))
                    .await
                {{
                    Some(ci) => ci,
                    None => {{
                        // ctx.send(|f| f.ephemeral(true).content(\"Timed out\"))
                        //     .await
                        //     .unwrap();
                        return Ok(());
                    }}
                }};
                let actual_title =
                    get_title(&mci.data.custom_id, &ctx.data().{}_aliases, ctx.guild_id().unwrap_or(poise::serenity_prelude::GuildId(0))).unwrap();
                mci.create_interaction_response(&http, |r| {{
                    r.kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|d| {{
                            let jacket = &ctx.data().{}_charts[&actual_title].jp_jacket;
                            if let Some(jacket) = jacket {{
                                d.content(format!(\"Query by <@{{}}>\", ctx.author().id)).add_file(AttachmentType::Image(
                                    url::Url::parse(&format!(
                                        \"{{}}{{}}\",
                                        {},
                                        jacket
                                    ))
                                    .unwrap(),
                                ));
                            }}
                            d
                        }})
                }})
                .await?;
            }}
        }}
        return Ok(());
    }}
    let title = actual_title.unwrap();
    let jacket = &ctx.data().{}_charts[&title].jp_jacket;
    if let Some(jacket) = jacket {{
        ctx.send(|f| {{
            f.attachment(serenity::AttachmentType::Image(
                url::Url::parse(&format!(
                    \"{{}}{{}}\",
                    {},
                    jacket
                ))
                .unwrap(),
            ))
        }})
        .await?;
    }}
    ", game, game, game, game, game, jacket_url, game, jacket_url);
    // println!("{}", code);
    code.parse().unwrap()
}

#[proc_macro]
pub fn info_template(tokens: TokenStream) -> TokenStream {
    let tokens = tokens.into_iter().collect::<Vec<_>>();
    // println!("{:?}", tokens);

    let game = tt2literal(&tokens[0]);
    let color = tt2literal(&tokens[2]);
    let mut jacket_url = tt2literal(&tokens[4]);
    // This is O(N) and I do not care
    let l = jacket_url.len();
    if jacket_url.starts_with('\\') {
        jacket_url.remove(l - 2);
        jacket_url.remove(0);
    }

    let code = format!("
    let actual_title = get_title(&title, &ctx.data().{}_aliases, ctx.guild_id().unwrap_or(poise::serenity_prelude::GuildId(0)));
    if actual_title == None {{
        let mut log = ctx.data().alias_log.lock().await;
        writeln!(log, \"{{}}\\t{}\", title)?;
        log.sync_all()?;
        drop(log);
        let closest = get_closest_title(&title, &ctx.data().{}_aliases, ctx.guild_id().unwrap_or(poise::serenity_prelude::GuildId(0)));
        let reply = format!(
            \"I couldn't find the results for **{{}}**;
Did you mean **{{}}** (for **{{}}**)?
(P.S. You can also use the `/add-alias` command to add this alias to the bot.)\",
            title, closest.0, closest.1
        );
        let sent = ctx
            .send(|f| {{
                let mut f = f.ephemeral(true).content(reply);
                if let Context::Application(_) = ctx {{
                    f = f.components(|c| {{
                        let mut button = CreateButton::default();
                        button.custom_id(closest.0);
                        button.label(format!(\"Yes (times out after {{}} seconds)\", 10));
                        let mut ar = CreateActionRow::default();
                        ar.add_button(button);
                        c.set_action_row(ar)
                    }})
                }}
                f
            }})
            .await?;
        if let ReplyHandle::Unknown {{ interaction, http }} = sent {{
            if let Context::Application(poise_ctx) = ctx {{
                let serenity_ctx = poise_ctx.discord;
                let m = interaction.get_interaction_response(http).await.unwrap();
                let mci = match m
                    .await_component_interaction(&serenity_ctx)
                    .timeout(Duration::from_secs(10))
                    .await
                {{
                    Some(ci) => ci,
                    None => {{
                        // ctx.send(|f| f.ephemeral(true).content(\"Timed out\"))
                        //     .await
                        //     .unwrap();
                        return Ok(());
                    }}
                }};
                let actual_title =
                    get_title(&mci.data.custom_id, &ctx.data().{}_aliases, ctx.guild_id().unwrap_or(poise::serenity_prelude::GuildId(0))).unwrap();
                mci.create_interaction_response(&http, |r| {{
                    r.kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|d| {{
                            // Make the message hidden for other users by setting `ephemeral(true)`.
                            d.ephemeral(false).content(format!(\"Query by <@{{}}>\", ctx.author().id)).embed(|f| {{
                                let (description, jacket) =
                                    get_{}_embed(actual_title.to_string(), ctx).unwrap();

                                let mut f = f
                                    .title({}_duplicate_alias_to_title(&actual_title))
                                    .description(description)
                                    .color(serenity::utils::Color::from_rgb({}));
                                if let Some(jacket) = jacket {{
                                    f = f.thumbnail(format!(
                                        \"{{}}{{}}\",
                                        {},
                                        jacket
                                    ));
                                }}

                                f
                            }})
                        }})
                }})
                .await?;
            }}
        }}
        return Ok(());
    }}

    match check_cooldown(&ctx).await {{
        Cooldown::Channel(t) => {{
            let is_slash_command = matches!(&ctx, poise::Context::Application(_));
            ctx.send(|f| {{
                f.ephemeral(is_slash_command).content(format!(
                    \"Channel cooldown: please wait {{}} seconds and try again, or try the #bot-commands channel for no cooldown.\",
                    t
                ))
            }})
            .await?;
            return Ok(());
        }}
        Cooldown::User(t) => {{
            if let poise::Context::Application(_) = &ctx {{
                ctx.send(|f| {{
                    f.ephemeral(true).content(format!(
                        \"Channel cooldown: please wait {{}} seconds and try again, or try the #bot-commands channel for no cooldown.\",
                        t
                    ))
                }})
                .await?;
            }}
            return Ok(());
        }}
        Cooldown::None => (),
    }}
    let title = actual_title.unwrap();
    let (description, jacket) = get_{}_embed(title.clone(), ctx)?;

    ctx.send(|f| {{
        f.embed(|f| {{
            let mut f = f
                .title({}_duplicate_alias_to_title(&title).replace('*', \"\\\\*\"))
                .description(description)
                .color(serenity::utils::Color::from_rgb({}));
            if let Some(jacket) = jacket {{
                f = f.thumbnail(format!(
                    \"{{}}{{}}\",
                    {},
                    jacket
                ));
            }}

            f
        }})
    }})
    .await?;", game, game, game, game, game, game, color, jacket_url, game, game, color, jacket_url);
    // println!("{}", code);
    code.parse().unwrap()
}
