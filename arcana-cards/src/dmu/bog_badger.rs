//! Bog Badger — `{2}{G}` 3/3 green Badger.
//!
//! Oracle:
//! * Kicker {B} — an alternative/additional casting cost. This is a static
//!   casting modifier, not a triggered or activated ability.
//!   GAP: Kicker is not in the usable `KeywordAbility` surface and the
//!   "pay an additional {B}" casting modifier is not expressible — `keywords: vec![]`.
//! * "When this creature enters, if it was kicked, creatures you control gain
//!   menace until end of turn." This is an ETB trigger gated on the
//!   intervening-if "if it was kicked". There is no `conditions::` predicate
//!   (nor any state accessor in the demonstrated API) for "this spell was
//!   kicked", so the gate is unexpressible — and firing the menace grant
//!   unconditionally would be a materially wrong card. GAP the whole trigger.

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
    let name = reg.interner_mut().intern("Bog Badger");
    let badger = reg.interner_mut().intern("Badger");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(badger);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_if_kicked_menace,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_if_kicked_menace(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "if it was kicked" intervening-if is unexpressible (no kicked-state
    // predicate in conditions:: / no kicked accessor). Firing the menace grant
    // unconditionally would be materially wrong, so the whole effect is gapped.
    Vec::new()
}
