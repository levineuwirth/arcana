//! Smoldering Egg // Ashmouth Dragon — `{1}{R}` Dragon Egg 0/4 with Defender (front).
//! Whenever you cast an instant or sorcery spell, put ember counters on this creature
//! equal to the mana spent to cast that spell. Then if it has 7+ ember counters,
//! remove them and transform it.
//! Back face (Ashmouth Dragon): 4/4 Dragon with Flying. Whenever you cast an instant
//! or sorcery spell, this creature deals 2 damage to any target.
//!
//! GAP: front face "put a number of ember counters equal to the amount of mana spent to
//! cast that spell" — the amount of mana SPENT to cast the triggering spell is not recorded
//! anywhere in the engine (GameEvent::SpellCast carries no mana amount, and PendingTrigger
//! has no spell-mana-value accessor). The missing primitive is a recorded `mana_spent` on
//! the cast event / a PendingTrigger spell-mana accessor. Because the ember-counter count
//! and the 7-counter threshold-transform both depend on that unavailable amount, the front
//! mechanic is left unwired; the counter primitive itself (Effect::AddCounters with
//! CounterKind::Named("ember")) exists.
//! Back face (Ashmouth Dragon): "Whenever you cast an instant or sorcery spell, this
//! creature deals 2 damage to any target" — WIRED (face-gated to the back face).

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, ObjectOrPlayer, TargetChoice, TargetCount,
    TargetFilter, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Smoldering Egg");
    let dragon_sub = reg.interner_mut().intern("Dragon");
    let egg_sub = reg.interner_mut().intern("Egg");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dragon_sub);
    subtypes.0.insert(egg_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Defender],
        ..Default::default()
    };

    let back_name = reg.interner_mut().intern("Ashmouth Dragon");
    let back_dragon_sub = reg.interner_mut().intern("Dragon");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(back_dragon_sub);
    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::red(),
            types: TypeLine::CREATURE.into(),
            subtypes: back_subtypes,
            power: Some(PtValue::Fixed(4)),
            toughness: Some(PtValue::Fixed(4)),
            keywords: vec![KeywordAbility::Flying],
            ..Default::default()
        },
        spell_ability: None,
    };

    let instant_sorcery_filter = ObjectFilter::new()
        .with_types_any(TypeLine(TypeLine::INSTANT | TypeLine::SORCERY));
    let back_instant_sorcery_filter = ObjectFilter::new()
        .with_types_any(TypeLine(TypeLine::INSTANT | TypeLine::SORCERY));

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            // Front: whenever you cast an instant or sorcery spell, put ember counters
            // equal to mana spent (GAP: amount not accessible) then check threshold (GAP).
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SpellCast {
                    filter: Some(instant_sorcery_filter),
                    caster: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: on_instant_sorcery_cast,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // Back (Ashmouth Dragon): whenever you cast an instant or sorcery spell,
            // this creature deals 2 damage to any target.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SpellCast {
                    filter: Some(back_instant_sorcery_filter),
                    caster: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: back_deal_two_damage,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::AnyTarget,
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
            })
            // Trigger 1 fires only on the front face; trigger 2 only on the back face.
            .with_trigger_face_gate(1, 0)
            .with_trigger_face_gate(2, 1),
    )
}

fn on_instant_sorcery_cast(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "put a number of ember counters equal to the amount of mana spent"
    // — mana spent to cast the triggering spell is not recorded in the engine
    // (no mana_spent on the cast event / no PendingTrigger accessor). The ember
    // counter primitive exists (Effect::AddCounters + CounterKind::Named("ember")),
    // but the count and the 7-counter threshold-transform both need the unavailable
    // amount, so the front mechanic is left unwired.
    Vec::new()
}

fn back_deal_two_damage(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else {
        return Vec::new();
    };
    let dt = match target {
        TargetChoice::Object(id) => DamageTarget::Object(*id),
        TargetChoice::Player(p) => DamageTarget::Player(*p),
        TargetChoice::ObjectOrPlayer(o) => match o {
            ObjectOrPlayer::Object(id) => DamageTarget::Object(*id),
            ObjectOrPlayer::Player(p) => DamageTarget::Player(*p),
        },
    };
    vec![Effect::DealDamage {
        source: trig.source,
        target: dt,
        amount: 2,
    }]
}
