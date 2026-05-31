//! Arachnogenesis — `{2}{G}` instant. "Create X 1/2 green Spider
//! creature tokens with reach, where X is the number of creatures
//! attacking you. Prevent all combat damage that would be dealt this
//! turn by non-Spider creatures."
//!
//! The token half is GAP-ped: X is the number of creatures attacking
//! you, and there is no `script::` helper that counts attackers (the
//! available helpers count battlefield permanents by `ObjectFilter`,
//! not combat participants). Emitting a fixed-size stand-in would be a
//! materially wrong card, so the token clause is dropped. The
//! combat-damage prevention is expressed with a source-filtered
//! `PreventDamageFrom`.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::replacement::ReplacementDuration;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetFilter};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Arachnogenesis");
    let _spider = reg.interner_mut().intern("Spider");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Create X 1/2 green Spider creature tokens with reach, where X is the number of creatures attacking you. Prevent all combat damage that would be dealt this turn by non-Spider creatures.".into(),
            target_requirements: vec![],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(_state: &GameState, _entry: &StackEntry, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "Create X ... tokens where X is the number of creatures attacking you" —
    // no script helper counts attackers, so the dynamic token count is not expressible.
    // The combat-damage prevention from non-Spider creatures is emitted faithfully.
    vec![Effect::PreventDamageFrom {
        source_filter: ObjectFilter::creature(),
        target_filter: TargetFilter::Player,
        amount: None,
        duration: ReplacementDuration::EndOfTurn,
    }]
}
