//! Arm the Cathars — `{1}{W}{W}` sorcery. "Until end of turn, target
//! creature gets +3/+3, up to one other target creature gets +2/+2, and
//! up to one other target creature gets +1/+1. Those creatures gain
//! vigilance until end of turn."

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{
    TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Arm the Cathars");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Until end of turn, target creature gets +3/+3, up to one other target creature gets +2/+2, and up to one other target creature gets +1/+1. Those creatures gain vigilance until end of turn.".into(),
            target_requirements: vec![
                TargetRequirement {
                    filter: TargetFilter::Creature,
                    count: TargetCount::Exactly(1),
                    controller: None,
                },
                TargetRequirement {
                    filter: TargetFilter::Creature,
                    count: TargetCount::UpTo(1),
                    controller: None,
                },
                TargetRequirement {
                    filter: TargetFilter::Creature,
                    count: TargetCount::UpTo(1),
                    controller: None,
                },
            ],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(_state: &GameState, entry: &StackEntry, _reg: &CardRegistry) -> Vec<Effect> {
    let bonuses = [3, 2, 1];
    entry
        .targets
        .targets
        .iter()
        .enumerate()
        .filter_map(|(i, t)| match t {
            TargetChoice::Object(id) => {
                let b = *bonuses.get(i).unwrap_or(&1);
                Some(Effect::Pump {
                    target: *id,
                    power: b,
                    toughness: b,
                    duration: Duration::EndOfTurn,
                    keywords: vec![KeywordAbility::Vigilance],
                })
            }
            _ => None,
        })
        .collect()
}
