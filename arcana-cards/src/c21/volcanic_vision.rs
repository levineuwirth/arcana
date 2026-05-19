//! Volcanic Vision — `{5}{R}{R}` sorcery. "Return target instant or
//! sorcery card from your graveyard to your hand. Volcanic Vision
//! deals damage equal to that card's mana value to each creature your
//! opponents control. Exile Volcanic Vision."
//!
//! # GAP: dynamic damage = MV of returned card; self-exile on resolution
//! ReturnFromGraveyardToHand is expressible. Reading the returned
//! card's mana value and dealing that as damage to each opponent's
//! creature requires dynamic state read — not available. Emitting
//! return only.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Volcanic Vision");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Return target instant or sorcery card from your graveyard to your hand. Volcanic Vision deals damage equal to that card's mana value to each creature your opponents control. Exile Volcanic Vision.".into(),
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Card {
                        zone: Zone::Graveyard(0),
                        filter: ObjectFilter::new()
                            .with_types_any(TypeLine(TypeLine::INSTANT | TypeLine::SORCERY)),
                    },
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
    // GAP: dynamic damage = returned card's MV applied to each opponent creature; self-exile
    vec![Effect::ReturnFromGraveyardToHand { target: *id }]
}
