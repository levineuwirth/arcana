//! Death's Caress — `{3}{B}{B}` sorcery. "Destroy target creature. If
//! that creature was a Human, you gain life equal to its toughness."
//! The post-destroy 'was a Human' subtype memory + its toughness needs
//! a pre-destroy capture which the resolver can do via script helpers
//! BEFORE emitting DestroyPermanent.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Death's Caress");
    let _human = reg.interner_mut().intern("Human");
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
                text: "Destroy target creature. If that creature was a Human, you gain life equal to its toughness.".into(),
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
    // GAP: 'was a Human' subtype predicate on the targeted creature
    // (no per-object subtype query in script helpers); the toughness
    // capture (script::toughness_of) would work for the amount, but
    // the conditional gating is missing — emit destroy only.
    let _ = script::toughness_of;
    vec![Effect::DestroyPermanent { target: *id }]
}
