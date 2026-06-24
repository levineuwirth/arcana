//! Egon, God of Death // Throne of Death — `{2}{B}` Legendary Creature — God (MDFC)
//! Front: 6/6 with Deathtouch. At the beginning of your upkeep, exile two cards from your
//! graveyard. If you can't, sacrifice Egon and draw a card.
//! Back: Legendary Artifact — Throne of Death. At the beginning of your upkeep, mill a card.
//! {2}{B}, {T}, Exile a creature card from your graveyard: Draw a card.
//!
//! GAP: front-face upkeep trigger "exile two cards from your graveyard; if you can't,
//! sacrifice Egon and draw" — the conditional graveyard-size check with self-sacrifice
//! is not expressible with the current engine API surface.
//! GAP: back-face activated ability ({2}{B},{T}, exile a creature card from your
//! graveyard: draw) — the "exile a CHOSEN creature card from your graveyard" cost is
//! not in the ActivationCost surface (exile_self exiles the source only; there is no
//! exile-other-from-graveyard additional cost). Left GAP'd.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
use arcana_core::triggers::{
    ControllerConstraint, PendingTrigger, TriggerCondition, TriggerFrequency,
    TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;
use arcana_core::state::GameState;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Egon, God of Death");
    let sub_god = reg.interner_mut().intern("God");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(sub_god);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(6)),
        toughness: Some(PtValue::Fixed(6)),
        keywords: vec![KeywordAbility::Deathtouch],
        ..Default::default()
    };

    // Back face: Legendary Artifact — Throne of Death
    let back_name = reg.interner_mut().intern("Throne of Death");
    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::black(),
            types: TypeLine::ARTIFACT.into(),
            supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_mdfc_back(back)
            // Front-face upkeep trigger: exile two cards from your graveyard;
            // if you can't, sacrifice Egon and draw a card.
            // GAP: conditional "exile 2 from graveyard or else sacrifice self + draw" —
            // not expressible with current engine effect/cost API.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::Upkeep,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: egon_upkeep_trigger,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![],
            })
            // Back face (Throne of Death): "At the beginning of your upkeep, mill a
            // card." Fires only while the back artifact face is showing.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::Upkeep,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: throne_upkeep_mill,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![],
            })
            // Trigger 1 is the front (creature) upkeep; trigger 2 the back
            // (artifact) upkeep mill.
            .with_trigger_face_gate(1, 0)
            .with_trigger_face_gate(2, 1),
    )
}

fn throne_upkeep_mill(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::Mill { player: trig.controller, count: 1 }]
}

fn egon_upkeep_trigger(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "exile two cards from your graveyard; if you can't, sacrifice Egon and draw a card"
    // The conditional graveyard-size check with self-sacrifice and the "exile two from GY"
    // cost are not expressible with the current engine effect API.
    Vec::new()
}
