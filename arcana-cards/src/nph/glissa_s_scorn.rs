//! Glissa's Scorn — `{1}{G}` instant. "Destroy target artifact. Its
//! controller loses 1 life."
//! GAP: "its controller loses 1 life" requires knowing the controller of
//! the destroyed artifact. This can be approximated by targeting a player
//! as a second target, but the spec has one target. Best-effort: destroy
//! the artifact and emit a LoseLife for the target's controller via a
//! separate player target is not in the spec. Emitting just DestroyPermanent.
//! GAP: target's controller LoseLife not expressible with single target shape.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Glissa's Scorn");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Destroy target artifact. Its controller loses 1 life.".into(),
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::new().with_types(TypeLine::ARTIFACT.into()),
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
    // GAP: "its controller loses 1 life" — no API to look up the controller
    // of a permanent from a single-target shape at resolution time
    vec![Effect::DestroyPermanent { target: *id }]
}
