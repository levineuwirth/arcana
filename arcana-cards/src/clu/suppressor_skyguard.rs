//! Suppressor Skyguard — `{2}{W}{U}` 2/4 Human Knight with Flying.
//! Whenever a player attacks you, if that player has another opponent who isn't
//! being attacked, prevent all combat damage that would be dealt to you this combat.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::effects::KeywordAbility;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Suppressor Skyguard");
    let human = reg.interner_mut().intern("Human");
    let knight = reg.interner_mut().intern("Knight");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(knight);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}{U}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            // GAP: trigger — "whenever a player attacks you" (a player declares an attack against
            // you) has no matching TriggerCondition variant; closest is CreatureAttacks but the
            // "attacks YOU" + the multiplayer intervening-if are not expressible.
            trigger_condition: TriggerCondition::CreatureAttacks {
                filter: arcana_core::targets::ObjectFilter::creature(),
            },
            // GAP: intervening-if "that player has another opponent who isn't being attacked"
            // is a multiplayer combat-topology predicate not in the conditions catalog.
            intervening_if: None,
            // GAP: "prevent all combat damage that would be dealt to you this combat" requires a
            // player-targeted combat-only damage-prevention replacement keyed to "you" — not
            // expressible with the available PreventDamageFrom/PreventDamage shapes here.
            effect: noop,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn noop(_state: &GameState, _trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    Vec::new()
}
