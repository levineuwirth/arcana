//! Soul Sear — `{2}{R}` instant. "Soul Sear deals 5 damage to target
//! creature or planeswalker. That permanent loses indestructible until end
//! of turn." The removal installs a targeted
//! `ContinuousEffect::remove_keyword` (Layer 6); within-layer
//! timestamps mean it beats earlier indestructible grants.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::{ContinuousEffect, Duration};
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Soul Sear");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Soul Sear deals 5 damage to target creature or planeswalker. That permanent loses indestructible until end of turn.".into(),
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Permanent(ObjectFilter::permanent().with_types_any(
                    TypeLine(TypeLine::CREATURE | TypeLine::PLANESWALKER),
                )),
                count: TargetCount::Exactly(1),
                controller: None,
            }],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(_state: &GameState, entry: &StackEntry, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = entry.targets.targets.first() else {
        return Vec::new();
    };
    // "That permanent loses indestructible until end of turn" — the
    // removal is installed BEFORE the damage so a previously-granted
    // indestructible doesn't save the permanent from this damage's
    // destruction SBA.
    vec![
        Effect::InstallContinuousEffect {
            effect: ContinuousEffect::remove_keyword(
                entry.source,
                *id,
                KeywordAbility::Indestructible,
                Duration::EndOfTurn,
            ),
        },
        Effect::DealDamage {
            source: entry.source,
            target: DamageTarget::Object(*id),
            amount: 5,
        },
    ]
}
