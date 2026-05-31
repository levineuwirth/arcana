//! Twisted Justice — `{4}{U}{B}` sorcery. "Target player sacrifices a
//! creature of their choice. You draw cards equal to that creature's
//! power."
//!
//! The sacrifice is expressible (target player sacrifices one
//! creature). The follow-up draw is dynamic — its count equals the
//! power of the specific creature the target player chooses to
//! sacrifice, which is not known at resolver-build time and is not
//! recoverable from the Sacrifice effect. Per the dynamic-amount rule a
//! literal would be a wrong card, so the draw is GAP-ed rather than
//! hardcoded.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Twisted Justice");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{U}{B}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::black(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Target player sacrifices a creature of their choice. You draw cards equal to that creature's power.".into(),
                target_requirements: vec![TargetRequirement::target_player()],
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
    let TargetChoice::Player(p) = target else { return Vec::new(); };
    // GAP: "draw cards equal to that creature's power" — the count
    // depends on the power of the specific creature the target player
    // chooses to sacrifice, which the Sacrifice effect does not surface
    // back to the resolver. Only the sacrifice is expressed.
    vec![Effect::Sacrifice {
        player: *p,
        filter: ObjectFilter::creature(),
        count: 1,
    }]
}
