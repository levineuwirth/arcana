//! Nature's Way — `{1}{G}` sorcery. "Target creature you control gains
//! vigilance and trample until end of turn. It deals damage equal to
//! its power to target creature you don't control."

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::layers::Duration;
use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Nature's Way");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Target creature you control gains vigilance and trample until end of turn. It deals damage equal to its power to target creature you don't control.".into(),
            target_requirements: vec![
                TargetRequirement::target_creature(),
                TargetRequirement::target_creature(),
            ],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(
    state: &GameState,
    entry: &StackEntry,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(mine)) = entry.targets.targets.first() else {
        return Vec::new();
    };
    let Some(TargetChoice::Object(theirs)) = entry.targets.targets.get(1) else {
        return Vec::new();
    };
    let power = script::power_of(state, *mine).max(0) as u32;
    vec![
        Effect::GrantKeyword {
            target: *mine,
            keyword: KeywordAbility::Vigilance,
            duration: Duration::EndOfTurn,
        },
        Effect::GrantKeyword {
            target: *mine,
            keyword: KeywordAbility::Trample,
            duration: Duration::EndOfTurn,
        },
        Effect::DealDamage {
            source: *mine,
            target: DamageTarget::Object(*theirs),
            amount: power,
        },
    ]
}
