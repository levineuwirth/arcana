//! Flame-Kin War Scout — `{3}{R}` 2/4 red Elemental Scout. "When
//! another creature enters, sacrifice this creature. If you do, this
//! creature deals 4 damage to that creature."
//!
//! Modelled as a battlefield-bound `ZoneChange` trigger filtered to
//! creatures, with a self-id guard in the resolver to enforce the
//! "another creature" clause. The "sacrifice this creature, if you
//! do, …" rider has no first-class `SacrificeSelf` effect in the
//! catalog; the resolver substitutes `DestroyPermanent` on the
//! source as the closest available stand-in and notes the gap. The
//! 4 damage is sourced from this creature (LKI carries the source
//! through its own destruction).

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Flame-Kin War Scout");
    let elemental = reg.interner_mut().intern("Elemental");
    let scout = reg.interner_mut().intern("Scout");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elemental);
    subtypes.0.insert(scout);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: ObjectFilter::creature(),
                    from: None,
                    to: Zone::Battlefield,
                },
                intervening_if: None,
                effect: on_another_creature_enters,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

/// "When another creature enters, sacrifice this creature. If you
/// do, this creature deals 4 damage to that creature." The
/// `another` rider is enforced by comparing the entering object's
/// id against `trig.source`. The sacrifice is approximated with
/// `DestroyPermanent` on the source; the 4 damage to the entering
/// creature carries `trig.source` as its source via LKI.
fn on_another_creature_enters(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(entered) = trig.entering_object() else {
        return Vec::new();
    };
    if entered == trig.source {
        return Vec::new();
    }
    vec![
        // GAP: no SacrificeSelf effect in the catalog; DestroyPermanent
        // on the source is used as the closest stand-in for the
        // self-sacrifice clause.
        Effect::DestroyPermanent { target: trig.source },
        Effect::DealDamage {
            target: DamageTarget::Object(entered),
            amount: 4,
            source: trig.source,
        },
    ]
}
