//! Involuntary Cooldown — `{3}{U}` sorcery. "Tap up to two target
//! artifacts and/or creatures. Put two stun counters on each of them."

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

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Involuntary Cooldown");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Tap up to two target artifacts and/or creatures. Put two stun counters on each of them. (If a permanent with a stun counter would become untapped, remove one from it instead.)".into(),
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Permanent(ObjectFilter::new().with_types_any(
                    TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE).into(),
                )),
                count: TargetCount::UpTo(2),
                controller: None,
            }],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(_state: &GameState, entry: &StackEntry, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: stun counters are not a demonstrated CounterKind variant;
    // emitting only the taps (the counter half is omitted rather than
    // inventing a variant).
    let mut effects = Vec::new();
    for t in &entry.targets.targets {
        let TargetChoice::Object(id) = t else { continue };
        effects.push(Effect::Tap { target: *id });
    }
    effects
}
