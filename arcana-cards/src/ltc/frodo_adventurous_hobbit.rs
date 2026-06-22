//! Frodo, Adventurous Hobbit — `{W}{B}` 1/3 Legendary Halfling Scout.
//!
//! * Partner with Sam, Loyal Attendant / Partner — not in the supported
//!   keyword surface; GAP.
//! * Vigilance (keyword).
//! * "Whenever Frodo attacks, if you gained 3 or more life this turn,
//!   the Ring tempts you. Then if Frodo is your Ring-bearer and the
//!   Ring has tempted you two or more times this game, draw a card." —
//!   the Ring-temptation / Ring-bearer mechanic has no primitive, and
//!   the life-gained-this-turn intervening-if has no condition helper.
//!   The SelfAttacks trigger is recorded but its effect is GAP'd
//!   (returns no effects).

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{
    CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Frodo, Adventurous Hobbit");
    let halfling = reg.interner_mut().intern("Halfling");
    let scout = reg.interner_mut().intern("Scout");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(halfling);
    subtypes.0.insert(scout);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{W}{B}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(3)),
        // GAP: Partner / Partner with not in supported keyword surface.
        keywords: vec![KeywordAbility::Vigilance],
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfAttacks,
                // GAP: "if you gained 3 or more life this turn" — no
                // condition helper for life-gained-this-turn.
                intervening_if: None,
                effect: ring_temptation,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn ring_temptation(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "the Ring tempts you" + Ring-bearer / tempted-count tracking
    // have no engine primitive; the conditional draw can't be expressed.
    Vec::new()
}
