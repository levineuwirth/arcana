//! Kozilek's Predator — `{3}{G}` 3/3 Creature — Eldrazi Drone.
//! "When this creature enters, create two 0/1 colorless Eldrazi Spawn creature tokens. They have "Sacrifice this token: Add {C}.""

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Kozilek's Predator");
    let eldrazi_sub = reg.interner_mut().intern("Eldrazi");
    let drone_sub = reg.interner_mut().intern("Drone");
    let _spawn = reg.interner_mut().intern("Spawn");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(eldrazi_sub);
    subtypes.0.insert(drone_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}").expect("valid cost")),
        colors: ColorSet::green(),
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
                effect: kozilek_s_predator_trigger,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn kozilek_s_predator_trigger(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let spawn_tok = reg.interner().lookup("Spawn").expect("Spawn interned during register()");
    let mut subtypes_0 = SubtypeSet::default();
    subtypes_0.0.insert(spawn_tok);
    let mut subtypes_1 = SubtypeSet::default();
    subtypes_1.0.insert(spawn_tok);
    vec![
        Effect::CreateToken { controller: trig.controller, token: TokenDefinition { name: spawn_tok, colors: ColorSet::colorless(), types: TypeLine::CREATURE.into(), subtypes: subtypes_0, power: Some(PtValue::Fixed(0)), toughness: Some(PtValue::Fixed(1)), keywords: vec![], abilities: vec![] } },
        Effect::CreateToken { controller: trig.controller, token: TokenDefinition { name: spawn_tok, colors: ColorSet::colorless(), types: TypeLine::CREATURE.into(), subtypes: subtypes_1, power: Some(PtValue::Fixed(0)), toughness: Some(PtValue::Fixed(1)), keywords: vec![], abilities: vec![] } },
    ]
}
