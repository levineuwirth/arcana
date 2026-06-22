//! Jurin, Leading the Charge — `{4}{R}{R}` 4/6 Legendary Human Barbarian (red).
//! Haste.
//! Jurin must be blocked if able.
//! Whenever Jurin attacks, each creature you control attacking a player gets
//! +1/+0 until end of turn for each creature that player controls.
//!
//! Haste is a base keyword. The "must be blocked if able" static and the
//! complex per-attacker, per-defender-creature-count pump cannot be expressed
//! with the available primitives — both GAP'd.

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
    let name = reg.interner_mut().intern("Jurin, Leading the Charge");
    let human = reg.interner_mut().intern("Human");
    let barbarian = reg.interner_mut().intern("Barbarian");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(barbarian);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(6)),
        keywords: vec![KeywordAbility::Haste],
        ..Default::default()
    };

    // GAP: static "Jurin must be blocked if able." (no must-be-blocked primitive).
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfAttacks,
            intervening_if: None,
            effect: on_attack,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn on_attack(_state: &GameState, _trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "each creature you control attacking a player gets +1/+0 until end of
    // turn for each creature that player controls" — requires per-attacker pumps
    // whose magnitude depends on the specific defending player each one is
    // attacking, which the available effect/scripting primitives cannot express.
    Vec::new()
}
