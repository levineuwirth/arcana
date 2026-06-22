//! Genestealer Patriarch — `{4}{U}` 4/4 Tyranid.
//!
//! Oracle:
//! * "Genestealer's Kiss — Whenever this creature attacks, put an
//!   infection counter on target creature defending player controls." — an
//!   attack trigger targeting an opponent's creature (the defending
//!   player's creature, approximated as a creature an opponent controls);
//!   add a named "infection" counter.
//! * "Children of the Cult — Whenever a creature with an infection counter
//!   on it dies, you create a token that's a copy of that creature, except
//!   it's a Tyranid in addition to its other types." — a death trigger
//!   filtered to creatures bearing an infection counter; mint a token copy
//!   of the dying creature via CopyPermanent. (The "except it's a Tyranid"
//!   type rider is not expressible on the copy — partial.)

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Genestealer Patriarch");
    let tyranid = reg.interner_mut().intern("Tyranid");
    let infection = reg.interner_mut().intern("infection");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(tyranid);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![],
        ..Default::default()
    };

    let mut infected_creature = ObjectFilter::creature();
    infected_creature.has_counter = Some(CounterKind::Named(infection));

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: attacks_infect,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::creature()
                            .controlled_by(ControllerConstraint::Opponent),
                    ),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: infected_creature,
                    from: Some(Zone::Battlefield),
                    to: Zone::Graveyard(0),
                },
                intervening_if: None,
                effect: infected_dies_copy,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn attacks_infect(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    let Some(kind) = reg.interner().lookup("infection").map(CounterKind::Named) else {
        return Vec::new();
    };
    vec![Effect::AddCounters { target: *id, kind, count: 1 }]
}

fn infected_dies_copy(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // "create a token that's a copy of that creature" — CopyPermanent of
    // the dying creature. The "except it's a Tyranid" type rider on the
    // copy is not expressible (partial).
    let Some(dying) = trig.dying_object() else { return Vec::new(); };
    vec![Effect::CopyPermanent { target: dying }]
}
