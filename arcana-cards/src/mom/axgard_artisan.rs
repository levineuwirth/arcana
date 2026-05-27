//! Axgard Artisan — `{1}{R}` 2/1 Dwarf Artificer. "Whenever one or more
//! +1/+1 counters are put on this creature for the first time each turn,
//! create a Treasure token."

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
    TriggerSelf,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Axgard Artisan");
    let dwarf = reg.interner_mut().intern("Dwarf");
    let artificer = reg.interner_mut().intern("Artificer");
    let _treasure = reg.interner_mut().intern("Treasure");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dwarf);
    subtypes.0.insert(artificer);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![],
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::CounterAdded {
                    on: TriggerSelf::Source,
                    kind: Some(CounterKind::PlusOnePlusOne),
                    chapter: None,
                },
                intervening_if: None,
                effect: on_counter_added_create_treasure,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::OncePerTurn,
                target_requirements: Vec::new(),
            }),
    )
}

fn on_counter_added_create_treasure(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let treasure = reg.interner().lookup("Treasure")
        .expect("Treasure interned during register()");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(treasure);
    let token = TokenDefinition {
        name: treasure,
        colors: ColorSet::colorless(),
        types: TypeLine::ARTIFACT.into(),
        subtypes,
        power: None,
        toughness: None,
        keywords: vec![],
        abilities: vec![],
    };
    vec![Effect::CreateToken { controller: trig.controller, token }]
}
