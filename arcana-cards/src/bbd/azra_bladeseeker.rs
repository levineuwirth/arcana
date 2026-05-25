//! Azra Bladeseeker — `{2}{R}` 3/2 red Azra Warrior. "When this creature
//! enters, each player on your team may discard a card, then each player
//! who discarded a card this way draws a card."
//! GAP: effect — "players on your team" (multiplayer team semantics) and
//! conditional "if they discarded" chaining are not in the catalog;
//! treating as controller discards then draws.

use arcana_core::effects::Effect;
use arcana_core::effects::DiscardChoice;
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
    let name = reg.interner_mut().intern("Azra Bladeseeker");
    let azra = reg.interner_mut().intern("Azra");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(azra);
    subtypes.0.insert(warrior);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: loot,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn loot(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "each player on your team" semantics not in catalog; using controller only
    vec![
        Effect::Discard { player: trig.controller, count: 1, choice: DiscardChoice::ControllerChooses },
        Effect::DrawCards { player: trig.controller, count: 1 },
    ]
}
