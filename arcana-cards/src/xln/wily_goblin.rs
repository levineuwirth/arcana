//! Wily Goblin — `{R}{R}` 1/1 Goblin Pirate. "When this creature enters,
//! create a Treasure token."
//!
//! Keywords: Treasure noted.
//! GAP: no Effect for creating a Treasure token (special artifact token with
//! mana ability); emitting best-effort via CreateToken without the mana
//! ability.

use arcana_core::effects::{Effect, TokenDefinition};
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
    let name = reg.interner_mut().intern("Wily Goblin");
    let _treasure = reg.interner_mut().intern("Treasure");
    let goblin = reg.interner_mut().intern("Goblin");
    let pirate = reg.interner_mut().intern("Pirate");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(goblin);
    subtypes.0.insert(pirate);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![],
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: on_etb_treasure,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn on_etb_treasure(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let treasure = reg.interner().lookup("Treasure")
        .expect("Treasure interned during register()");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(treasure);
    // GAP: Treasure token has "Sacrifice: Add one mana of any color" which
    // is not expressible in TokenDefinition abilities list.
    let token = TokenDefinition {
        name: treasure,
        colors: ColorSet::default(),
        types: TypeLine::ARTIFACT.into(),
        subtypes,
        power: None,
        toughness: None,
        keywords: vec![],
        abilities: vec![],
    };
    vec![Effect::CreateToken { controller: trig.controller, token }]
}
