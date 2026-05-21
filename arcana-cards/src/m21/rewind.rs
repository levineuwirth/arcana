//! Rewind — `{2}{U}{U}` instant. "Counter target spell. Untap up to four
//! lands."
//!
//! Counter expressible; 'untap up to four lands' is a free-pick variable
//! choice not in the target_requirements model (one TargetRequirement here is
//! the spell). GAP the untap-up-to-four half — we lack a way to bind 'up to
//! N' lands as additional spell targets here.

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
    let name = reg.interner_mut().intern("Rewind");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Counter target spell. Untap up to four lands.".into(),
                target_requirements: vec![
                    TargetRequirement {
                        filter: TargetFilter::Spell(ObjectFilter::default()),
                        count: TargetCount::Exactly(1),
                        controller: None,
                    },
                    TargetRequirement {
                        filter: TargetFilter::Permanent(
                            ObjectFilter::permanent().with_types(TypeLine::LAND.into()),
                        ),
                        count: TargetCount::UpTo(4),
                        controller: None,
                    },
                ],
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
    let mut effects: Vec<Effect> = Vec::new();
    for t in entry.targets.targets.iter() {
        if let TargetChoice::Object(id) = t {
            // First target is the spell, remaining are lands. The engine resolves the
            // counter on a stack id and untap on a battlefield id — both use Object(id).
            if effects.is_empty() {
                effects.push(Effect::Counter { target: *id });
            } else {
                effects.push(Effect::Untap { target: *id });
            }
        }
    }
    effects
}
