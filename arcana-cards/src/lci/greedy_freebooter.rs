//! Greedy Freebooter — `{B}` 1/1 Creature — Human Pirate.
//! "When this creature dies, scry 1 and create a Treasure token. (To scry 1, look at the top card of your library. You may put that card on the bottom. A Treasure token is an artifact with "{T}, Sacrifice this token: Add one mana of any color.")"

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Greedy Freebooter");
    let human_sub = reg.interner_mut().intern("Human");
    let pirate_sub = reg.interner_mut().intern("Pirate");
    let _treasure = reg.interner_mut().intern("Treasure");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human_sub);
    subtypes.0.insert(pirate_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfDies,
                intervening_if: None,
                effect: greedy_freebooter_trigger,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn greedy_freebooter_trigger(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let treasure = reg.interner().lookup("Treasure").expect("Treasure interned during register()");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(treasure);
    vec![
        Effect::Scry { player: trig.controller, count: 1 },
        Effect::CreateToken {
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
        },
    ]
}
