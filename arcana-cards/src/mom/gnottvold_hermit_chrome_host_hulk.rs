//! Gnottvold Hermit // Chrome Host Hulk — `{3}{G}` green Creature — Troll // Creature — Phyrexian Troll.
//! Front: 4/4.
//! {5}{U/P}: Transform this creature. Activate only as a sorcery.
//! ({U/P} can be paid with either {U} or 2 life.)
//! ---
//! Back (Chrome Host Hulk): Phyrexian Troll 5/5.
//! Whenever this creature attacks, up to one other target creature has base
//! power and toughness 5/5 until end of turn.
//!
//! GAP: Phyrexian mana cost {U/P} is not parseable by ManaCost::parse; the
//!      activated transform ability uses {5}{U} as a best-effort approximation.
//! GAP: "Activate only as a sorcery" speed restriction not modeled.
//! GAP: The "when attacks" trigger is a back-face-only ability; it is authored
//!      on the CardDefinition and will fire on the front face too (engine debt:
//!      back-face-only triggered ability not modeled).

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardFace, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Gnottvold Hermit");
    let troll_sub = reg.interner_mut().intern("Troll");
    let mut front_subtypes = SubtypeSet::default();
    front_subtypes.0.insert(troll_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes: front_subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![],
        ..Default::default()
    };

    let back_name = reg.interner_mut().intern("Chrome Host Hulk");
    let phyrexian_sub = reg.interner_mut().intern("Phyrexian");
    let troll_sub2 = reg.interner_mut().intern("Troll");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(phyrexian_sub);
    back_subtypes.0.insert(troll_sub2);

    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::green(),
            types: TypeLine::CREATURE.into(),
            subtypes: back_subtypes,
            power: Some(PtValue::Fixed(5)),
            toughness: Some(PtValue::Fixed(5)),
            keywords: vec![],
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            // {5}{U/P}: Transform this creature (sorcery speed).
            // GAP: {U/P} Phyrexian mana approximated as {5}{U}.
            // GAP: "activate only as a sorcery" not modeled.
            .with_activated_ability(ActivatedAbilityDef {
                text: "{5}{U}: Transform this creature. (GAP: {U/P} approximated as {U})".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{5}{U}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: transform_self,
            })
            // Whenever this creature attacks (back face), up to one other target
            // creature has base power and toughness 5/5 until end of turn.
            // GAP: back-face-only triggered ability; fires on both faces.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: on_attack_set_base_pt,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::new().with_types(TypeLine::CREATURE.into()),
                    ),
                    count: TargetCount::UpTo(1),
                    controller: None,
                }],
            }),
    )
}

fn transform_self(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::Transform { target: ctx.source }]
}

fn on_attack_set_base_pt(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Object(id) = target else {
        return Vec::new();
    };
    vec![Effect::SetBasePT {
        target: *id,
        power: 5,
        toughness: 5,
        duration: Duration::EndOfTurn,
    }]
}
