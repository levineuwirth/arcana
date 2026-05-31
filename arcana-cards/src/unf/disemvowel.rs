//! Disemvowel — `{3}{B}{B}` sorcery. "Destroy target creature. That
//! creature's controller loses 1 life for each unique vowel in the
//! creature's name. (The vowels are A, E, I, O, U, and Y.)"
//!
//! The destroy is expressible. The follow-up life loss scales with the
//! number of distinct vowels in the destroyed creature's NAME — a
//! string-introspection quantity that no `script::*` helper exposes
//! (there is no name/char access in the permitted scripting surface).
//! We emit the destroy faithfully and GAP the rider rather than
//! hardcode a wrong literal life loss.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Disemvowel");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Destroy target creature. That creature's controller loses 1 life for each unique vowel in the creature's name. (The vowels are A, E, I, O, U, and Y.)".into(),
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
    // GAP: life loss "for each unique vowel in the creature's name" needs
    // name-string introspection, which no script:: helper exposes. Emitting
    // only the destroy; the variable life-loss rider is omitted rather than
    // hardcoded.
    vec![Effect::DestroyPermanent { target: *id }]
}
