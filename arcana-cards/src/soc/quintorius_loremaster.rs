//! Quintorius, Loremaster — `{3}{R}{W}` 3/5 Legendary Elephant Cleric.
//!
//! Vigilance.
//! At the beginning of your end step, exile target noncreature, nonland card
//! from your graveyard. Create a 3/2 red and white Spirit creature token.
//! {1}{R}{W}, {T}, Sacrifice a Spirit: Choose target card exiled with
//! Quintorius. You may cast that card this turn without paying its mana cost.
//! If that spell would be put into a graveyard, put it on the bottom of its
//! owner's library instead.

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Quintorius, Loremaster");
    let elephant = reg.interner_mut().intern("Elephant");
    let cleric = reg.interner_mut().intern("Cleric");
    let _spirit = reg.interner_mut().intern("Spirit");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elephant);
    subtypes.0.insert(cleric);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}{W}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![KeywordAbility::Vigilance],
        ..Default::default()
    };

    let spirit_sac = arcana_core::script::subtype_filter(reg, "Spirit");

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::End,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: end_step_exile_and_token,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Card {
                        zone: Zone::Graveyard(0),
                        filter: ObjectFilter::new()
                            .without_types(TypeLine(TypeLine::CREATURE | TypeLine::LAND))
                            .controlled_by(ControllerConstraint::You),
                    },
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{1}{R}{W}, {T}, Sacrifice a Spirit: Choose target card exiled with Quintorius. You may cast that card this turn without paying its mana cost.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{1}{R}{W}").expect("valid cost"),
                    tap: true,
                    sacrifice_other: Some(spirit_sac),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: cast_exiled_card,
            }),
    )
}

fn end_step_exile_and_token(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let mut effects = Vec::new();
    if let Some(TargetChoice::Object(id)) = trig.targets.targets.first() {
        effects.push(Effect::ExileFromGraveyard { target: *id });
    }
    let spirit = reg.interner().lookup("Spirit").expect("Spirit interned during register()");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spirit);
    let token = TokenDefinition {
        name: spirit,
        colors: ColorSet::red() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![],
        abilities: vec![],
    };
    effects.push(Effect::CreateToken { controller: trig.controller, token });
    effects
}

fn cast_exiled_card(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "Choose target card exiled with Quintorius. You may cast that card
    // this turn without paying its mana cost. If that spell would be put into a
    // graveyard, put it on the bottom of its owner's library instead." — there
    // is no effect to cast a specific exiled card for free, nor to install the
    // graveyard-replacement rider.
    Vec::new()
}
