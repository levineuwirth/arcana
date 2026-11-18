//! Mausoleum Wanderer — `{U}` 1/1 Spirit with Flying.
//!
//! * Flying — base keyword.
//! * "Whenever another Spirit you control enters, this creature gets +1/+1
//!   until end of turn." — a Spirit-filtered `ZoneChange` to the battlefield
//!   under your control; the pump targets this creature. The filter excludes
//!   the source, so the Wanderer's own entry does not grow it.
//! * "Sacrifice this creature: Counter target instant or sorcery spell unless
//!   its controller pays {X}, where X is this creature's power." — the
//!   Wanderer is gone when the ability resolves, so X is its power as it last
//!   existed on the battlefield (CR 113.7a, 608.2h): the pump above and any
//!   anthem are in it. The spell's controller pays {X} or the spell is
//!   countered.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Mausoleum Wanderer");
    let spirit = reg.interner_mut().intern("Spirit");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spirit);

    let spirit_filter = script::subtype_filter(reg, "Spirit")
        .controlled_by(ControllerConstraint::You)
        .excluding_source();

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: spirit_filter,
                    from: None,
                    to: Zone::Battlefield,
                },
                intervening_if: None,
                effect: spirit_pump,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "Sacrifice this creature: Counter target instant or sorcery spell unless its controller pays {X}, where X is this creature's power.".into(),
                cost: ActivationCost {
                    sacrifice: true,
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Spell(
                        ObjectFilter::new().with_types_any(TypeLine(
                            TypeLine::INSTANT | TypeLine::SORCERY,
                        )),
                    ),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: true,
                face_gate: None,
                effect: sac_counter_spell,
            }),
    )
}

fn spirit_pump(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::Pump {
        target: trig.source,
        power: 1,
        toughness: 1,
        duration: Duration::EndOfTurn,
        keywords: vec![],
    }]
}

fn sac_counter_spell(
    state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Object(id) = target else {
        return Vec::new();
    };
    // X is the sacrificed Wanderer's last-known power; a power below zero
    // asks for {0} (CR 107.1b).
    let x = script::last_known_power(state, ctx.source).unwrap_or(0).max(0);
    vec![Effect::CounterUnlessPays {
        target: *id,
        cost: ManaCost::empty().with_generic_delta(x),
    }]
}
