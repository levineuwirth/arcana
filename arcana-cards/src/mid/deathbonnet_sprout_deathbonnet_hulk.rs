//! Deathbonnet Sprout // Deathbonnet Hulk — `{G}` 1/1 green Fungus.
//!
//! Front face (Deathbonnet Sprout):
//!   At the beginning of your upkeep, mill a card. Then if there are three or more
//!   creature cards in your graveyard, transform this creature.
//!
//! Back face (Deathbonnet Hulk):
//!   At the beginning of your upkeep, you may exile a card from a graveyard. If a
//!   creature card was exiled this way, put a +1/+1 counter on this creature.
//!
//! The "if there are three or more creature cards in your graveyard, transform" clause is
//!   an effect-level branch AFTER the (unconditional) mill — not a whole-trigger
//!   intervening-if — so it is gated inside the resolver via
//!   `conditions::graveyard_matching_at_least` (the mill happens every upkeep regardless).
//! GAP: Back-face upkeep trigger ("exile a card from a graveyard, if creature put +1/+1")
//!   — "exile from any graveyard" with creature-check and conditional counter not
//!   expressible (no graveyard-exile targeting in TriggerCondition). Emitted as GAP.
//! GAP: Back-face-only triggered ability not auto-installed on transform.

use arcana_core::conditions;
use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Deathbonnet Sprout");
    let fungus_sub = reg.interner_mut().intern("Fungus");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(fungus_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![],
        ..Default::default()
    };

    let back_name = reg.interner_mut().intern("Deathbonnet Hulk");
    let back_fungus_sub = reg.interner_mut().intern("Fungus");
    let back_horror_sub = reg.interner_mut().intern("Horror");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(back_fungus_sub);
    back_subtypes.0.insert(back_horror_sub);

    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::green(),
            types: TypeLine::CREATURE.into(),
            subtypes: back_subtypes,
            power: Some(PtValue::Fixed(3)),
            toughness: Some(PtValue::Fixed(3)),
            keywords: vec![],
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            // Front-face upkeep trigger: mill 1, then (if 3+ creature cards in
            //   graveyard) transform. The transform is gated inside the resolver
            //   (effect-level branch after the unconditional mill).
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::Upkeep,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: front_upkeep,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // GAP: back-face-only upkeep trigger not modeled (exile from graveyard + conditional counter).
    )
}

fn front_upkeep(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // Mill is unconditional; the transform is gated on "3+ creature cards in your
    // graveyard". The count is read pre-mill (the resolver builds all effects up
    // front), a slight off-by-one approximation only in the case where the milled
    // card itself is the 3rd creature.
    let mut effects = vec![Effect::Mill { player: trig.controller, count: 1 }];
    if conditions::graveyard_matching_at_least(
        state,
        trig.controller,
        &ObjectFilter::new().with_types(TypeLine::CREATURE.into()),
        3,
    ) {
        effects.push(Effect::Transform { target: trig.source });
    }
    effects
}
