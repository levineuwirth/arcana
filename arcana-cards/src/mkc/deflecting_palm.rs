//! Deflecting Palm — `{R}{W}` instant. "The next time a source of your
//! choice would deal damage to you this turn, prevent that damage. If
//! damage is prevented this way, Deflecting Palm deals that much damage
//! to that source's controller." The engine does not model
//! prevent-and-redirect-by-chosen-source; only the simple
//! `PreventDamage` shield is available, which doesn't capture the
//! reflected-damage rider. We emit a best-effort blanket prevent shield
//! on you and gap the redirect.

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::replacement::ReplacementDuration;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Deflecting Palm");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{R}{W}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::white(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "The next time a source of your choice would deal damage to you this turn, prevent that damage. If damage is prevented this way, Deflecting Palm deals that much damage to that source's controller.".into(),
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
    // GAP: cannot model "next damage from a chosen source" or the
    // redirect-as-damage rider; approximate with a player-targeted
    // shield (no amount cap matches a single source either).
    vec![Effect::PreventDamage {
        target: DamageTarget::Player(entry.controller),
        amount: None,
        duration: ReplacementDuration::EndOfTurn,
    }]
}
