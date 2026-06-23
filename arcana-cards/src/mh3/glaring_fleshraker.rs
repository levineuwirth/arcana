//! Glaring Fleshraker — `{2}{C}` 2/2 Creature — Eldrazi Drone.
//!
//! * Whenever you cast a colorless spell, create a 0/1 colorless Eldrazi Spawn
//!   creature token with "Sacrifice this token: Add {C}."
//! * Whenever another colorless creature you control enters, this creature
//!   deals 1 damage to each opponent.
//!
//! GAP (fidelity): the Eldrazi Spawn token's "Sacrifice this token: Add {C}"
//! activated ability can't be wired — `TokenDefinition.abilities` holds only
//! triggered abilities, not activated ones. The bare token is created.
//! The "another" self-exclusion on the second trigger isn't expressible on the
//! ZoneChange filter (documented fidelity gap).

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Glaring Fleshraker");
    let eldrazi = reg.interner_mut().intern("Eldrazi");
    let drone = reg.interner_mut().intern("Drone");
    let _ = reg.interner_mut().intern("Spawn");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(eldrazi);
    subtypes.0.insert(drone);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{C}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SpellCast {
                    filter: Some(ObjectFilter::new().with_colors(ColorSet::colorless())),
                    caster: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: make_eldrazi_spawn,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: ObjectFilter::creature()
                        .with_colors(ColorSet::colorless())
                        .controlled_by(ControllerConstraint::You),
                    from: None,
                    to: Zone::Battlefield,
                },
                intervening_if: None,
                effect: damage_each_opponent,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn make_eldrazi_spawn(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: the token's "Sacrifice this token: Add {C}" activated ability is not
    // expressible on TokenDefinition (which carries only triggered abilities).
    let spawn = reg.interner().lookup("Spawn").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spawn);
    vec![Effect::CreateToken {
        controller: trig.controller,
        token: TokenDefinition {
            name: spawn,
            colors: ColorSet::colorless(),
            types: TypeLine::CREATURE.into(),
            subtypes,
            power: Some(PtValue::Fixed(0)),
            toughness: Some(PtValue::Fixed(1)),
            keywords: vec![],
            abilities: vec![],
        },
    }]
}

fn damage_each_opponent(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let effects: Vec<Effect> = script::opponents(state, trig.controller)
        .into_iter()
        .map(|p| Effect::DealDamage {
            source: trig.source,
            target: DamageTarget::Player(p),
            amount: 1,
        })
        .collect();
    vec![Effect::Sequence(effects)]
}
