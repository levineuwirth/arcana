//! Marching Duodrone — `{2}` colorless 2/2 Artifact Creature — Construct.
//! "Whenever this creature attacks, each player creates a Treasure token."
//! GAP: "each player creates" — no per-player loop in script; using script::all_players
//! to create one token per player via Effect::Sequence.

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
use arcana_core::script;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Marching Duodrone");
    let construct = reg.interner_mut().intern("Construct");
    let _treasure = reg.interner_mut().intern("Treasure");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(construct);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE).into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: each_player_treasure,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn each_player_treasure(
    state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let treasure = reg.interner().lookup("Treasure")
        .expect("Treasure interned during register()");
    let players = script::all_players(state);
    let effects: Vec<Effect> = players.into_iter().map(|p| {
        let mut subtypes = SubtypeSet::default();
        subtypes.0.insert(treasure);
        Effect::CreateToken {
            controller: p,
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
        }
    }).collect();
    vec![Effect::Sequence(effects)]
}
