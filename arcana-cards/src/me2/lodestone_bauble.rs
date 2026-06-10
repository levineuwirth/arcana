//! Lodestone Bauble — `{0}` artifact.
//! "{1}, {T}, Sacrifice this artifact: Put up to four target basic land
//! cards from a player's graveyard on top of their library in any order.
//! That player draws a card at the beginning of the next turn's upkeep."
//! Up-to-four graveyard-card targets each go on top of their library; the
//! 'in any order' choice and the delayed upkeep draw are documented gaps.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Lodestone Bauble");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{0}").expect("valid cost")),
        colors: ColorSet::new(),
        types: TypeLine::ARTIFACT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(
            ActivatedAbilityDef {
                text: "{1}, {T}, Sacrifice this artifact: Put up to four \
                       target basic land cards from a player's graveyard \
                       on top of their library in any order. That player \
                       draws a card at the beginning of the next turn's \
                       upkeep."
                    .into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{1}").expect("valid cost"),
                    tap: true,
                    sacrifice: true,
                    ..ActivationCost::default()
                },
                // GAP: 'a player's graveyard' — the Card target zone is
                // fixed at registration; Zone::Graveyard(0) is the
                // demonstrated form.
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Card {
                        zone: Zone::Graveyard(0),
                        filter: ObjectFilter::new()
                            .with_types(TypeLine::LAND.into())
                            .with_supertypes(
                                SupertypeSet::new()
                                    .with(SupertypeSet::BASIC),
                            ),
                    },
                    count: TargetCount::UpTo(4),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: stack_lands_on_library,
            },
        ),
    )
}

fn stack_lands_on_library(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: 'in any order' — the controller's ordering choice is not
    // modeled (cards go on top in target order). GAP: 'That player draws
    // a card at the beginning of the next turn's upkeep' — DelayedAction
    // has no Draw action and no upkeep timing.
    ctx.targets
        .targets
        .iter()
        .filter_map(|t| match t {
            TargetChoice::Object(id) => {
                Some(Effect::PutOnTopOfLibrary { target: *id })
            }
            _ => None,
        })
        .collect()
}
