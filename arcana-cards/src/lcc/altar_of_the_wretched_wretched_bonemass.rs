//! Altar of the Wretched // Wretched Bonemass — `{2}{B}` Artifact (front),
//! Craft transforming DFC into a Skeleton Horror creature (back).
//!
//! Front (Altar of the Wretched, Artifact):
//! * When this artifact enters, you may sacrifice a nontoken creature.
//!   If you do, draw X cards, then mill X cards, where X is that
//!   creature's power.
//! * Craft with one or more creatures {2}{B}{B}.
//! * {2}{B}: Return this card from your graveyard to your hand.
//!
//! Back (Wretched Bonemass, Creature — Skeleton Horror):
//! * Power/toughness each equal to the total power of the exiled cards
//!   used to craft it; gains a long list of evergreen keywords if an
//!   exiled craft card has them.
//!
//! GAP: Craft (CR 702.166 exile-and-transform) is not a modeled
//! keyword — the front never transforms via this engine. The back face
//! is declared as a vanilla 0/0 placeholder; its craft-derived dynamic
//! P/T and conditional keyword grants are not expressible.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardFace, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{
    CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Altar of the Wretched");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::ARTIFACT.into(),
        supertypes: SupertypeSet::default(),
        ..Default::default()
    };

    // Back face — Wretched Bonemass (Creature — Skeleton Horror).
    // Dynamic P/T (= total power of exiled craft cards) and the
    // conditional keyword grants are not expressible; declare a 0/0
    // placeholder.
    let back_name = reg.interner_mut().intern("Wretched Bonemass");
    let skeleton = reg.interner_mut().intern("Skeleton");
    let horror = reg.interner_mut().intern("Horror");
    let mut back_subs = SubtypeSet::default();
    back_subs.0.insert(skeleton);
    back_subs.0.insert(horror);
    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            mana_cost: None,
            colors: ColorSet::black(),
            types: TypeLine::CREATURE.into(),
            subtypes: back_subs,
            power: Some(PtValue::Fixed(0)),
            toughness: Some(PtValue::Fixed(0)),
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            // ETB: may sacrifice a nontoken creature, draw X / mill X.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: altar_etb,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // {2}{B}: Return this card from your graveyard to your hand.
            .with_activated_ability(ActivatedAbilityDef {
                text: "{2}{B}: Return this card from your graveyard to your hand.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{2}{B}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Graveyard,
                is_instant_speed: false,
                face_gate: None,
                effect: altar_recur,
            }),
    )
}

fn altar_etb(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "you may sacrifice a nontoken creature. If you do, draw X,
    // then mill X, where X is that creature's power." Sacrifice is not
    // an OptionalPayment cost kind (only Mana / Life), and X depends on
    // the chosen creature's power, which isn't known until the optional
    // sacrifice resolves. Not expressible with the catalog.
    Vec::new()
}

fn altar_recur(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::ReturnFromGraveyardToHand { target: ctx.source }]
}
