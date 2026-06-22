//! Berta, Wise Extrapolator — `{2}{G}{U}` 1/4 Legendary Creature — Frog Druid.
//!
//! * Increment — GAP: not a `KeywordAbility` variant; its triggered counter
//!   placement (cast a spell whose spent mana exceeds Berta's power/toughness
//!   → +1/+1 counter) is not expressible with the documented primitives
//!   (no "mana spent on the spell" accessor), so it is omitted.
//! * `Whenever one or more +1/+1 counters are put on Berta, add one mana of
//!   any color.` Trigger is expressible (`CounterAdded`), but "add one mana of
//!   ANY color" is not (no `ManaColor::Any`; only fixed-color `ManaUnit`s),
//!   so its resolver is GAP'd.
//! * `{X}, {T}: Create a 0/0 green and blue Fractal creature token and put X
//!   +1/+1 counters on it.` The token is created; "put X +1/+1 counters on
//!   it" is GAP'd — no primitive returns the freshly-minted token's id to
//!   target with `AddCounters`.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggerSelf,
    TriggeredAbilityDef,
};
use arcana_core::types::{
    CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Berta, Wise Extrapolator");
    let frog = reg.interner_mut().intern("Frog");
    let druid = reg.interner_mut().intern("Druid");
    // Pre-intern the token subtype so the resolver's read-only lookup finds it.
    let _fractal = reg.interner_mut().intern("Fractal");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(frog);
    subtypes.0.insert(druid);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}{U}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::CounterAdded {
                    on: TriggerSelf::Source,
                    kind: Some(CounterKind::PlusOnePlusOne),
                    chapter: None,
                },
                intervening_if: None,
                effect: add_any_color_mana,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{X}, {T}: Create a 0/0 green and blue Fractal creature token and put X +1/+1 counters on it.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{X}").expect("valid cost"),
                    tap: true,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: make_fractal,
            }),
    )
}

fn add_any_color_mana(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "add one mana of any color" — no `ManaColor::Any`; only
    // fixed-color `ManaUnit`s are expressible.
    Vec::new()
}

fn make_fractal(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let fractal = reg.interner().lookup("Fractal").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(fractal);
    // GAP: "put X +1/+1 counters on it" — the freshly-minted token's id is
    // not returned by CreateToken, so the counters can't be placed on it.
    vec![Effect::CreateToken {
        controller: ctx.controller,
        token: arcana_core::effects::TokenDefinition {
            name: fractal,
            colors: ColorSet::green() | ColorSet::blue(),
            types: TypeLine::CREATURE.into(),
            subtypes,
            power: Some(PtValue::Fixed(0)),
            toughness: Some(PtValue::Fixed(0)),
            keywords: vec![],
            abilities: vec![],
        },
    }]
}
