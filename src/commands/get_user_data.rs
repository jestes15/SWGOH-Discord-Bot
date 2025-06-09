use crate::commands::reqwest_api;
use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::{
    command::CommandOptionType,
    interaction::application_command::{ApplicationCommandInteraction, CommandDataOptionValue},
};

pub async fn get_user_data(command_interaction: &mut ApplicationCommandInteraction) -> String {
    // Get list of options from command interaction context
    let options = &command_interaction.data.options;

    // Get character option from the list of options
    let ally_code = options
        .get(0)
        .expect("Expected user option")
        .resolved
        .as_ref()
        .expect("Expected user object");

    let character = options
        .get(1)
        .expect("Expected user option")
        .resolved
        .as_ref()
        .expect("Expected user object");

    let ally_code_val: &i64;
    let character_val: &String;

    if let CommandDataOptionValue::Integer(value) = ally_code {
        ally_code_val = value;
    } else {
        return format!("{:?} is not a valid ally code", ally_code);
    }

    if let CommandDataOptionValue::String(value) = character {
        character_val = value;
    } else {
        return format!("{:?} is not a valid string", character);
    }

    let data = crate::commands::reqwest_api::reqwest_api::request_user_data(ally_code_val).await;

    let mut information: String = String::from("Character Not Found");

    for unit in data.units {
        if unit.data.base_id == character_val.as_str() {
            // Get vec of abilities
            let mut abilities: Vec<reqwest_api::user_schema::Ability> = vec![];
            for ability in unit.data.ability_data {
                let temp: String;

                let max_num_of_white_squares: i64 = ability.tier_max
                    - (if ability.is_zeta { 1 } else { 0 })
                    - (if ability.is_omicron { 1 } else { 0 });

                temp = format!(
                    "{}{}{}",
                    ":white_large_square:".repeat(max_num_of_white_squares as usize),
                    if ability.is_zeta && ability.has_zeta_learned {
                        ":blue_square:"
                    } else if ability.is_zeta {
                        ":black_large_square:"
                    } else {
                        ""
                    },
                    if ability.is_omicron && ability.has_omicron_learned {
                        ":yellow_square:"
                    } else if ability.is_omicron {
                        ":black_large_square:"
                    } else {
                        ""
                    }
                );

                let type_of_skill: String;
                if ability.id.contains("basicskill") {
                    type_of_skill = String::from("BASIC ");
                } else if ability.id.contains("uniqueskill") {
                    type_of_skill = String::from("UNIQUE ");
                } else if ability.id.contains("specialskill") {
                    type_of_skill = String::from("SPECIAL ");
                } else if ability.id.contains("leaderskill") {
                    type_of_skill = String::from("LEADER:");
                } else {
                    type_of_skill = String::from("UNKNOWN ");
                }

                abilities.push(reqwest_api::user_schema::Ability {
                    name: format!("`{:9}{:40}`", type_of_skill, ability.name),
                    level: ability.ability_tier,
                    progress: temp,
                    max_ab_level: ability.tier_max,
                });
            }

            information = format!("Name: {}\n", unit.data.name);
            information = format!("{information}Unit Level: `{}`\n", unit.data.level);
            information = format!("{information}Gear Level: `{}`\n", unit.data.gear_level);
            information = format!(
                "{information}Relic Level: `{}`\n",
                unit.data.relic_tier.unwrap_or(0)
            );
            information = format!("{information}Power: `{}`\n", unit.data.power);
            information = format!("{information}Rarity: `{}`\n", unit.data.rarity);
            information = format!("{information}Unit Speed: `{}`\n", unit.data.stats.n5);
            information = format!("{information}Unit Health: `{}`\n", unit.data.stats.n1);
            information = format!(
                "{information}Unit Protection: `{}`\n",
                unit.data.stats.n28.unwrap_or(0.0)
            );

            for ability in abilities {
                information = format!(
                    "{information}{}: {}  -  `{}/{}`\n",
                    ability.name, ability.progress, ability.level, ability.max_ab_level
                );
            }
        }
    }

    return information;
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("get_user_data")
        .description("Get user data from swgoh.gg api endpoint api/player/<id>")
        .create_option(|option| {
            option
                .name("allycode")
                .description("The ally code of the user you wish to get the character info for")
                .kind(CommandOptionType::Integer)
                .required(true)
        })
        .create_option(|option| {
            option
                .name("character")
                .description("The character ID you would like to view the states of")
                .kind(CommandOptionType::String)
                .required(true)
        })
}
