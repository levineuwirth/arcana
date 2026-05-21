//! Nature's Way — `{1}{G}` sorcery. "Target creature you control
//! gains vigilance and trample until end of turn. It deals damage
//! equal to its power to target creature you don't control." Use a
//! two-target shape (you+opponent), then Fight (since each fights deals
//! its power to the other — but Nature's Way is asymmetric: only the
//! first deals damage to the second). Best modelable: emit Pump grant
//! + a DealDamage from your creature using power_of.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::events::DamageTarget;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
};
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
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Target creature you control gains vigilance and trample until end of turn. It deals damage equal to its power to target creature you don't control.".into(),
                target_requirements: vec![
                    TargetRequirement {
                        filter: TargetFilter::Permanent(
                            ObjectFilter::creature().controlled_by(ControllerConstraint::You),
                        ),
                        count: TargetCount::Exactly(1),
                        controller: None,
                    },
                    TargetRequirement {
                        filter: TargetFilter::Permanent(
                            ObjectFilter::creature().controlled_by(ControllerConstraint::Opponent),
                        ),
                        count: TargetCount::Exactly(1),
                        controller: None,
                    },
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
    let Some(first) = entry.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(self_id) = first else { return Vec::new(); };
    let self_id = *self_id;
    let Some(second) = entry.targets.targets.get(1) else { return Vec::new(); };
    let TargetChoice::Object(other_id) = second else { return Vec::new(); };
    let amount = script::power_of(state, self_id).max(0) as u32;
    vec![
        Effect::GrantKeyword {
            target: self_id,
            keyword: KeywordAbility::Vigilance,
            duration: Duration::EndOfTurn,
        },
        Effect::GrantKeyword {
            target: self_id,
            keyword: KeywordAbility::Trample,
            duration: Duration::EndOfTurn,
        },
        Effect::DealDamage {
            source: entry.source,
            target: DamageTarget::Object(*other_id),
            amount,
        },
    ]
}
