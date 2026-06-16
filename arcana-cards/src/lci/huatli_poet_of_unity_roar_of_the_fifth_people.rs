//! Huatli, Poet of Unity // Roar of the Fifth People
//! — `{2}{G}` Legendary Human Warrior Bard creature 2/3.
//! Front: When Huatli enters, search your library for a basic land card, reveal
//!   it, put it into your hand, then shuffle.
//! {3}{R/W}{R/W}: Exile Huatli, then return her to the battlefield transformed
//!   under her owner's control. Activate only as a sorcery.
//! Back (Roar of the Fifth People): Enchantment — Saga (IV chapters).
//!   I — Create two 3/3 green Dinosaur creature tokens.
//!   II — This Saga gains "Creatures you control have '{T}: Add {R}, {G}, or {W}.'"
//!   III — Search your library for a Dinosaur card, reveal it, put it into your
//!          hand, then shuffle.
//!   IV — Dinosaurs you control gain double strike and trample until end of turn.
//!
//! GAP: {3}{R/W}{R/W} hybrid cost — ManaCost::parse handles hybrid, but the
//!   activation "exile self then return transformed" is modeled as
//!   Effect::Transform (which swaps characteristics in-place, not exile/return).
//! GAP: Back face (Saga) chapter abilities cannot be expressed on a CardFace
//!   back-characteristics; the engine only supports static characteristics on
//!   transform backs, not triggered-ability installation. Chapters are GAP'd.
//! GAP: Chapter II "Creatures you control have '{T}: Add {R},{G}, or {W}.'"
//!   — mana ability grant not expressible.
//! GAP: Back face Saga lore counter accumulation and chapter triggers not modeled
//!   (back-face-only triggered abilities not auto-installed on transform).
//! GAP: Back face has no EntersWithSpec for Lore counter (ETB from transform).

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
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Huatli, Poet of Unity");
    let human_sub = reg.interner_mut().intern("Human");
    let warrior_sub = reg.interner_mut().intern("Warrior");
    let bard_sub = reg.interner_mut().intern("Bard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human_sub);
    subtypes.0.insert(warrior_sub);
    subtypes.0.insert(bard_sub);

    // Pre-intern Dinosaur token subtype
    let _dino_sub = reg.interner_mut().intern("Dinosaur");

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    // Back face: Roar of the Fifth People (Enchantment — Saga).
    // GAP: back-face Saga chapter abilities not auto-installed on transform.
    let back_name = reg.interner_mut().intern("Roar of the Fifth People");
    let saga_sub = reg.interner_mut().intern("Saga");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(saga_sub);

    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::green(),
            types: TypeLine::ENCHANTMENT.into(),
            subtypes: back_subtypes,
            supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            // ETB: search library for a basic land card, put it into your hand.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_tutor_land,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![],
            })
            // {3}{R/W}{R/W}: Exile Huatli, then return her transformed.
            // Modeled as Effect::Transform (in-place swap). "Exile then return"
            // semantics not fully expressible; Transform is the best fit.
            .with_activated_ability(ActivatedAbilityDef {
                text: "{3}{R/W}{R/W}: Exile Huatli, then return her to the battlefield transformed. Activate only as a sorcery.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{3}{R/W}{R/W}").unwrap(),
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

fn etb_tutor_land(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // Search for any basic land card.
    let filter = arcana_core::targets::ObjectFilter::new()
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
    // GAP: Oracle says "exile, then return transformed" but engine Transform
    // swaps characteristics in-place. The exile/return semantics (new object,
    // ETB triggers for the Saga) are not reproduced.
    vec![Effect::Transform { target: ctx.source }]
}
