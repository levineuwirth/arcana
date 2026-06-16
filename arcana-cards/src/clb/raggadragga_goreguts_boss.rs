//! Raggadragga, Goreguts Boss — `{2}{R}{G}` 4/4 Legendary Human Boar.
//! Static: each creature you control with a mana ability gets +2/+2 (GAP).
//! "Whenever a creature you control with a mana ability attacks, untap it."
//! "Whenever you cast a spell, if at least seven mana was spent to cast it,
//! untap target creature. It gets +7/+7 and gains trample until end of turn."

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{
    CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Raggadragga, Goreguts Boss");
    let human = reg.interner_mut().intern("Human");
    let boar = reg.interner_mut().intern("Boar");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(boar);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}{G}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };
    // GAP: static "each creature you control with a mana ability gets +2/+2"
    // — no static anthem primitive, and "has a mana ability" is not an
    // ObjectFilter predicate.
    reg.register(
        CardDefinition::new(name, chars)
            // GAP: filter cannot restrict to "with a mana ability"; matches
            // any creature you control that attacks.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::CreatureAttacks {
                    filter: ObjectFilter::creature()
                        .controlled_by(ControllerConstraint::You),
                },
                intervening_if: None,
                effect: untap_attacker,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // GAP: intervening-if "if at least seven mana was spent to cast
            // it" has no condition helper; firing unconditionally.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SpellCast {
                    filter: None,
                    caster: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: untap_pump_trample,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement::target_creature()],
            }),
    )
}

fn untap_attacker(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(id) = trig.attacking_creature() else { return Vec::new(); };
    vec![Effect::Untap { target: id }]
}

fn untap_pump_trample(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    vec![
        Effect::Untap { target: *id },
        Effect::Pump {
            target: *id,
            power: 7,
            toughness: 7,
            duration: Duration::EndOfTurn,
            keywords: vec![KeywordAbility::Trample],
        },
    ]
}
