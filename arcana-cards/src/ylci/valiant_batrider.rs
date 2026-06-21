//! Valiant Batrider — `{2}{W}` 3/3 Human Knight with Flying.
//! "Whenever Valiant Batrider deals combat damage to a player, that
//! player gets a one-time boon with 'When you cast a noncreature spell,
//! you may pay {1}. If you don't, each opponent draws a card.'"
//!
//! Flying is a base keyword. The combat-damage trigger grants a
//! delayed/boon ability to the damaged player — the engine has no
//! "one-time boon" / grant-a-triggered-ability-to-a-PLAYER primitive
//! (GrantTriggeredAbility targets a permanent, not a player), so the
//! boon payload is GAP'd; the bones + keyword + trigger shape are kept.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::TargetFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::effects::KeywordAbility;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Valiant Batrider");
    let human = reg.interner_mut().intern("Human");
    let knight = reg.interner_mut().intern("Knight");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(knight);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::DamageDealt {
                source_filter: arcana_core::targets::ObjectFilter::default(),
                target_filter: TargetFilter::Player,
                combat_only: true,
            },
            intervening_if: None,
            effect: grant_boon,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn grant_boon(_state: &GameState, _trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "that player gets a one-time boon with '<triggered ability>'" — the
    // engine grants triggered abilities to PERMANENTS (GrantTriggeredAbility
    // takes an object target), not to players; a one-time player boon is not
    // expressible with the demonstrated API.
    Vec::new()
}
