//! Minsc, Beloved Ranger — `{R}{G}{W}` 3/3 Legendary Human Ranger.
//! "When Minsc enters, create Boo, a legendary 1/1 red Hamster creature
//! token with trample and haste. {X}: Until end of turn, target creature you
//! control has base power and toughness X/X and becomes a Giant in addition
//! to its other types. Activate only as a sorcery."
//!
//! The ETB creates the Boo token (a 1/1 red Hamster with trample and haste;
//! the token's Legendary supertype is not expressible on a TokenDefinition —
//! minor GAP). The {X} activation sets a target creature you control's base
//! power and toughness to X/X (read from ctx.x_value); "becomes a Giant"
//! (gaining a creature subtype) is not expressible with the demonstrated
//! primitives — a GAP. Sorcery-speed is modeled via is_instant_speed: false.

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::layers::Duration;
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
    let name = reg.interner_mut().intern("Minsc, Beloved Ranger");
    let human = reg.interner_mut().intern("Human");
    let ranger = reg.interner_mut().intern("Ranger");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(ranger);

    // Pre-intern the Boo token's name and subtype.
    let _boo = reg.interner_mut().intern("Boo");
    let _hamster = reg.interner_mut().intern("Hamster");

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{R}{G}{W}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::red() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: make_boo,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{X}: Until end of turn, target creature you control has \
                       base power and toughness X/X and becomes a Giant in \
                       addition to its other types. Activate only as a sorcery."
                    .into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{X}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::creature()
                            .controlled_by(ControllerConstraint::You),
                    ),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: set_xx,
            }),
    )
}

fn make_boo(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: the Boo token's Legendary supertype is not expressible on a
    // TokenDefinition; the token is otherwise faithful.
    let boo = reg.interner().lookup("Boo").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    if let Some(h) = reg.interner().lookup("Hamster") {
        subtypes.0.insert(h);
    }
    vec![Effect::CreateToken {
        controller: trig.controller,
        token: TokenDefinition {
            name: boo,
            colors: ColorSet::red(),
            types: TypeLine::CREATURE.into(),
            subtypes,
            power: Some(PtValue::Fixed(1)),
            toughness: Some(PtValue::Fixed(1)),
            keywords: vec![KeywordAbility::Trample, KeywordAbility::Haste],
            abilities: vec![],
        },
    }]
}

fn set_xx(
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
    let x = ctx.x_value.unwrap_or(0) as i32;
    // GAP: "becomes a Giant in addition to its other types" — gaining a
    // creature subtype is not expressible with the demonstrated primitives.
    vec![Effect::SetBasePT {
        target: *id,
        power: x,
        toughness: x,
        duration: Duration::EndOfTurn,
    }]
}
