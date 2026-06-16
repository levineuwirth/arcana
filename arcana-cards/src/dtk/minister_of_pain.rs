//! Minister of Pain — `{2}{B}` 2/3 Human Shaman (black).
//!
//! Oracle:
//! * "Exploit (When this creature enters, you may sacrifice a creature.)" —
//!   the Exploit keyword. There is no `KeywordAbility::Exploit` variant and
//!   no exploit-sacrifice ETB form is expressible on this card class, so
//!   `keywords` is empty and the exploit sacrifice itself is GAP'd.
//! * "When this creature exploits a creature, creatures your opponents
//!   control get -1/-1 until end of turn." — the EXPLOIT trigger has no
//!   matching `TriggerCondition` variant (the closest, `SelfEntersBattlefield`,
//!   over-fires: it can't gate on "actually exploited a creature"). We wire
//!   the closest trigger and GAP it; the -1/-1 payload would itself be
//!   expressible (ForEach -1/-1 to opponents' creatures), but firing it
//!   unconditionally would be a materially wrong card, so the whole effect
//!   is GAP'd.

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
    let name = reg.interner_mut().intern("Minister of Pain");
    let human = reg.interner_mut().intern("Human");
    let shaman = reg.interner_mut().intern("Shaman");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(shaman);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                // GAP: trigger — "when this creature exploits a creature" has
                // no matching TriggerCondition; SelfEntersBattlefield is the
                // closest but over-fires (can't gate on the exploit sacrifice).
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: on_exploit,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn on_exploit(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: the -1/-1 sweep is gated on actually exploiting a creature, which
    // the engine cannot detect on this card class; firing it on every ETB
    // would be a materially wrong card, so the whole effect is GAP'd.
    Vec::new()
}
