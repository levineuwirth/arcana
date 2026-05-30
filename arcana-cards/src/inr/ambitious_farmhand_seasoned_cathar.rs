//! Ambitious Farmhand // Seasoned Cathar — `{1}{W}` Human Peasant creature 1/1.
//! Front: When this creature enters, you may search your library for a basic
//!   Plains card, reveal it, put it into your hand, then shuffle.
//! Coven — {1}{W}{W}: Transform this creature. Activate only if you control
//!   three or more creatures with different powers.
//! Back (Seasoned Cathar): Lifelink.
//!
//! GAP: Coven condition "activate only if you control three or more creatures
//!   with different powers" — activation precondition on different-power count
//!   is not expressible; the activation is modeled without the legality gate.
//! GAP: Plains tutor searches for a "basic Plains card" — using
//!   TutorToHand with a supertypes+subtype filter as best effort.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardFace, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Ambitious Farmhand");
    let human_sub = reg.interner_mut().intern("Human");
    let peasant_sub = reg.interner_mut().intern("Peasant");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human_sub);
    subtypes.0.insert(peasant_sub);

    // Pre-intern Plains subtype for tutor filter
    let _plains_sub = reg.interner_mut().intern("Plains");

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };

    let back_name = reg.interner_mut().intern("Seasoned Cathar");
    let back_human_sub = reg.interner_mut().intern("Human");
    let back_knight_sub = reg.interner_mut().intern("Knight");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(back_human_sub);
    back_subtypes.0.insert(back_knight_sub);

    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::white(),
            types: TypeLine::CREATURE.into(),
            subtypes: back_subtypes,
            power: Some(PtValue::Fixed(3)),
            toughness: Some(PtValue::Fixed(3)),
            keywords: vec![KeywordAbility::Lifelink],
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            // ETB: search library for a basic Plains card, put it into your hand.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_tutor_plains,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![],
            })
            // Coven — {1}{W}{W}: Transform. Activate only if you control three
            // or more creatures with different powers.
            // GAP: Coven precondition not enforceable; modeled as unconditional.
            .with_activated_ability(ActivatedAbilityDef {
                text: "{1}{W}{W}: Transform this creature. (Coven — activate only if you control three or more creatures with different powers.)".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{1}{W}{W}").unwrap(),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: Some(0), // front face only
                effect: transform_self,
            }),
    )
}

fn etb_tutor_plains(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    // Search for a basic Plains card and put it into hand.
    // Use subtype_filter for "Plains" (land subtype), combined with basic supertype.
    let filter = script::subtype_filter(reg, "Plains")
        .with_supertypes(SupertypeSet::new().with(SupertypeSet::BASIC))
        .with_types(TypeLine::LAND.into());
    vec![Effect::TutorToHand {
        player: trig.controller,
        filter,
        reveal: true,
    }]
}

fn transform_self(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::Transform { target: ctx.source }]
}
