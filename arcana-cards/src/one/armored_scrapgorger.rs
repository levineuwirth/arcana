//! Armored Scrapgorger — `{1}{G}` 0/3 Phyrexian Beast.
//! "This creature gets +3/+0 as long as it has three or more oil counters on
//! it." (conditional static P/T — GAP'd)
//! "{T}: Add one mana of any color."
//! "Whenever this creature becomes tapped, exile target card from a graveyard
//! and put an oil counter on this creature."

use arcana_core::effects::Effect;
use arcana_core::mana::{ManaCost, ManaUnit};
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, ManaColor, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Armored Scrapgorger");
    let phyrexian = reg.interner_mut().intern("Phyrexian");
    let beast = reg.interner_mut().intern("Beast");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(phyrexian);
    subtypes.0.insert(beast);
    // Pre-intern the oil counter name for the trigger's AddCounters.
    let _oil = reg.interner_mut().intern("oil");

    // GAP (static): "+3/+0 as long as it has three or more oil counters" — no
    // counter-conditional static P/T primitive in the available surface.

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}: Add one mana of any color.".into(),
                cost: ActivationCost { tap: true, ..ActivationCost::default() },
                target_requirements: Vec::new(),
                is_mana_ability: true,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: add_mana_any,
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfBecomesTapped,
                intervening_if: None,
                effect: exile_and_oil,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Card {
                        zone: Zone::Graveyard(0),
                        filter: ObjectFilter::default(),
                    },
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
            }),
    )
}

fn add_mana_any(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP (fidelity): any-color choice modeled as one colorless mana
    // (no any-color mana primitive in the available surface).
    vec![Effect::AddMana {
        player: ctx.controller,
        mana: vec![ManaUnit::plain(ManaColor::Colorless, ctx.source)],
    }]
}

fn exile_and_oil(_state: &GameState, trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect> {
    let mut effects = Vec::new();
    if let Some(TargetChoice::Object(id)) = trig.targets.targets.first() {
        effects.push(Effect::ExileFromGraveyard { target: *id });
    }
    if let Some(oil) = reg.interner().lookup("oil") {
        effects.push(Effect::AddCounters {
            target: trig.source,
            kind: CounterKind::Named(oil),
            count: 1,
        });
    }
    effects
}
