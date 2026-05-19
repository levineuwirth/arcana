//! Soul Reap — `{1}{B}` sorcery, "Destroy target nongreen creature. Its controller
//! loses 3 life if you've cast another black spell this turn."
//!
//! # GAP: nongreen-creature target filter — ObjectFilter has no color-exclusion
//! builder; falling back to plain creature filter.
//! # GAP: cast-another-black-spell-this-turn condition — no Effect::Conditional
//! condition variant for checking whether the controller cast another spell of a
//! given color this turn.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Soul Reap");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Destroy target nongreen creature. Its controller loses 3 life if you've cast another black spell this turn.".into(),
                // GAP: nongreen filter not expressible; using plain creature target
                target_requirements: vec![TargetRequirement::target_creature()],
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
    // GAP: cast-another-black-spell-this-turn condition — no Conditional variant
    // for checking spell-cast history this turn. Life-loss rider omitted.
    vec![Effect::DestroyPermanent { target: *id }]
}
