//! Bull Aurochs — `{1}{G}` 2/1 Aurochs.
//!
//! Oracle:
//! * Trample
//! * "Whenever this creature attacks, it gets +1/+0 until end of turn
//!   for each other attacking Aurochs."
//!
//! Trample is emitted. The SelfAttacks trigger is wired, but the
//! dynamic pump amount — "+1/+0 for each OTHER ATTACKING Aurochs" — is
//! GAP'd: the `script::` helpers can count Aurochs you control (and
//! subtype filters), but there is no helper that counts ATTACKING
//! creatures of a subtype. Counting all Aurochs you control would be
//! materially wrong (it must be the attackers, minus self), so the
//! effect returns `Vec::new()`.

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
    let name = reg.interner_mut().intern("Bull Aurochs");
    let aurochs = reg.interner_mut().intern("Aurochs");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(aurochs);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Trample],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfAttacks,
            intervening_if: None,
            effect: attack_pump,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn attack_pump(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "+1/+0 for each OTHER ATTACKING Aurochs" — no script helper
    // counts attacking creatures of a subtype; the all-you-control count
    // would be wrong, so the whole effect is GAP'd.
    Vec::new()
}
