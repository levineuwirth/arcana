//! Snarlfang Vermin — `{B}` 2/1 Rat.
//! "Whenever Snarlfang Vermin deals combat damage to a creature, if that
//! creature is still on the battlefield, suspect that creature. It perpetually
//! gains this ability."
//! "Whenever a suspected creature an opponent controls dies while Snarlfang
//! Vermin is in your graveyard, that opponent loses 1 life."

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Snarlfang Vermin");
    let rat = reg.interner_mut().intern("Rat");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(rat);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::DamageDealt {
                    source_filter: ObjectFilter::new(),
                    target_filter: TargetFilter::Creature,
                    combat_only: true,
                },
                intervening_if: None,
                effect: suspect_damaged_creature,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                // GAP (filter): "suspected creature" — ObjectFilter has no
                // `suspected` predicate; restricted only to an opponent's
                // creatures dying. The "while ~ is in your graveyard" zone gate
                // and "that opponent loses 1 life" rider are unexpressible below.
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: ObjectFilter::creature()
                        .controlled_by(ControllerConstraint::Opponent),
                    from: Some(Zone::Battlefield),
                    to: Zone::Graveyard(0),
                },
                intervening_if: None,
                effect: suspected_dies_drain,
                trigger_zones: vec![Zone::Graveyard(0)],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn suspect_damaged_creature(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "suspect that creature [it dealt damage to]. It perpetually gains
    // this ability." No PendingTrigger accessor exposes the combat-damaged
    // creature's id, and perpetual ability-granting from graveyard is unmodeled.
    Vec::new()
}

fn suspected_dies_drain(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "while Snarlfang Vermin is in your graveyard, that opponent loses 1
    // life" — the controller-relative life-loss target (the dying creature's
    // controller) plus the graveyard-presence gate aren't jointly expressible.
    Vec::new()
}
