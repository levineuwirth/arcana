//! Zoraline, Cosmos Caller — `{1}{W}{B}` 3/3 Legendary Bat Cleric.
//! "Flying, vigilance. Whenever a Bat you control attacks, you gain 1 life.
//! Whenever Zoraline enters or attacks, you may pay {W}{B} and 2 life. When you
//! do, return target nonland permanent card with mana value 3 or less from your
//! graveyard to the battlefield with a finality counter on it."

use arcana_core::actions::OptionalPaymentKind;
use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

fn graveyard_target() -> Vec<TargetRequirement> {
    vec![TargetRequirement {
        filter: TargetFilter::Card {
            zone: Zone::Graveyard(0),
            filter: ObjectFilter::permanent()
                .without_types(TypeLine::LAND.into())
                .with_max_cmc(3)
                .controlled_by(ControllerConstraint::You),
        },
        count: TargetCount::Exactly(1),
        controller: None,
    }]
}

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Zoraline, Cosmos Caller");
    let bat = reg.interner_mut().intern("Bat");
    let cleric = reg.interner_mut().intern("Cleric");
    // "finality counter" has no dedicated CounterKind variant.
    let _finality = reg.interner_mut().intern("finality");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(bat);
    subtypes.0.insert(cleric);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}{B}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Flying, KeywordAbility::Vigilance],
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::CreatureAttacks {
                    filter: ObjectFilter::creature()
                        .controlled_by(ControllerConstraint::You)
                        .with_subtype_sym(bat),
                },
                intervening_if: None,
                effect: bat_attacks_gain_life,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // "Whenever Zoraline enters OR attacks" — split into two triggers.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: maybe_reanimate,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: graveyard_target(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 3,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: maybe_reanimate,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: graveyard_target(),
            }),
    )
}

fn bat_attacks_gain_life(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::GainLife { player: trig.controller, amount: 1 }]
}

fn maybe_reanimate(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    let Some(kind) = reg.interner().lookup("finality").map(CounterKind::Named) else {
        return Vec::new();
    };
    // GAP: the cost is "{W}{B} AND 2 life"; OptionalPaymentKind expresses only
    // one of Mana / Life, so only the {W}{B} mana payment is modeled here. The
    // reflexive "when you do" reanimation runs as the OptionalPayment's then.
    vec![Effect::OptionalPayment {
        chooser: trig.controller,
        cost: OptionalPaymentKind::Mana(ManaCost::parse("{W}{B}").expect("valid cost")),
        then: Box::new(Effect::ReturnFromGraveyardWithCounters {
            target: *id,
            kind,
            count: 1,
        }),
        else_effect: None,
    }]
}
