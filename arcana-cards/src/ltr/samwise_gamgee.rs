//! Samwise Gamgee — `{G}{W}` 2/2 Legendary Halfling Peasant.
//!
//! * "Whenever another nontoken creature you control enters, create a Food token."
//! * "Sacrifice three Foods: Return target historic card from your graveyard to
//!   your hand." — modeled as a sacrifice_other (Food, count 3) cost; the
//!   "historic" target filter (artifact/legendary/Saga) is approximated by a
//!   plain graveyard-card target (see GAP).

use arcana_core::effects::{CommodityToken, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Samwise Gamgee");
    let halfling = reg.interner_mut().intern("Halfling");
    let peasant = reg.interner_mut().intern("Peasant");
    let food = reg.interner_mut().intern("Food");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(halfling);
    subtypes.0.insert(peasant);

    let food_filter = ObjectFilter::permanent()
        .controlled_by(ControllerConstraint::You)
        .with_subtype_sym(food);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{G}{W}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                // "another nontoken creature you control enters"
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: ObjectFilter::creature()
                        .controlled_by(ControllerConstraint::You)
                        .nontoken(),
                    from: None,
                    to: Zone::Battlefield,
                },
                intervening_if: None,
                effect: make_food,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "Sacrifice three Foods: Return target historic card from your graveyard to your hand."
                    .into(),
                cost: ActivationCost {
                    sacrifice_other: Some(food_filter),
                    sacrifice_other_count: 3,
                    ..ActivationCost::default()
                },
                // GAP: "historic" (artifact OR legendary OR Saga) target filter is not
                // expressible as one ObjectFilter; targeting any card in your graveyard.
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Card {
                        zone: Zone::Graveyard(0),
                        filter: ObjectFilter::default(),
                    },
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: return_historic,
            }),
    )
}

fn make_food(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::CreateCommodityToken {
        controller: trig.controller,
        kind: CommodityToken::Food,
        count: 1,
    }]
}

fn return_historic(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Object(id) = target else {
        return Vec::new();
    };
    vec![Effect::ReturnFromGraveyardToHand { target: *id }]
}
