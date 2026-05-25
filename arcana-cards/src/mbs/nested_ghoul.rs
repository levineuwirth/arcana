//! Nested Ghoul — `{3}{B}{B}` 4/2 black Phyrexian Zombie Warrior.
//! "Whenever a source deals damage to Nested Ghoul, create a 2/2 black
//! Phyrexian Zombie creature token."
//! GAP: "damage dealt to this creature" trigger not in TriggerCondition catalog;
//! using SelfEntersBattlefield as structural placeholder.

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
    let name = reg.interner_mut().intern("Nested Ghoul");
    let phyrexian = reg.interner_mut().intern("Phyrexian");
    let zombie = reg.interner_mut().intern("Zombie");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(phyrexian);
    subtypes.0.insert(zombie);
    subtypes.0.insert(warrior);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                // GAP: "damage dealt to this creature" trigger not in TriggerCondition catalog;
                // using SelfEntersBattlefield as structural placeholder
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: damage_received_effect,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn damage_received_effect(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let zombie_id = reg.interner().lookup("Zombie").expect("Zombie interned during register()");
    let phyrexian_id = reg.interner().lookup("Phyrexian").expect("Phyrexian interned during register()");
    let mut token_subtypes = SubtypeSet::default();
    token_subtypes.0.insert(phyrexian_id);
    token_subtypes.0.insert(zombie_id);
    let token = TokenDefinition {
        name: zombie_id,
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes: token_subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![],
        abilities: vec![],
    };
    vec![Effect::CreateToken { controller: trig.controller, token }]
}
