//! Zopandrel, Hunger Dominus — `{5}{G}{G}` 4/6 Legendary Phyrexian
//! Horror with Reach.
//! Ability 1 ("At the beginning of each combat, double the power and
//! toughness of each creature you control until end of turn.") doubles
//! every creature's current P/T — a per-object dynamic P/T-doubling
//! effect with no demonstrated primitive (Pump/SetBasePT take fixed
//! amounts), so it is GAP'd.
//! Ability 2 ("{G/P}{G/P}, Sacrifice two other creatures: Put an
//! indestructible counter on Zopandrel.") wired as an activated ability
//! whose cost is two Phyrexian-green pips plus sacrificing two other
//! creatures; the indestructible counter is a named counter.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Phase;
use arcana_core::targets::ControllerConstraint;
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Zopandrel, Hunger Dominus");
    let phyrexian = reg.interner_mut().intern("Phyrexian");
    let horror = reg.interner_mut().intern("Horror");
    // Pre-intern the named counter so the resolver's lookup succeeds.
    let _ = reg.interner_mut().intern("indestructible");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(phyrexian);
    subtypes.0.insert(horror);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(6)),
        keywords: vec![KeywordAbility::Reach],
        ..Default::default()
    };

    let creature_filter = ObjectFilter::creature();

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::PhaseBegins {
                    phase: Phase::Combat,
                    whose: ControllerConstraint::Any,
                },
                intervening_if: None,
                effect: double_creatures,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{G/P}{G/P}, Sacrifice two other creatures: Put an indestructible counter on Zopandrel.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{G/P}{G/P}").expect("valid cost"),
                    sacrifice_other: Some(creature_filter),
                    sacrifice_other_count: 2,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: add_indestructible_counter,
            }),
    )
}

fn double_creatures(_state: &GameState, _trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "double the power and toughness of each creature you control
    // until end of turn" — per-object dynamic P/T doubling has no
    // demonstrated primitive (Pump / SetBasePT take fixed amounts).
    Vec::new()
}

fn add_indestructible_counter(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let kind = reg
        .interner()
        .lookup("indestructible")
        .map(CounterKind::Named)
        .unwrap_or(CounterKind::Shield);
    vec![Effect::AddCounters {
        target: ctx.source,
        kind,
        count: 1,
    }]
}
