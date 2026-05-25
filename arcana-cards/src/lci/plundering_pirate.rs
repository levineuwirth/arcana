//! Plundering Pirate — `{2}{R}` 3/2 Creature — Orc Pirate.
//! "When this creature enters, create a Treasure token. (It's an artifact with "{T}, Sacrifice this token: Add one mana of any color.")"

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Plundering Pirate");
    let orc_sub = reg.interner_mut().intern("Orc");
    let pirate_sub = reg.interner_mut().intern("Pirate");
    let _treasure = reg.interner_mut().intern("Treasure");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(orc_sub);
    subtypes.0.insert(pirate_sub);

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
                effect: plundering_pirate_trigger,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn plundering_pirate_trigger(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let treasure = reg.interner().lookup("Treasure").expect("Treasure interned during register()");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(treasure);
    vec![Effect::CreateToken {
        controller: trig.controller,
        token: TokenDefinition {
            name: treasure,
            colors: ColorSet::colorless(),
            types: TypeLine::ARTIFACT.into(),
            subtypes,
            power: None,
            toughness: None,
            keywords: vec![],
            abilities: vec![],
        },
    }]
}
