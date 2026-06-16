//! Edge Rover — `{G}` 2/2 Artifact Creature — Robot Scout with Reach.
//! "When this creature dies, each player creates a Lander token." The Lander's
//! printed activated ability (sacrifice to fetch a basic land) is not wired
//! (no such commodity token), but the bare token is minted for every player.

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Edge Rover");
    let robot = reg.interner_mut().intern("Robot");
    let scout = reg.interner_mut().intern("Scout");
    reg.interner_mut().intern("Lander");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(robot);
    subtypes.0.insert(scout);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Reach],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfDies,
            intervening_if: None,
            effect: each_player_makes_lander,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn each_player_makes_lander(state: &GameState, _trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect> {
    let lander = reg.interner().lookup("Lander").unwrap_or_default();
    let subtypes = {
        let mut s = SubtypeSet::default();
        s.0.insert(lander);
        s
    };
    script::all_players(state)
        .into_iter()
        .map(|p| Effect::CreateToken {
            controller: p,
            token: TokenDefinition {
                name: lander,
                colors: ColorSet::colorless(),
                types: TypeLine::ARTIFACT.into(),
                subtypes: subtypes.clone(),
                power: None,
                toughness: None,
                keywords: vec![],
                abilities: vec![],
            },
        })
        .collect()
}
