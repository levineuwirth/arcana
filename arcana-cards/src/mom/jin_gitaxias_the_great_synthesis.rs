//! Jin-Gitaxias // The Great Synthesis
//!
//! Front face: Legendary Creature — Phyrexian Praetor, 5/5, {3}{U}{U}
//!   Ward {2}
//!   Whenever you cast a noncreature spell with mana value 3 or greater, draw a card.
//!   {3}{U}: Exile Jin-Gitaxias, then return it to the battlefield transformed under its
//!     owner's control. Activate only as a sorcery and only if you have seven or more
//!     cards in hand.
//!     "only if you have seven or more cards in hand" modeled via
//!     `activation_condition` + `conditions::hand_at_least`.
//!     GAP: exile-then-return-transformed not directly expressible; modeled as Transform.
//!
//! Back face: Enchantment — Saga (The Great Synthesis)
//!   I — Draw cards equal to the number of cards in your hand. You have no maximum hand
//!     size for as long as you control this Saga.
//!     GAP: "no maximum hand size" static continuous effect not expressible.
//!     Draw count is DYNAMIC — script::hand_size used.
//!   II — Return all non-Phyrexian creatures to their owners' hands.
//!     GAP: chapter not wired — the Saga chapter triggers (lore counters + I/II/III)
//!     are not authored on this DFC at all (see back-face GAP below). The
//!     "non-Phyrexian" exclusion itself is now expressible
//!     (ObjectFilter::without_subtype_sym) once the chapter is wired.
//!   III — You may cast any number of spells from your hand without paying their mana
//!     costs. Exile this Saga, then return it to the battlefield (front face up).
//!     GAP: "cast any number of spells for free" not expressible.
//!     Front-face return is modeled as Transform.
//!   GAP: back-face-only triggered abilities (Saga chapter triggers) not auto-installed
//!     on transform.

use arcana_core::conditions;
use arcana_core::effects::Effect;
use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, ObjectId};
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardFace, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PlayerId, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Jin-Gitaxias");

    let sub_phyrexian = reg.interner_mut().intern("Phyrexian");
    let sub_praetor = reg.interner_mut().intern("Praetor");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(sub_phyrexian);
    subtypes.0.insert(sub_praetor);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        subtypes,
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![KeywordAbility::Ward(ManaCost::parse("{2}").expect("valid cost"))],
        ..Default::default()
    };

    // Back face: The Great Synthesis — Enchantment — Saga
    let back_name = reg.interner_mut().intern("The Great Synthesis");
    let back_sub_saga = reg.interner_mut().intern("Saga");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(back_sub_saga);

    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::blue(),
            types: TypeLine::ENCHANTMENT.into(),
            supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
            subtypes: back_subtypes,
            ..Default::default()
        },
        spell_ability: None,
    };

    // Filter for noncreature spells with CMC >= 3
    let noncreature_mv3_filter = ObjectFilter::new()
        .without_types(TypeLine::CREATURE.into())
        .with_min_cmc(3);

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            // Whenever you cast a noncreature spell with mana value 3 or greater, draw a card.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SpellCast {
                    filter: Some(noncreature_mv3_filter),
                    caster: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: on_noncreature_cast,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // {3}{U}: Transform (exile then return transformed). Sorcery speed.
            // "only if you have 7+ cards in hand" modeled via activation_condition.
            .with_activated_ability(ActivatedAbilityDef {
                text: "{3}{U}: Exile Jin-Gitaxias, then return it to the battlefield transformed. Activate only as a sorcery and only if you have seven or more cards in hand.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{3}{U}").unwrap(),
                    activation_condition: Some(precond_hand_7),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: Some(0), // front face only
                effect: transform_to_saga,
            })
            // GAP: back-face-only triggered abilities (Saga chapter triggers I, II, III)
            // not auto-installed on transform.
    )
}

fn on_noncreature_cast(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::DrawCards {
        player: trig.controller,
        count: 1,
    }]
}

fn transform_to_saga(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: oracle says "exile Jin-Gitaxias, then return it transformed";
    // modeled as Transform directly.
    vec![Effect::Transform { target: ctx.source }]
}

fn precond_hand_7(state: &GameState, _source: ObjectId, you: PlayerId, _reg: &CardRegistry) -> bool {
    conditions::hand_at_least(state, you, 7)
}
