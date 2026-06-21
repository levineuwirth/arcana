//! Tobias, Doomed Conqueror — `{2}{W}{U}` 3/2 Legendary Creature —
//! Human Soldier with Flash.
//!
//! Oracle:
//! * Flash — base keyword.
//! * When Tobias dies, for each nontoken creature you controlled that died this
//!   turn, create a 2/2 black Zombie creature token. — a dies trigger; the count
//!   is DYNAMIC ("for each nontoken creature you controlled that died this
//!   turn"). There is no `script::*` helper that counts nontoken creatures you
//!   controlled that died this turn (only `creatures_of_subtype_died_this_turn`
//!   exists, which is subtype-keyed and counts all controllers). Emitting a
//!   fixed token count would be a materially wrong card, so the whole effect is
//!   GAP'd while the dies-trigger shape is recorded.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Tobias, Doomed Conqueror");
    let human = reg.interner_mut().intern("Human");
    let soldier = reg.interner_mut().intern("Soldier");
    let _zombie = reg.interner_mut().intern("Zombie");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(soldier);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}{U}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Flash],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfDies,
                intervening_if: None,
                effect: dies_make_zombies,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn dies_make_zombies(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: effect — "for each nontoken creature you controlled that died this
    // turn, create a 2/2 black Zombie creature token" (no script helper counts
    // your nontoken creatures that died this turn; a fixed count would be wrong).
    Vec::new()
}
