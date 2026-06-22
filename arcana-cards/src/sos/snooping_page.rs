//! Snooping Page — `{1}{W}{B}` 2/3 Human Cleric.
//!
//! * Repartee — Whenever you cast an instant or sorcery spell that targets a
//!   creature, this creature can't be blocked this turn.
//! * Whenever this creature deals combat damage to a player, you draw a card
//!   and lose 1 life.
//!
//! "Repartee" is an ability word (no keyword). The first trigger uses a
//! `SpellCast` filtered to instant-or-sorcery; the "that targets a creature"
//! sub-restriction is not filterable (fidelity GAP — it over-fires for
//! instants/sorceries that don't target a creature). The second trigger uses
//! `DamageDealt` to a player with `combat_only: true`; following codebase
//! precedent (Fungal Shambler), the self-only source restriction is an
//! `ObjectFilter::new()` approximation (over-fires for other sources).

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Snooping Page");
    let human = reg.interner_mut().intern("Human");
    let cleric = reg.interner_mut().intern("Cleric");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(cleric);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}{B}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            // Repartee — cast instant/sorcery targeting a creature → unblockable.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SpellCast {
                    // GAP: "that targets a creature" sub-restriction is not
                    //       filterable; this fires on any instant/sorcery cast.
                    filter: Some(ObjectFilter::new().with_types_any(TypeLine(
                        TypeLine::INSTANT | TypeLine::SORCERY,
                    ))),
                    caster: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: make_unblockable,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // Combat damage to a player → draw a card and lose 1 life.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::DamageDealt {
                    // GAP: self-only source restriction not expressible via
                    //       ObjectFilter; ObjectFilter::new() over-fires.
                    source_filter: ObjectFilter::new(),
                    target_filter: TargetFilter::Player,
                    combat_only: true,
                },
                intervening_if: None,
                effect: draw_and_lose_life,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn make_unblockable(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::CantBeBlocked {
        target: trig.source,
        duration: Duration::EndOfTurn,
    }]
}

fn draw_and_lose_life(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![
        Effect::DrawCards { player: trig.controller, count: 1 },
        Effect::LoseLife { player: trig.controller, amount: 1 },
    ]
}
