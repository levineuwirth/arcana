//! Parnesse, the Subtle Brush — `{2}{U}{B}{R}` 4/4 Legendary Vampire Wizard.
//!
//! Oracle:
//! * Whenever you or a permanent you control becomes the target of a spell or
//!   ability an opponent controls, counter that spell or ability unless that
//!   player pays 4 life.
//! * Whenever you copy a spell, up to one target opponent may also copy that
//!   spell. They may choose new targets for that copy.
//!
//! Both abilities are GAP'd at the effect level:
//! * The first watches "you OR a permanent you control" becoming a target.
//!   `SelfBecomesTarget` only fires for THIS creature, and there is no
//!   "counter that spell/ability unless they pay life" effect primitive, so
//!   the whole effect is unexpressible — emitted as a GAP'd trigger using the
//!   closest condition for catalog presence.
//! * The second triggers on copying a spell ("Whenever you copy a spell"),
//!   which has no matching `TriggerCondition` variant, and the "another player
//!   may copy that spell" payload has no effect primitive — fully GAP'd.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Parnesse, the Subtle Brush");
    let vampire = reg.interner_mut().intern("Vampire");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(vampire);
    subtypes.0.insert(wizard);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}{B}{R}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::black() | ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                // Closest condition: this becomes the target of an opponent's
                // spell/ability. Oracle is broader ("you or a permanent you
                // control"); the effect is GAP'd regardless.
                trigger_condition: TriggerCondition::SelfBecomesTarget {
                    caster: ControllerConstraint::Opponent,
                },
                intervening_if: None,
                effect: counter_unless_pay,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                // GAP: trigger — "Whenever you copy a spell" has no matching
                // TriggerCondition variant; using SpellCast(You) as a placeholder
                // for catalog presence. Effect is fully GAP'd.
                trigger_condition: TriggerCondition::SpellCast {
                    filter: None,
                    caster: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: copy_rider,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn counter_unless_pay(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "counter that spell or ability unless that player pays 4 life" — no
    // counter-target-spell/ability effect primitive, and the trigger watches a
    // broader subject ("you or a permanent you control") than expressible.
    Vec::new()
}

fn copy_rider(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "up to one target opponent may also copy that spell" — no copy-grant
    // effect primitive; the copy-a-spell trigger itself is also unmodeled.
    Vec::new()
}
