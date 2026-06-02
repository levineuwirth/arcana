//! Rogues' Gallery — `{2}{B}` sorcery. "For each color, return up to
//! one target creature card of that color from your graveyard to your
//! hand." One up-to-one creature-card target per color (W/U/B/R/G),
//! each constrained to its color via the inner `ObjectFilter`; the
//! resolver returns each chosen card to hand.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Rogues' Gallery");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };

    let color_req = |c: ColorSet| TargetRequirement {
        filter: TargetFilter::Card {
            zone: Zone::Graveyard(0),
            filter: ObjectFilter::creature().with_colors(c),
        },
        count: TargetCount::UpTo(1),
        controller: None,
    };

    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "For each color, return up to one target creature card of that color from your graveyard to your hand.".into(),
            target_requirements: vec![
                color_req(ColorSet::white()),
                color_req(ColorSet::blue()),
                color_req(ColorSet::black()),
                color_req(ColorSet::red()),
                color_req(ColorSet::green()),
            ],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(
    _state: &GameState,
    entry: &StackEntry,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    entry
        .targets
        .targets
        .iter()
        .filter_map(|t| match t {
            TargetChoice::Object(id) => {
                Some(Effect::ReturnFromGraveyardToHand { target: *id })
            }
            _ => None,
        })
        .collect()
}
