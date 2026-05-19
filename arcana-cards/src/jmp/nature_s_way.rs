//! Nature's Way — `{1}{G}` sorcery, "Target creature you control gains
//! vigilance and trample until end of turn. It deals damage equal to its power
//! to target creature you don't control."

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement};
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
                            ObjectFilter::creature().controlled_by(ControllerConstraint::You)
                        ),
                        count: TargetCount::Exactly(1),
                        controller: None,
                    },
                    TargetRequirement {
                        filter: TargetFilter::Permanent(
                            ObjectFilter::creature().controlled_by(ControllerConstraint::Opponent)
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
    let mut targets = entry.targets.targets.iter();
    let Some(t0) = targets.next() else { return Vec::new(); };
    let Some(t1) = targets.next() else { return Vec::new(); };
    let TargetChoice::Object(src_id) = t0 else { return Vec::new(); };
    let TargetChoice::Object(dst_id) = t1 else { return Vec::new(); };
    let power = script::power_of(state, *src_id);
    vec![
        Effect::Pump {
            target: *src_id,
            power: 0,
            toughness: 0,
            duration: Duration::EndOfTurn,
            keywords: vec![KeywordAbility::Vigilance, KeywordAbility::Trample],
        },
        Effect::DealDamage {
            source: entry.source,
            target: arcana_core::events::DamageTarget::Object(*dst_id),
            amount: power.max(0) as u32,
        },
    ]
}
