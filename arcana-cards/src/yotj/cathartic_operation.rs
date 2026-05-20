//! Cathartic Operation — `{3}{U}{B}` instant. "Return up to two
//! target creature cards from your graveyard to your hand, then seek
//! two noncreature, nonland cards."
//!
//! Up to two graveyard creature-card targets are returned to hand.
//!
//! GAP: "seek" (find a card matching a description from your library
//! at random without searching) has no catalog Effect; the two seeks
//! are not emitted.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Cathartic Operation");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}{B}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::black(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Return up to two target creature cards from your graveyard to your hand, then seek two noncreature, nonland cards.".into(),
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Card {
                    zone: Zone::Graveyard(0),
                    filter: ObjectFilter::creature(),
                },
                count: TargetCount::UpTo(2),
                controller: None,
            }],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(_state: &GameState, entry: &StackEntry, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "seek two noncreature, nonland cards" not expressible.
    let mut effects = Vec::new();
    for t in &entry.targets.targets {
        if let TargetChoice::Object(id) = t {
            effects.push(Effect::ReturnFromGraveyardToHand { target: *id });
        }
    }
    effects
}
