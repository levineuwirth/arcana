//! Nullstone Gargoyle — `{9}` 4/5 Artifact Creature — Gargoyle with Flying.
//!
//! Oracle:
//! * Flying.
//! * Whenever the first noncreature spell of a turn is cast, counter that
//!   spell.
//!
//! Flying is a base characteristic. The trigger watches a noncreature spell
//! being cast (expressible via `SpellCast` with a noncreature filter), but two
//! pieces are not expressible: (1) the "FIRST … of a turn" qualifier has no
//! per-turn-first gate in the trigger/intervening-if surface, and (2) there is
//! no counter-target-spell effect in the engine effect catalog (no
//! `Effect::Counter`). So the effect body is GAP'd; we still register the
//! trigger shape against the closest matching condition.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Nullstone Gargoyle");
    let gargoyle = reg.interner_mut().intern("Gargoyle");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(gargoyle);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{9}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SpellCast {
                    filter: Some(ObjectFilter::new().without_types(TypeLine::CREATURE.into())),
                    caster: ControllerConstraint::Any,
                },
                intervening_if: None,
                effect: counter_first_noncreature_spell,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn counter_first_noncreature_spell(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "counter that spell" has no engine effect primitive (the effect
    // catalog exposes CopySpell but no counter-spell effect), and the "FIRST …
    // of a turn" qualifier has no per-turn-first trigger/intervening-if gate.
    // The trigger shape is registered against the closest condition; the body
    // is GAP'd rather than firing on every noncreature spell.
    Vec::new()
}
