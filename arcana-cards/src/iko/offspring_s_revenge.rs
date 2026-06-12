//! Offspring's Revenge — `{2}{R}{W}{B}` enchantment (Streets of New
//! Capenna, 2022). "At the beginning of combat on your turn, exile
//! target red, white, or black creature card from your graveyard. Create
//! a token that's a copy of that card, except it's 1/1. It gains haste
//! until your next turn."
//!
//! The combat trigger and the graveyard-card target are wired
//! faithfully; the copy is minted via `Effect::CopyPermanent` BEFORE the
//! exile (the exiled card is no longer addressable). Documented GAPs:
//! the "except it's 1/1" override and the haste-until-your-next-turn
//! grant can't reach the freshly minted token (its id is unknown at
//! resolve time), and the R/W/B color constraint uses the filter's
//! `with_colors` color-set match.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount,
    TargetFilter, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Phase;
use arcana_core::types::{CardId, ColorSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Offspring's Revenge");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}{W}{B}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::red() | ColorSet::white(),
        types: TypeLine::ENCHANTMENT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(
            TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::PhaseBegins {
                    phase: Phase::Combat,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: revenge_copy,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Card {
                        zone: Zone::Graveyard(0),
                        filter: ObjectFilter::creature().with_colors(
                            ColorSet::red()
                                | ColorSet::white()
                                | ColorSet::black(),
                        ),
                    },
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
            },
        ),
    )
}

/// "exile target red, white, or black creature card from your graveyard.
/// Create a token that's a copy of that card, except it's 1/1. It gains
/// haste until your next turn."
fn revenge_copy(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = trig.targets.targets.first() else {
        return Vec::new();
    };
    // GAP: "except it's 1/1" and "It gains haste until your next turn" —
    // the minted token's id is unknown at resolve time, so the P/T
    // override and the haste grant cannot be applied (the
    // until-your-next-turn duration itself exists). Copy is minted
    // before the exile since the exiled card is no longer addressable.
    vec![
        Effect::CopyPermanent { target: *id },
        Effect::ExileFromGraveyard { target: *id },
    ]
}
