//! Gerrard, Weatherlight Hero — `{2}{R}{W}` 3/3 Legendary Human Soldier.
//!
//! First strike.
//! When Gerrard dies, exile it and return to the battlefield all artifact and
//! creature cards in your graveyard that were put there from the battlefield
//! this turn.
//!
//! The dies trigger fires, but its effect is GAP'd: there is no batch
//! "return all matching cards from your graveyard" primitive, and the
//! "put there from the battlefield this turn" restriction is unexpressible.

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
    let name = reg.interner_mut().intern("Gerrard, Weatherlight Hero");
    let human = reg.interner_mut().intern("Human");
    let soldier = reg.interner_mut().intern("Soldier");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(soldier);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}{W}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::new().with(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::FirstStrike],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfDies,
            intervening_if: None,
            effect: dies_recur,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn dies_recur(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: no batch return-from-graveyard primitive, and the "put there from
    // the battlefield this turn" / self-exile restriction is unexpressible.
    Vec::new()
}
