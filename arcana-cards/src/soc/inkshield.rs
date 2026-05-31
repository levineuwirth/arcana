//! Inkshield — `{3}{W}{B}` instant. "Prevent all combat damage that would
//! be dealt to you this turn. For each 1 damage prevented this way, create a
//! 2/1 white and black Inkling creature token with flying."
//!
//! The combat-damage prevention to you is expressible via
//! `PreventDamageFrom` (creature sources -> the player, this turn). The
//! token-creation rider, however, scales off the AMOUNT of damage actually
//! prevented at resolution — a quantity that is computed by the replacement
//! effect later in the turn, not available to the resolver via any
//! `script::*` helper. There is no primitive to mint tokens "per damage
//! prevented", so that half is GAPed rather than emitted with a wrong fixed
//! count.

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
    let name = reg.interner_mut().intern("Inkshield");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{W}{B}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::black(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Prevent all combat damage that would be dealt to you this turn. For each 1 damage prevented this way, create a 2/1 white and black Inkling creature token with flying.".into(),
            target_requirements: vec![],
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
    // GAP: no primitive mints "a token for each 1 damage prevented this way";
    // that count is determined by the prevention replacement later in the
    // turn and is not available to the resolver. Emit only the expressible
    // prevention of all (combat) damage dealt to you this turn.
    vec![Effect::PreventDamageFrom {
        source_filter: ObjectFilter::permanent(),
        target_filter: TargetFilter::Player,
        amount: None,
        duration: ReplacementDuration::EndOfTurn,
    }]
}
