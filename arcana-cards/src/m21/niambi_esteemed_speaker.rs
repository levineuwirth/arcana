//! Niambi, Esteemed Speaker — `{W}{U}` 2/1 legendary Human Cleric.
//!
//! Flash.
//! When Niambi enters, you may return another target creature you
//! control to its owner's hand. If you do, you gain life equal to that
//! creature's mana value.
//! {1}{W}{U}, {T}, Discard a legendary card: Draw two cards.
//!
//! The ETB returns the chosen creature to hand (faithful). The "gain
//! life equal to that creature's mana value" rider is GAP'd: there is
//! no script helper for a creature's mana value, and a dynamic amount
//! must not be hardcoded.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Niambi, Esteemed Speaker");
    let human = reg.interner_mut().intern("Human");
    let cleric = reg.interner_mut().intern("Cleric");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(cleric);

    let creature_you_control =
        ObjectFilter::creature().controlled_by(ControllerConstraint::You);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{W}{U}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Flash],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_bounce,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(creature_you_control),
                    count: TargetCount::UpTo(1),
                    controller: None,
                }],
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{1}{W}{U}, {T}, Discard a legendary card: Draw two cards.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{1}{W}{U}").expect("valid cost"),
                    tap: true,
                    discard_other: Some(
                        ObjectFilter::default()
                            .with_supertypes(SupertypeSet(SupertypeSet::LEGENDARY)),
                    ),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: draw_two,
            }),
    )
}

fn etb_bounce(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Object(id) = target else {
        return Vec::new();
    };
    // GAP: "you gain life equal to that creature's mana value" — no
    // mana-value script helper; dynamic amount cannot be hardcoded.
    vec![Effect::ReturnToHand { target: *id }]
}

fn draw_two(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::DrawCards {
        player: ctx.controller,
        count: 2,
    }]
}
