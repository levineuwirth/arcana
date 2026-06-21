//! Custodi Soulcaller — `{1}{W}{W}` 1/2 Human Cleric.
//!
//! Melee.
//! Whenever this creature attacks, return target creature card with mana value
//! X or less from your graveyard to the battlefield, where X is the number of
//! players you attacked this combat.
//!
//! GAP (keyword): Melee is not in the usable keyword surface (no
//! `KeywordAbility::Melee`).
//! GAP (effect): the attack trigger's payload is doubly inexpressible — there
//! is no `script::` accessor for "the number of players you attacked this
//! combat" (X), and an `ObjectFilter`'s `with_max_cmc` is fixed at registration
//! so the mana-value cap cannot be parametrized by that runtime X. The
//! SelfAttacks trigger is wired; its reanimation body is the gap.

use arcana_core::effects::Effect;
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
    let name = reg.interner_mut().intern("Custodi Soulcaller");
    let human = reg.interner_mut().intern("Human");
    let cleric = reg.interner_mut().intern("Cleric");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(cleric);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfAttacks,
            intervening_if: None,
            effect: attack_reanimate,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

/// GAP: reanimate-with-runtime-mv-cap is not expressible (see module doc).
fn attack_reanimate(_state: &GameState, _trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    Vec::new()
}
