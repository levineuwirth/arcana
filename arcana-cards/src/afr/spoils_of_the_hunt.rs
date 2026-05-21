//! Spoils of the Hunt — `{2}{G}` instant. "Target creature you
//! control gets +1/+0 until end of turn for each mana from a
//! Treasure that was spent to cast this spell. Then that creature
//! deals damage equal to its power to target creature an opponent
//! controls."

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
    let name = reg.interner_mut().intern("Spoils of the Hunt");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Target creature you control gets +1/+0 until end of turn for each mana from a Treasure that was spent to cast this spell. Then that creature deals damage equal to its power to target creature an opponent controls.".into(),
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
                        ObjectFilter::creature()
                            .controlled_by(ControllerConstraint::Opponent),
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
    let Some(TargetChoice::Object(own)) = entry.targets.targets.first() else {
        return Vec::new();
    };
    let Some(TargetChoice::Object(foe)) = entry.targets.targets.get(1) else {
        return Vec::new();
    };
    // "deals damage equal to its power" — read the creature's current
    // power and have it deal that much to the opponent's creature.
    let power = script::power_of(state, *own).max(0) as u32;
    vec![Effect::DealDamage {
        source: *own,
        target: DamageTarget::Object(*foe),
        amount: power,
    }]
    // GAP: "+1/+0 for each mana from a Treasure spent to cast this" —
    // mana-source provenance (Treasure-spent mana) is not observable,
    // so the pump amount cannot be computed; the pump is omitted.
}
