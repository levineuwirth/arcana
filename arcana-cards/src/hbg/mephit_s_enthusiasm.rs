//! Mephit's Enthusiasm — `{1}{R}` sorcery. "This sorcery deals 4
//! damage to target creature or planeswalker. If excess damage was
//! dealt this way, note that excess damage, then you get a one-time
//! boon with 'When you cast a creature spell, it perpetually gets
//! +X/+0, where X is the noted number.'"
//!
//! The 4 damage to a creature-or-planeswalker target is expressible.
//! The excess-damage noting + one-time perpetual-boon rider is not.

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Mephit's Enthusiasm");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "This sorcery deals 4 damage to target creature or planeswalker. If excess damage was dealt this way, note that excess damage, then you get a one-time boon with \"When you cast a creature spell, it perpetually gets +X/+0, where X is the noted number.\"".into(),
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Permanent(
                    ObjectFilter::permanent()
                        .with_types_any(TypeLine(TypeLine::CREATURE | TypeLine::PLANESWALKER)),
                ),
                count: TargetCount::Exactly(1),
                controller: None,
            }],
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
    let Some(target) = entry.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    // GAP: excess-damage noting + one-time perpetual-boon ("creature spells
    // perpetually get +X/+0") is not expressible. Emit only the 4 damage.
    vec![Effect::DealDamage {
        source: entry.source,
        target: DamageTarget::Object(*id),
        amount: 4,
    }]
}
