//! Viconia, Drow Apostate — `{2}{B}` 2/3 Legendary Elf Cleric.
//!
//! Oracle:
//! * At the beginning of your upkeep, if there are four or more creature
//!   cards in your graveyard, return a creature card at random from your
//!   graveyard to your hand.
//! * Choose a Background.
//!
//! The upkeep trigger's intervening-if (4+ creature cards in graveyard) is
//! modeled. Its effect — return a creature card AT RANDOM to hand — has no
//! primitive (the only graveyard→hand return is targeted, not random), so it
//! is GAP'd. "Choose a Background" is a deck-building rule with no engine
//! representation and is GAP'd.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, ObjectId};
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::conditions;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, PlayerId, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Viconia, Drow Apostate");
    let elf = reg.interner_mut().intern("Elf");
    let cleric = reg.interner_mut().intern("Cleric");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elf);
    subtypes.0.insert(cleric);

    // GAP: keyword/static — "Choose a Background" (Commander deck-building rule).

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::Upkeep,
                    whose: ControllerConstraint::You,
                },
                intervening_if: Some(if_four_creature_cards_in_gy),
                effect: return_random_creature,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn if_four_creature_cards_in_gy(
    s: &GameState,
    _src: ObjectId,
    you: PlayerId,
    _reg: &CardRegistry,
) -> bool {
    conditions::graveyard_matching_at_least(s, you, &ObjectFilter::creature(), 4)
}

fn return_random_creature(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "return a creature card at random from your graveyard to your hand"
    // — no random graveyard→hand return primitive (ReturnFromGraveyardToHand is
    // targeted; "at random" is not a chosen target).
    Vec::new()
}
