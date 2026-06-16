//! A-Cloister Gargoyle — `{1}{W}` 0/3 Artifact Creature — Gargoyle.
//! "When Cloister Gargoyle enters, venture into the dungeon.
//!  As long as you've completed a dungeon, Cloister Gargoyle gets +3/+0 and
//!  has flying."

use arcana_core::effects::Effect;
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
    let name = reg.interner_mut().intern("A-Cloister Gargoyle");
    let gargoyle = reg.interner_mut().intern("Gargoyle");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(gargoyle);

    // GAP: static "As long as you've completed a dungeon, ~ gets +3/+0 and has
    // flying" — a dungeon-completion-conditional continuous boost is not
    // expressible in this card class.
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: etb_venture,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn etb_venture(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::Venture {
        player: trig.controller,
    }]
}
