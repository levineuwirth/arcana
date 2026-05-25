//! Armored Kincaller — `{2}{G}` 3/3 green Dinosaur. "When this
//! creature enters, you may reveal a Dinosaur card from your hand.
//! If you do or if you control another Dinosaur, you gain 3 life."
//!
//! GAP: the "may reveal a Dinosaur card from your hand" branch is not
//! expressible with the current Effect catalog (no reveal-from-hand
//! primitive and no player-choice modal at trigger resolution). This
//! implementation honors the second disjunct only — if the
//! controller controls another Dinosaur, gain 3 life — which is the
//! engine-faithful subset of the conditional.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Armored Kincaller");
    let dinosaur = reg.interner_mut().intern("Dinosaur");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dinosaur);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_gain_life_if_dino,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

/// ETB resolution. We can't model the "may reveal a Dinosaur card
/// from your hand" choice with the current Effect catalog, so we
/// implement only the "if you control another Dinosaur" branch: if
/// the controller has at least one Dinosaur on the battlefield
/// besides this one (i.e. total Dinosaurs they control ≥ 2 — this
/// card is already on the battlefield by the time the ETB trigger
/// resolves), gain 3 life.
fn etb_gain_life_if_dino(
    state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let dino_filter =
        script::subtype_filter(reg, "Dinosaur").controlled_by(ControllerConstraint::You);
    let count = script::count_matching(state, &dino_filter, trig.controller);
    // GAP: cannot model the "you may reveal a Dinosaur card from your
    // hand" disjunct — no reveal-from-hand Effect primitive.
    if count >= 2 {
        vec![Effect::GainLife { player: trig.controller, amount: 3 }]
    } else {
        Vec::new()
    }
}
