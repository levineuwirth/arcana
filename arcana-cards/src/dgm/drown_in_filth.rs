//! Drown in Filth — `{B}{G}` sorcery. "Choose target creature. Mill
//! four cards, then that creature gets -1/-1 until end of turn for
//! each land card in your graveyard."

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Drown in Filth");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{B}{G}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::green(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Choose target creature. Mill four cards, then that creature gets -1/-1 until end of turn for each land card in your graveyard.".into(),
            target_requirements: vec![TargetRequirement::target_creature()],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(state: &GameState, entry: &StackEntry, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = entry.targets.targets.first() else {
        return Vec::new();
    };
    let lands = script::graveyard_matching(
        state,
        &ObjectFilter::permanent().with_types(TypeLine::LAND.into()),
        entry.controller,
        entry.controller,
    ) as i32;
    vec![
        Effect::Mill {
            player: entry.controller,
            count: 4,
        },
        Effect::Pump {
            target: *id,
            power: -lands,
            toughness: -lands,
            duration: Duration::EndOfTurn,
            keywords: vec![],
        },
    ]
}
