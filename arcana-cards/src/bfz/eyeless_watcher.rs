//! Eyeless Watcher — `{3}{G}` 1/1 Eldrazi Drone with Devoid (colorless).
//! "When this creature enters, create two 1/1 colorless Eldrazi Scion creature
//!  tokens. They have 'Sacrifice this token: Add {C}.'"

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Eyeless Watcher");
    let eldrazi = reg.interner_mut().intern("Eldrazi");
    let drone = reg.interner_mut().intern("Drone");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(eldrazi);
    subtypes.0.insert(drone);

    // Pre-intern the token subtype so the resolver can rebuild it.
    let _scion = reg.interner_mut().intern("Scion");

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}").expect("valid cost")),
        colors: ColorSet::colorless(), // Devoid — this card has no color.
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_scions,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_scions(_state: &GameState, trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect> {
    let scion = reg.interner().lookup("Scion").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(scion);

    // GAP: the token's activated ability "Sacrifice this token: Add {C}" cannot be
    // attached — TokenDefinition.abilities only holds triggered abilities, not
    // activated ones. The bare 1/1 colorless Eldrazi Scion tokens are minted.
    let make_token = || Effect::CreateToken {
        controller: trig.controller,
        token: TokenDefinition {
            name: scion,
            colors: ColorSet::colorless(),
            types: TypeLine::CREATURE.into(),
            subtypes: subtypes.clone(),
            power: Some(PtValue::Fixed(1)),
            toughness: Some(PtValue::Fixed(1)),
            keywords: vec![],
            abilities: vec![],
        },
    };
    vec![make_token(), make_token()]
}
