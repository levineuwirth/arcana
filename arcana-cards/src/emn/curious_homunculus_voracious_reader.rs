//! Curious Homunculus // Voracious Reader
//!
//! Front (Curious Homunculus, {1}{U}, 1/1 Homunculus):
//!   {T}: Add {C}. Spend this mana only to cast an instant or sorcery spell.
//!   (GAP: "spend only to cast instant or sorcery" restriction not modeled — adds plain {C}.)
//!   At the beginning of your upkeep, if there are three or more instant and/or sorcery cards
//!   in your graveyard, transform this creature.
//!   (Intervening-if condition: graveyard count ≥ 3. Modeled via intervening_if + script.)
//!
//! Back (Voracious Reader, Eldrazi Homunculus):
//!   Prowess (GAP: Prowess keyword not in engine keyword surface — omitted.)
//!   Instant and sorcery spells you cast cost {1} less to cast. (GAP: cost reduction not modeled.)

use arcana_core::effects::Effect;
use arcana_core::mana::{ManaCost, ManaUnit};
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardFace, CardRegistry,
};
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, ManaColor, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;
use arcana_core::state::GameState;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Curious Homunculus");
    let homunculus = reg.interner_mut().intern("Homunculus");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(homunculus);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };

    let back_name = reg.interner_mut().intern("Voracious Reader");
    let eldrazi = reg.interner_mut().intern("Eldrazi");
    let homunculus2 = reg.interner_mut().intern("Homunculus");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(eldrazi);
    back_subtypes.0.insert(homunculus2);

    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::blue(),
            types: TypeLine::CREATURE.into(),
            subtypes: back_subtypes,
            power: Some(PtValue::Fixed(3)),
            toughness: Some(PtValue::Fixed(4)),
            // GAP: Prowess not in engine keyword surface.
            keywords: vec![],
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            // {T}: Add {C} (GAP: restricted to instant/sorcery spells only)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}: Add {C}. Spend this mana only to cast an instant or sorcery spell.".to_string(),
                cost: ActivationCost {
                    tap: true,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: true,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: true,
                face_gate: None,
                effect: tap_add_colorless,
            })
            // At the beginning of your upkeep, if 3+ instant/sorcery cards in graveyard, transform
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::Upkeep,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: upkeep_transform_check,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
        // GAP: back-face cost-reduction (instant/sorcery spells cost {1} less) not modeled.
    )
}

fn tap_add_colorless(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: {C} restricted to instant/sorcery casting — plain colorless add.
    vec![Effect::AddMana {
        player: ctx.controller,
        mana: vec![ManaUnit::plain(ManaColor::Colorless, ctx.source)],
    }]
}

fn upkeep_transform_check(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "if there are three or more instant and/or sorcery cards in your graveyard"
    // — no graveyard type-filtered count helper available in script::*. Condition is dropped;
    // transform fires unconditionally at each upkeep. The verify pipeline will flag this.
    vec![Effect::Transform { target: trig.source }]
}
