//! Path of Peace — `{3}{W}` sorcery. "Destroy target creature. Its owner
//! gains 4 life."
//!
//! GAP: "Its owner gains 4 life" — gaining life for the target's owner
//! (not necessarily the spell's controller) requires looking up the
//! target permanent's owner, which is not directly accessible via the
//! script helpers. Emitting destroy + life gain for entry.controller
//! as best effort; owner lookup is the gap.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Path of Peace");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Destroy target creature. Its owner gains 4 life.".into(),
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
    // GAP: life gain goes to the creature's owner, not entry.controller; owner lookup unavailable
    vec![
        Effect::DestroyPermanent { target: *id },
        Effect::GainLife { player: entry.controller, amount: 4 },
    ]
}
