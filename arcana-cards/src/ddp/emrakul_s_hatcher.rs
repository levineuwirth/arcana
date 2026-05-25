//! Emrakul's Hatcher — `{4}{R}` 3/3 red Eldrazi Drone. "When this creature
//! enters, create three 0/1 colorless Eldrazi Spawn creature tokens. They
//! have 'Sacrifice this token: Add {C}.'"
//!
//! Token note: Eldrazi Spawn sacrifice-for-mana activation is recognized
//! by the engine via subtype but the activation itself is deferred engine
//! work — emitting the token cleanly without activation in `abilities`.

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
    let name = reg.interner_mut().intern("Emrakul's Hatcher");
    let eldrazi = reg.interner_mut().intern("Eldrazi");
    let drone = reg.interner_mut().intern("Drone");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(eldrazi);
    subtypes.0.insert(drone);

    // Pre-intern token subtypes for resolve-time lookup
    let _spawn_eldrazi = reg.interner_mut().intern("Eldrazi");
    let _spawn_spawn = reg.interner_mut().intern("Spawn");

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: create_spawn_tokens,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn create_spawn_tokens(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let eldrazi = reg.interner().lookup("Eldrazi")
        .expect("Eldrazi interned during register()");
    let spawn = reg.interner().lookup("Spawn")
        .expect("Spawn interned during register()");
    let make_token = || {
        let mut token_subtypes = SubtypeSet::default();
        token_subtypes.0.insert(eldrazi);
        token_subtypes.0.insert(spawn);
        Effect::CreateToken {
            controller: trig.controller,
            token: TokenDefinition {
                name: spawn,
                colors: ColorSet::colorless(),
                types: TypeLine::CREATURE.into(),
                subtypes: token_subtypes,
                power: Some(PtValue::Fixed(0)),
                toughness: Some(PtValue::Fixed(1)),
                keywords: vec![],
                abilities: vec![],
            },
        }
    };
    vec![make_token(), make_token(), make_token()]
}
