//! Contest of Claws — `{1}{G}` sorcery. "Target creature you control
//! deals damage equal to its power to another target creature. If
//! excess damage was dealt this way, discover X..."
//!
//! The "deal power-damage to another creature" portion is
//! expressible; the excess-damage discover rider has no catalog
//! Effect and is gapped.

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
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
    let name = reg.interner_mut().intern("Contest of Claws");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Target creature you control deals damage equal to its power to another target creature. If excess damage was dealt this way, discover X, where X is that excess damage.".into(),
            target_requirements: vec![
                TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::creature().controlled_by(ControllerConstraint::You),
                    ),
                    count: TargetCount::Exactly(1),
                    controller: None,
                },
                TargetRequirement {
                    filter: TargetFilter::Permanent(ObjectFilter::creature()),
                    count: TargetCount::Exactly(1),
                    controller: None,
                },
            ],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(state: &GameState, entry: &StackEntry, _reg: &CardRegistry) -> Vec<Effect> {
    let mut it = entry.targets.targets.iter();
    let (Some(TargetChoice::Object(src)), Some(TargetChoice::Object(victim))) =
        (it.next(), it.next())
    else {
        return Vec::new();
    };
    let amount = script::power_of(state, *src).max(0) as u32;
    // GAP: excess-damage "discover X" rider has no catalog Effect.
    vec![Effect::DealDamage {
        source: *src,
        target: DamageTarget::Object(*victim),
        amount,
    }]
}
