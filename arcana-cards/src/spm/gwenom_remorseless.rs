//! Gwenom, Remorseless — `{3}{B}{B}` 4/4 Legendary Symbiote Spider Hero
//! with Deathtouch and Lifelink.
//!
//! Oracle:
//! * Deathtouch, lifelink
//! * Whenever Gwenom attacks, until end of turn, you may look at the top
//!   card of your library any time and you may play cards from the top
//!   of your library. If you cast a spell this way, pay life equal to
//!   its mana value rather than pay its mana cost. (GAP — play-from-top
//!   permission with an alternate life cost is not an expressible
//!   Effect. The attack trigger is wired; its body is GAP'd.)

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
    let name = reg.interner_mut().intern("Gwenom, Remorseless");
    let symbiote = reg.interner_mut().intern("Symbiote");
    let spider = reg.interner_mut().intern("Spider");
    let hero = reg.interner_mut().intern("Hero");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(symbiote);
    subtypes.0.insert(spider);
    subtypes.0.insert(hero);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Deathtouch, KeywordAbility::Lifelink],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfAttacks,
            intervening_if: None,
            effect: play_from_top,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn play_from_top(_state: &GameState, _trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "until end of turn, you may play cards from the top of your
    // library; cast a spell this way by paying life equal to its mana
    // value" — play-from-top with an alternate life cost is not
    // expressible.
    Vec::new()
}
