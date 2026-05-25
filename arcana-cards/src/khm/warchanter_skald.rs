//! Warchanter Skald — `{2}{W}` 2/3 white creature. "Whenever this creature
//! becomes tapped, if it's enchanted or equipped, create a 2/1 red Dwarf
//! Berserker creature token."
//!
//! GAP: intervening_if — "if enchanted or equipped" check not expressible.
//! Emitting token unconditionally as best-effort.

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
    let name = reg.interner_mut().intern("Warchanter Skald");
    let dwarf = reg.interner_mut().intern("Dwarf");
    let cleric = reg.interner_mut().intern("Cleric");
    let _berserker = reg.interner_mut().intern("Berserker");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dwarf);
    subtypes.0.insert(cleric);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfBecomesTapped,
                intervening_if: None, // GAP: "if enchanted or equipped" check
                effect: tapped_create_berserker,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn tapped_create_berserker(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let dwarf = reg.interner().lookup("Dwarf").expect("Dwarf interned");
    let berserker = reg.interner().lookup("Berserker").expect("Berserker interned");
    let mut st = SubtypeSet::default();
    st.0.insert(dwarf);
    st.0.insert(berserker);
    vec![Effect::CreateToken {
        controller: trig.controller,
        token: TokenDefinition {
            name: dwarf,
            colors: ColorSet::red(),
            types: TypeLine::CREATURE.into(),
            subtypes: st,
            power: Some(PtValue::Fixed(2)),
            toughness: Some(PtValue::Fixed(1)),
            keywords: vec![],
            abilities: vec![],
        },
    }]
}
