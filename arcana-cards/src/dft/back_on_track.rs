//! Back on Track — `{4}{B}` sorcery.
//! "Return target creature or Vehicle card from your graveyard to the battlefield. Create a 1/1
//! colorless Pilot creature token with 'This token saddles Mounts and crews Vehicles as though
//! its power were 2 greater.'"
//!
//! # GAP: VehicleSubtypeGraveyardTarget — TargetFilter::Card graveyard targeting only has
//! ObjectFilter::creature() shorthand; targeting "creature or Vehicle" requires a Vehicle subtype
//! filter that is not available.
//! # GAP: TokenWithSpecialActivatedAbility — TokenDefinition.abilities is Vec but the "saddles
//! Mounts and crews Vehicles as though its power were 2 greater" rider ability cannot be expressed
//! with the current TokenDefinition fields.
//! Best effort: reanimate a creature from graveyard; Pilot token creation is dropped.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetFilter, TargetCount, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Back on Track");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Return target creature or Vehicle card from your graveyard to the battlefield. Create a 1/1 colorless Pilot creature token with 'This token saddles Mounts and crews Vehicles as though its power were 2 greater.'".into(),
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Card {
                        zone: Zone::Graveyard(0),
                        filter: arcana_core::targets::ObjectFilter::creature(),
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
    // GAP: VehicleSubtypeGraveyardTarget — cannot filter for "creature or Vehicle" card in GY.
    // GAP: TokenWithSpecialActivatedAbility — Pilot token's saddle/crew ability not expressible.
    vec![Effect::ReturnFromGraveyardToBattlefield { target: *id }]
}
