//! Contagious Vorrac — `{2}{G}` 3/3 green Phyrexian Boar Beast.
//! "When this creature enters, look at the top four cards of your library.
//! You may reveal a land card from among them and put it into your hand.
//! Put the rest on the bottom of your library in a random order. If you
//! didn't put a card into your hand this way, proliferate."
//!
//! GAP: "look at top 4, optionally reveal land to hand, rest to bottom" —
//! not expressible. Emitting Proliferate only (best-effort for the fallback
//! branch; the look+reveal is a GAP).

use arcana_core::effects::Effect;
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
    let name = reg.interner_mut().intern("Contagious Vorrac");
    let phyrexian = reg.interner_mut().intern("Phyrexian");
    let boar = reg.interner_mut().intern("Boar");
    let beast = reg.interner_mut().intern("Beast");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(phyrexian);
    subtypes.0.insert(boar);
    subtypes.0.insert(beast);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: etb_look_or_proliferate,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn etb_look_or_proliferate(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "look at top 4, reveal land to hand, rest to bottom random order"
    // not expressible. Emitting Proliferate for the fallback branch only.
    vec![Effect::Proliferate]
}
