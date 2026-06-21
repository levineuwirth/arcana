//! Consuming Blob — `{3}{G}{G}` */*+1 Ooze.
//! * "Consuming Blob's power is equal to the number of card types among
//!   cards in your graveyard and its toughness is equal to that number
//!   plus 1." — a characteristic-defining ability (*/*+1). The base
//!   P/T are transcribed as Star / StarPlus(1); the runtime computation
//!   of the value is engine CDA machinery not expressible here. GAP.
//! * "At the beginning of your end step, create a green Ooze creature
//!   token with [the same CDA]." The token is minted as a */*+1 Ooze;
//!   the token's CDA computation is GAP'd for the same reason.

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::targets::ControllerConstraint;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Consuming Blob");
    let ooze = reg.interner_mut().intern("Ooze");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(ooze);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Star),
        toughness: Some(PtValue::StarPlus(1)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::End,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: make_ooze_token,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn make_ooze_token(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let ooze = reg.interner().lookup("Ooze").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(ooze);
    // GAP: the token's "*/*+1 = card types in your graveyard" CDA is not
    // attachable to a minted token in this surface — minted with the
    // Star / StarPlus(1) base P/T but no live computation.
    vec![Effect::CreateToken {
        controller: trig.controller,
        token: TokenDefinition {
            name: ooze,
            colors: ColorSet::green(),
            types: TypeLine::CREATURE.into(),
            subtypes,
            power: Some(PtValue::Star),
            toughness: Some(PtValue::StarPlus(1)),
            keywords: vec![],
            abilities: vec![],
        },
    }]
}
