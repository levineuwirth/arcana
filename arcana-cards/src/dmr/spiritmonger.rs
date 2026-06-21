//! Spiritmonger — `{3}{B}{G}` 6/6 Beast.
//!
//! "Whenever this creature deals damage to a creature, put a +1/+1 counter
//! on this creature."
//! "{B}: Regenerate this creature."
//! "{G}: This creature becomes the color of your choice until end of turn."
//!   (GAP — no documented way to express the player's color choice)

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Spiritmonger");
    let beast = reg.interner_mut().intern("Beast");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(beast);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}{G}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(6)),
        toughness: Some(PtValue::Fixed(6)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                // GAP (over-fire): no self-only damage-source filter exists, so
                // a broad creature source-filter is used; this may fire when
                // another creature deals damage to a creature. The "+1/+1
                // counter on this creature" payload is faithful.
                trigger_condition: TriggerCondition::DamageDealt {
                    source_filter: ObjectFilter::creature(),
                    target_filter: TargetFilter::Creature,
                    combat_only: false,
                },
                intervening_if: None,
                effect: grow,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{B}: Regenerate this creature.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{B}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: regen,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{G}: This creature becomes the color of your choice until end of turn.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{G}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: become_color,
            }),
    )
}

fn grow(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::AddCounters {
        target: trig.source,
        kind: CounterKind::PlusOnePlusOne,
        count: 1,
    }]
}

fn regen(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::Regenerate { target: ctx.source }]
}

fn become_color(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "becomes the color of your choice" — no documented way to capture
    // the player's chosen color (SetColor needs a fixed ColorSet; there is no
    // ChosenColor target variant).
    Vec::new()
}
