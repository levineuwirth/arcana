//! Alania, Divergent Storm — `{3}{U}{R}` 3/5 Legendary Creature — Otter
//! Wizard (red/blue). "Whenever you cast a spell, if it's the first instant
//! spell, the first sorcery spell, or the first Otter spell other than Alania
//! you've cast this turn, you may have target opponent draw a card. If you do,
//! copy that spell. You may choose new targets for the copy."
//!
//! Modeled as a `SpellCast` (you) trigger targeting an opponent who draws a
//! card. The intervening-if "first instant / first sorcery / first Otter
//! spell this turn" is GAP'd (no per-spell-type first-of-turn predicate in the
//! conditions surface — fires on every cast). The "copy that spell" rider is
//! GAP'd: the triggering spell's stack id is not exposed via a PendingTrigger
//! accessor, so `Effect::CopySpell { target }` cannot reference it.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Alania, Divergent Storm");
    let otter = reg.interner_mut().intern("Otter");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(otter);
    subtypes.0.insert(wizard);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}{R}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(5)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                // GAP: intervening-if "first instant / first sorcery / first
                // Otter spell other than Alania you've cast this turn" — no
                // per-spell-type first-of-turn predicate in conditions; fires
                // on every cast.
                trigger_condition: TriggerCondition::SpellCast {
                    filter: None,
                    caster: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: opponent_draws_then_copy,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Player,
                    count: TargetCount::Exactly(1),
                    controller: Some(ControllerConstraint::Opponent),
                }],
            }),
    )
}

fn opponent_draws_then_copy(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Player(p) = target else { return Vec::new(); };
    // "you may have target opponent draw a card" — the optional ("may") and
    // the "if you do, copy that spell" rider are GAP'd: the triggering
    // spell's stack id is not exposed via a PendingTrigger accessor, so
    // Effect::CopySpell { target } cannot reference it.
    vec![Effect::DrawCards { player: *p, count: 1 }]
}
