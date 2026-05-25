//! Pawn of Ulamog — `{1}{B}{B}` 2/2 Vampire Shaman. "Whenever this creature
//! or another nontoken creature you control dies, you may create a 0/1
//! colorless Eldrazi Spawn creature token. It has 'Sacrifice this token:
//! Add {C}.'"
//!
//! GAP: token ability "Sacrifice this token: Add {C}" cannot be expressed
//! in the TokenDefinition abilities list; emitting token without the mana
//! ability. Also, "nontoken" filter for the trigger condition is not
//! expressible in ZoneChange; using SelfDies only (misses allied creatures).

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
    let name = reg.interner_mut().intern("Pawn of Ulamog");
    let _eldrazi = reg.interner_mut().intern("Eldrazi");
    let _spawn = reg.interner_mut().intern("Spawn");
    let vampire = reg.interner_mut().intern("Vampire");
    let shaman = reg.interner_mut().intern("Shaman");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(vampire);
    subtypes.0.insert(shaman);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
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
                trigger_condition: TriggerCondition::SelfDies,
                intervening_if: None,
                effect: on_dies_spawn,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn on_dies_spawn(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let eldrazi = reg.interner().lookup("Eldrazi")
        .expect("Eldrazi interned during register()");
    let spawn = reg.interner().lookup("Spawn")
        .expect("Spawn interned during register()");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(eldrazi);
    subtypes.0.insert(spawn);
    // GAP: token ability "Sacrifice this token: Add {C}" not expressible in
    // the TokenDefinition abilities list.
    let token = TokenDefinition {
        name: spawn,
        colors: ColorSet::default(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![],
        abilities: vec![],
    };
    vec![Effect::CreateToken { controller: trig.controller, token }]
}
