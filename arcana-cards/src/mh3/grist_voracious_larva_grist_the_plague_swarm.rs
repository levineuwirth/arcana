//! Grist, Voracious Larva // Grist, the Plague Swarm
//! Front: `{G}` Legendary Creature — Insect 1/2 with Deathtouch.
//! Whenever Grist or another creature you control enters, if it entered from your graveyard
//! or you cast it from your graveyard, you may pay {G}. If you do, exile Grist, then return
//! it to the battlefield transformed under its owner's control.
//!
//! Back: "Grist, the Plague Swarm" — Legendary Planeswalker — Grist (loyalty 3).
//! +1: Create a 1/1 black and green Insect creature token, then mill two cards.
//!     Put a deathtouch counter on the token if a black card was milled this way.
//! −2: Destroy target artifact or enchantment.
//! −6: For each creature card in your graveyard, create a token that's a copy of it,
//!     except it's a 1/1 black and green Insect.
//!
//! # GAPs
//! - Front trigger condition: "if it entered from your graveyard or you cast it from your
//!   graveyard" — conditional zone-of-origin check not expressible; modeled as ZoneChange
//!   trigger from Graveyard only (partial fidelity).
//! - "+1 put a deathtouch counter on the token if a black card was milled this way" —
//!   conditional counter placement based on milled card colors not expressible.
//! - "−6 create a copy of each creature card in your graveyard, except it's a 1/1 black
//!   and green Insect" — CopyPermanent works on battlefield objects; graveyard-card-copy
//!   with stat override not expressible. GAP: entire −6 effect.
//! - Back-face planeswalker loyalty abilities not auto-installed on transform.

use arcana_core::actions::OptionalPaymentKind;
use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Grist, Voracious Larva");
    let insect_sub = reg.interner_mut().intern("Insect");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(insect_sub);

    // Pre-intern token subtype
    let _insect_tok = reg.interner_mut().intern("Insect");

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Deathtouch],
        ..Default::default()
    };

    // Back face: Grist, the Plague Swarm — Legendary Planeswalker — Grist
    let back_name = reg.interner_mut().intern("Grist, the Plague Swarm");
    let grist_sub = reg.interner_mut().intern("Grist");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(grist_sub);
    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::black() | ColorSet::green(),
            types: TypeLine::PLANESWALKER.into(),
            subtypes: back_subtypes,
            supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
            loyalty: Some(3),
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            // Front triggered ability: when Grist or another creature you control enters from
            // graveyard, you may pay {G} to transform.
            // GAP: precise "entered from graveyard or cast from graveyard" condition not
            // expressible; approximated as any creature ZoneChange from Graveyard to Battlefield
            // under your control.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: ObjectFilter::creature().controlled_by(ControllerConstraint::You),
                    from: Some(Zone::Graveyard(0)),
                    to: Zone::Battlefield,
                },
                intervening_if: None,
                effect: graveyard_return_trigger,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
        // GAP: back-face loyalty abilities (+1, −2, −6) not auto-installed on transform
        // — back-face-only triggered/activated abilities not modeled.
    )
}

fn graveyard_return_trigger(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // "You may pay {G}. If you do, exile Grist, then return it transformed."
    // Modeled as OptionalPayment of {G} then Transform.
    vec![Effect::OptionalPayment {
        chooser: trig.controller,
        cost: OptionalPaymentKind::Mana(ManaCost::parse("{G}").expect("valid cost")),
        then: Box::new(Effect::Transform { target: trig.source }),
        else_effect: None,
    }]
}
