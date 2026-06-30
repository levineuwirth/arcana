//! Supreme Verdict — `{1}{W}{W}{U}` sorcery. "This spell can't be countered.
//! Destroy all creatures." A targetless board wipe; the destroy list is built
//! from the battlefield at resolution (indestructible is respected by
//! `DestroyPermanent`).

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Supreme Verdict");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}{W}{U}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::blue(),
        types: TypeLine::SORCERY.into(),
        cant_be_countered: true,
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "This spell can't be countered. Destroy all creatures.".into(),
            target_requirements: Vec::new(),
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(
    state: &GameState,
    _entry: &StackEntry,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    state
        .objects
        .objects_in_zone(Zone::Battlefield)
        .filter(|o| o.is_creature())
        .map(|o| Effect::DestroyPermanent { target: o.id })
        .collect()
}
