//! Assemble the Legion — `{3}{R}{W}` enchantment (Gatecrash, 2013).
//! "At the beginning of your upkeep, put a muster counter on this
//! enchantment. Then create a 1/1 red and white Soldier creature token
//! with haste for each muster counter on this enchantment."
//!
//! Counter-accumulation enchantment: a your-upkeep `StepBegins` trigger
//! adds one `muster` counter (`CounterKind::Named`), then mints one
//! 1/1 red-and-white Soldier with haste per muster counter. The count
//! is read via `script::source_counter_count`, which observes state
//! BEFORE this resolution's own `AddCounters` applies — so we add the
//! printed `+1` for the counter we are placing this turn (CR 121.2,
//! the token count includes the just-added counter). One
//! `Effect::CreateToken` is emitted per Soldier (there is no count
//! field), wrapped in `Effect::Sequence` after the `AddCounters`.

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Assemble the Legion");
    let _muster = reg.interner_mut().intern("muster");
    let _soldier = reg.interner_mut().intern("Soldier");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}{W}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::white(),
        types: TypeLine::ENCHANTMENT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(
            TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::Upkeep,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: muster_and_assemble,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            },
        ),
    )
}

/// "…put a muster counter on this enchantment. Then create a 1/1 red and
/// white Soldier creature token with haste for each muster counter on
/// this enchantment." The just-added counter counts toward the total, so
/// `+1` past the pre-resolution count.
fn muster_and_assemble(
    state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let muster = reg.interner().lookup("muster")
        .expect("muster interned during register()");
    let soldier = reg.interner().lookup("Soldier").unwrap_or_default();

    let n = script::source_counter_count(
        state,
        trig.source,
        CounterKind::Named(muster),
    ) + 1;

    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(soldier);

    let mut effects = Vec::with_capacity(1 + n as usize);
    effects.push(Effect::AddCounters {
        target: trig.source,
        kind: CounterKind::Named(muster),
        count: 1,
    });
    for _ in 0..n {
        effects.push(Effect::CreateToken {
            controller: trig.controller,
            token: TokenDefinition {
                name: soldier,
                colors: ColorSet::red() | ColorSet::white(),
                types: TypeLine::CREATURE.into(),
                subtypes: subtypes.clone(),
                power: Some(PtValue::Fixed(1)),
                toughness: Some(PtValue::Fixed(1)),
                keywords: vec![KeywordAbility::Haste],
                abilities: vec![],
            },
        });
    }
    vec![Effect::Sequence(effects)]
}
