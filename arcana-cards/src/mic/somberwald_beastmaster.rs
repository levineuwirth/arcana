//! Somberwald Beastmaster — `{6}{G}` 1/1 Human Ranger.
//! ETB: create a 2/2 Wolf, a 3/3 Beast, and a 4/4 Beast token.
//! Static: creature tokens you control have deathtouch (GAP — static).

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
    let name = reg.interner_mut().intern("Somberwald Beastmaster");
    let human = reg.interner_mut().intern("Human");
    let ranger = reg.interner_mut().intern("Ranger");
    // Pre-intern token subtypes.
    reg.interner_mut().intern("Wolf");
    reg.interner_mut().intern("Beast");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(ranger);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{6}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            // GAP: static "creature tokens you control have deathtouch" is a
            // continuous static buff, not a triggered/activated ability.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_make_tokens,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_make_tokens(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let wolf = reg.interner().lookup("Wolf").unwrap_or_default();
    let beast = reg.interner().lookup("Beast").unwrap_or_default();
    let mut wolf_subtypes = SubtypeSet::default();
    wolf_subtypes.0.insert(wolf);
    let mut beast_subtypes = SubtypeSet::default();
    beast_subtypes.0.insert(beast);

    vec![
        Effect::CreateToken {
            controller: trig.controller,
            token: TokenDefinition {
                name: wolf,
                colors: ColorSet::green(),
                types: TypeLine::CREATURE.into(),
                subtypes: wolf_subtypes.clone(),
                power: Some(PtValue::Fixed(2)),
                toughness: Some(PtValue::Fixed(2)),
                keywords: vec![],
                abilities: vec![],
            },
        },
        Effect::CreateToken {
            controller: trig.controller,
            token: TokenDefinition {
                name: beast,
                colors: ColorSet::green(),
                types: TypeLine::CREATURE.into(),
                subtypes: beast_subtypes.clone(),
                power: Some(PtValue::Fixed(3)),
                toughness: Some(PtValue::Fixed(3)),
                keywords: vec![],
                abilities: vec![],
            },
        },
        Effect::CreateToken {
            controller: trig.controller,
            token: TokenDefinition {
                name: beast,
                colors: ColorSet::green(),
                types: TypeLine::CREATURE.into(),
                subtypes: beast_subtypes,
                power: Some(PtValue::Fixed(4)),
                toughness: Some(PtValue::Fixed(4)),
                keywords: vec![],
                abilities: vec![],
            },
        },
    ]
}
