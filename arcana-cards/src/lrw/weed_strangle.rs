//! Weed Strangle — `{3}{B}{B}` sorcery, "Destroy target creature.
//! Clash with an opponent. If you win, you gain life equal to that
//! creature's toughness."
//!
//! The destroy is fully expressible. The Clash mechanic (each
//! clashing player reveals their top card and compares mana values)
//! and the win-gated life gain riding on it have no catalog effect,
//! so that rider is GAPed rather than invented.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Weed Strangle");
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
                text: "Destroy target creature. Clash with an opponent. If you win, you gain life equal to that creature's toughness.".into(),
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
    // GAP: Clash mechanic (reveal-and-compare-mana-value contest) and
    // its win-gated "gain life equal to that creature's toughness"
    // rider are not expressible with any catalog Effect variant.
    vec![Effect::DestroyPermanent { target: *id }]
}
