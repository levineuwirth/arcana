//! Angelic Ascension — `{1}{W}` instant. "Exile target creature or
//! planeswalker. Its controller creates a 4/4 white Angel creature
//! token with flying." The token goes to the exiled permanent's
//! controller, which the catalog cannot resolve from a target; only
//! the exile is emitted.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Angelic Ascension");
    let _angel = reg.interner_mut().intern("Angel");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Exile target creature or planeswalker. Its controller creates a 4/4 white Angel creature token with flying.".into(),
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
    // GAP: "Its controller creates a 4/4 Angel" — the token's
    // controller is the exiled permanent's controller, which cannot
    // be resolved from a target id with the catalog.
    vec![Effect::ExilePermanent { target: *id }]
}
