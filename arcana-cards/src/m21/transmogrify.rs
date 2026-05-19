//! Transmogrify — `{3}{R}` sorcery. "Exile target creature. That creature's controller reveals
//! cards from the top of their library until they reveal a creature card. That player puts that
//! card onto the battlefield, then shuffles the rest into their library."
//! GAP: exile creature then reveal-until-creature-and-put-onto-battlefield for that creature's
//!      controller (not the spell's controller) not in catalog.
//! Best effort: exile target creature; tutor portion is GAP.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Transmogrify");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Exile target creature. That creature's controller reveals cards from the top of their library until they reveal a creature card. That player puts that card onto the battlefield, then shuffles the rest into their library.".into(),
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
    // GAP: reveal-until-creature + put onto battlefield for that creature's controller
    vec![Effect::ExilePermanent { target: *id }]
}
