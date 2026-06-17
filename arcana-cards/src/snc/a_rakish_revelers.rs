//! A-Rakish Revelers — `{2}{R}{G}{W}` 5/3 Elf Druid Rogue.
//! "When Rakish Revelers enters, create a 1/1 green and white Citizen creature
//! token. {1}, Exile Rakish Revelers from your hand: Target land gains '{T}:
//! Add {R}, {G}, or {W}' until Rakish Revelers is cast from exile. You may cast
//! Rakish Revelers for as long as it remains exiled."

use arcana_core::effects::{Effect, TokenDefinition};
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
    let name = reg.interner_mut().intern("A-Rakish Revelers");
    let elf = reg.interner_mut().intern("Elf");
    let druid = reg.interner_mut().intern("Druid");
    let rogue = reg.interner_mut().intern("Rogue");
    let _citizen = reg.interner_mut().intern("Citizen");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elf);
    subtypes.0.insert(druid);
    subtypes.0.insert(rogue);
    // GAP: the hand-exile activated ability "{1}, Exile Rakish Revelers from
    // your hand: Target land gains '{T}: Add {R}, {G}, or {W}' until ... ; you
    // may cast it for as long as it remains exiled" — granting an arbitrary
    // ACTIVATED (mana) ability to a target, plus the cast-from-exile
    // permission, is not expressible in this shape.
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}{G}{W}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::red() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: etb_citizen,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn etb_citizen(_state: &GameState, trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect> {
    let citizen = reg.interner().lookup("Citizen").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(citizen);
    vec![Effect::CreateToken {
        controller: trig.controller,
        token: TokenDefinition {
            name: citizen,
            colors: ColorSet::green() | ColorSet::white(),
            types: TypeLine::CREATURE.into(),
            subtypes,
            power: Some(PtValue::Fixed(1)),
            toughness: Some(PtValue::Fixed(1)),
            keywords: vec![],
            abilities: vec![],
        },
    }]
}
