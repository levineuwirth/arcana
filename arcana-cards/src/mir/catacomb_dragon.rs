//! Catacomb Dragon — `{4}{B}{B}` 4/4 Dragon with Flying.
//!
//! Oracle:
//! * Flying
//! * Whenever this creature becomes blocked by a nonartifact, non-Dragon
//!   creature, that creature gets -X/-0 until end of turn, where X is half the
//!   creature's power, rounded down.
//!
//! Flying is a base keyword. The block trigger fires on `SelfBecomesBlocked`
//! and reads the blocking creature via `trig.other_combatant()`, then pumps it
//! by -X/-0 where X = floor(blocker power / 2) computed at resolution with
//! `script::power_of`.
//!
//! PARTIAL: the "nonartifact, non-Dragon creature" restriction on the blocker
//! is not applied — `SelfBecomesBlocked` carries no blocker filter in the
//! demonstrated catalog, so the debuff fires for any blocker.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Catacomb Dragon");
    let dragon = reg.interner_mut().intern("Dragon");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dragon);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfBecomesBlocked,
            intervening_if: None,
            effect: weaken_blocker,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn weaken_blocker(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(blocker) = trig.other_combatant() else {
        return Vec::new();
    };
    let x = (script::power_of(state, blocker).max(0) as u32) / 2;
    vec![Effect::Pump {
        target: blocker,
        power: -(x as i32),
        toughness: 0,
        duration: Duration::EndOfTurn,
        keywords: vec![],
    }]
}
