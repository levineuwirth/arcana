//! Finishing Move — `{2}{G}` sorcery. "You get {TK}{TK}, then you may put a
//! sticker on a nonland permanent you own. Target creature you control deals
//! damage equal to its power to target creature you don't control."
//!
//! GAP: {TK} ticket counters — no Effect variant for ticket counters.
//! GAP: "put a sticker on a nonland permanent" — no Effect variant for stickers.
//! GAP: one-directional "creature deals damage equal to its power" — Fight is
//! mutual; using Fight as best-effort approximation.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Finishing Move");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "You get {TK}{TK}, then you may put a sticker on a nonland permanent you own. Target creature you control deals damage equal to its power to target creature you don't control.".into(),
                target_requirements: vec![
                    TargetRequirement::target_creature(),
                    TargetRequirement::target_creature(),
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
    // GAP: {TK} ticket counters — no Effect variant
    // GAP: "put a sticker" — no Effect variant
    // GAP: one-directional power-based damage; using Fight as approximation
    let targets = &entry.targets.targets;
    if targets.len() < 2 { return Vec::new(); }
    let TargetChoice::Object(id_a) = &targets[0] else { return Vec::new(); };
    let TargetChoice::Object(id_b) = &targets[1] else { return Vec::new(); };
    vec![Effect::Fight { a: *id_a, b: *id_b }]
}
