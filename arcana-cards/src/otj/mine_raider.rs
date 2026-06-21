//! Mine Raider — `{2}{R}` 3/2 red Human Rogue with Trample.
//!
//! Oracle:
//! * Trample
//! * When this creature enters, if you control another outlaw, create a
//!   Treasure token. (Assassins, Mercenaries, Pirates, Rogues, and Warlocks
//!   are outlaws.)
//!
//! Intervening-if: since Mine Raider is itself an outlaw (Rogue), "another
//! outlaw" is modeled as controlling at least two outlaws.

use arcana_core::conditions;
use arcana_core::effects::{CommodityToken, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, ObjectId};
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PlayerId, PtValue, SubtypeSet, TypeLine};
use arcana_core::effects::KeywordAbility;
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Mine Raider");
    let human = reg.interner_mut().intern("Human");
    let rogue = reg.interner_mut().intern("Rogue");
    // Pre-intern the outlaw subtypes so the intervening-if can recover them.
    let _assassin = reg.interner_mut().intern("Assassin");
    let _mercenary = reg.interner_mut().intern("Mercenary");
    let _pirate = reg.interner_mut().intern("Pirate");
    let _warlock = reg.interner_mut().intern("Warlock");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(rogue);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Trample],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: Some(if_control_another_outlaw),
            effect: etb_make_treasure,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn outlaw_filter(reg: &CardRegistry) -> ObjectFilter {
    let mut syms = Vec::new();
    for name in ["Assassin", "Mercenary", "Pirate", "Rogue", "Warlock"] {
        if let Some(sym) = reg.interner().lookup(name) {
            syms.push(sym);
        }
    }
    ObjectFilter::creature().with_subtypes_any(syms)
}

fn if_control_another_outlaw(
    s: &GameState,
    _src: ObjectId,
    you: PlayerId,
    reg: &CardRegistry,
) -> bool {
    // Mine Raider counts as one outlaw; "another" → at least two.
    conditions::you_control_at_least(s, you, &outlaw_filter(reg), 2)
}

fn etb_make_treasure(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::CreateCommodityToken {
        controller: trig.controller,
        kind: CommodityToken::Treasure,
        count: 1,
    }]
}
