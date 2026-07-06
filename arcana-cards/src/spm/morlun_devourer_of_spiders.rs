//! Morlun, Devourer of Spiders — `{X}{B}{B}` 2/1 Legendary Vampire Villain.
//!
//! Oracle:
//! Lifelink
//! Morlun enters with X +1/+1 counters on him.
//! When Morlun enters, he deals X damage to target opponent.
//!
//! Decomposition: Lifelink keyword + an "enters with X +1/+1 counters" static
//! and an ETB damage trigger. Both X-dependent clauses reference X = the value
//! chosen for the `{X}` in the cast cost; that cast-time X is not exposed to a
//! triggered ability's effect fn in the demonstrated API (no `trig.x_value`,
//! and `with_trigger_dynamic_x` reads trigger-EVENT data, not the spell's X).
//! So:
//!  - "enters with X +1/+1 counters" — an enters-with replacement; GAP'd (X
//!    unexpressible, and enters-with is not a triggered/activated ability).
//!  - the ETB "deals X damage to target opponent" trigger is wired with its
//!    target (a player), but its effect is GAP'd because X is unavailable.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::TargetRequirement;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{
    CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

// GAP: "Morlun enters with X +1/+1 counters on him" — an enters-with
// replacement (not a triggered/activated ability), and the cast-time X is not
// exposed in the demonstrated API.

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Morlun, Devourer of Spiders");
    let vampire = reg.interner_mut().intern("Vampire");
    let villain = reg.interner_mut().intern("Villain");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(vampire);
    subtypes.0.insert(villain);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{X}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Lifelink],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: etb_deal_x_damage,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: vec![TargetRequirement::target_opponent()],
        }),
    )
}

fn etb_deal_x_damage(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "deals X damage to target opponent" — X is the cast-time `{X}`
    // value, which is not exposed to a triggered ability's effect fn in the
    // demonstrated API (no `trig.x_value`; `with_trigger_dynamic_x` reads
    // trigger-event data, not the spell's X).
    Vec::new()
}
