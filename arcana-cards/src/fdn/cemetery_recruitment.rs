//! Cemetery Recruitment — `{1}{B}` sorcery.
//! "Return target creature card from your graveyard to your hand. If it's a
//! Zombie card, draw a card."
//! GAP: conditional draw based on subtype check on the returned card.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetFilter, TargetCount, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};
use arcana_core::zones::Zone;
use arcana_core::targets::ObjectFilter;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Cemetery Recruitment");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Return target creature card from your graveyard to your hand. If it's a Zombie card, draw a card.".into(),
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Card { zone: Zone::Graveyard(0), filter: ObjectFilter::creature() },
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
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
    // Return to hand is expressed; the conditional Zombie draw is a GAP.
    // GAP: conditional draw if returned card has Zombie subtype
    vec![Effect::ReturnFromGraveyardToHand { target: *id }]
}
