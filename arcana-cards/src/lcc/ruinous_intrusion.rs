//! Ruinous Intrusion — `{3}{G}` instant. "Exile target artifact or
//! enchantment. Put X +1/+1 counters on target creature you control,
//! where X is the mana value of the permanent exiled this way."
//!
//! The exile of the first target is expressible; the +1/+1 counters on
//! the second target are GAP'd because X (the mana value of the exiled
//! permanent) cannot be computed from the available script helpers.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Ruinous Intrusion");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Exile target artifact or enchantment. Put X +1/+1 counters on target creature you control, where X is the mana value of the permanent exiled this way.".into(),
                target_requirements: vec![
                    TargetRequirement {
                        filter: TargetFilter::Permanent(
                            ObjectFilter::new()
                                .with_types_any(TypeLine(TypeLine::ARTIFACT | TypeLine::ENCHANTMENT)),
                        ),
                        count: TargetCount::Exactly(1),
                        controller: None,
                    },
                    TargetRequirement {
                        filter: TargetFilter::Permanent(
                            ObjectFilter::creature().controlled_by(ControllerConstraint::You),
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
    _state: &GameState,
    entry: &StackEntry,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(art)) = entry.targets.targets.first() else {
        return Vec::new();
    };
    // GAP: X = the mana value of the permanent exiled this way. No script
    // helper exposes a target permanent's mana value, so the second-target
    // +1/+1 counter clause (count = X) cannot be computed. The exile of the
    // artifact/enchantment is emitted; the counter placement is GAP'd rather
    // than hardcoded to a wrong literal.
    vec![Effect::ExilePermanent { target: *art }]
}
