//! Vine Gecko — `{1}{G}` 2/2 Elemental Lizard.
//!
//! Oracle:
//! * "The first kicked spell you cast each turn costs {1} less to cast."
//!   A static cost-reduction keyed on the kicker mechanic — no demonstrated
//!   Effect/static primitive expresses a per-turn-first cost reduction gated
//!   on whether the cast spell was kicked. GAP (see below).
//! * "Whenever you cast a kicked spell, put a +1/+1 counter on this creature."
//!   A `SpellCast` trigger, but `ObjectFilter` has no "was-kicked" predicate in
//!   the demonstrated API, so the trigger cannot be restricted to KICKED
//!   casts. Firing on every spell would be a materially wrong card, so the
//!   effect is GAP'd rather than over-firing. The trigger shell is still
//!   emitted (closest condition: any spell you cast) with an empty resolver +
//!   GAP note, so the cast hook is recorded.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Vine Gecko");
    let elemental = reg.interner_mut().intern("Elemental");
    let lizard = reg.interner_mut().intern("Lizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elemental);
    subtypes.0.insert(lizard);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    // GAP: static — "The first kicked spell you cast each turn costs {1} less
    // to cast." No demonstrated cost-reduction primitive gated on the kicker
    // mechanic + a per-turn-first restriction.
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SpellCast {
                    filter: None,
                    caster: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: on_cast_kicked_spell,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn on_cast_kicked_spell(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: trigger should fire only when the cast spell was KICKED. No
    // demonstrated ObjectFilter / SpellCast predicate distinguishes a kicked
    // cast from an ordinary one, so the +1/+1 counter is withheld rather than
    // applied to every spell.
    Vec::new()
}
