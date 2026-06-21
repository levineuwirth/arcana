//! Marrow Chomper — `{3}{B}{G}` 3/3 Zombie Lizard with Devour 2.
//! "Devour 2 (As this creature enters, you may sacrifice any number of
//!  creatures. It enters with twice that many +1/+1 counters on it.)"
//! "When this creature enters, you gain 2 life for each creature it devoured."
//!
//! Devour 2 is a usable parametrized keyword. The ETB lifegain scales with the
//! number of creatures devoured, but no pending-trigger / script accessor
//! exposes the devoured count, so the amount is not expressible — the effect is
//! GAP'd rather than hardcoding a wrong literal.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Marrow Chomper");
    let zombie = reg.interner_mut().intern("Zombie");
    let lizard = reg.interner_mut().intern("Lizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(zombie);
    subtypes.0.insert(lizard);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}{G}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Devour(2)],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_gain_per_devoured,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_gain_per_devoured(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "gain 2 life for each creature it devoured" — no accessor exposes the
    // number of creatures sacrificed to Devour; the scaling amount is not
    // computable, so the effect is omitted.
    Vec::new()
}
