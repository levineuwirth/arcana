//! Zurgo's Vanguard — `{2}{R}` */3 red Dog Soldier.
//!
//! * Mobilize 1 — not in the usable keyword surface, so the keyword
//!   itself is GAP'd; its reminder text spells out a triggered ability
//!   ("Whenever this creature attacks, create a tapped and attacking
//!   1/1 red Warrior creature token. Sacrifice it at the beginning of
//!   the next end step."), which we wire as a `SelfAttacks` trigger
//!   minting a 1/1 red Warrior via `CreateTokenSacEot` (create + sac at
//!   the next end step). FIDELITY GAP: the documented token catalog has
//!   no "tapped and attacking" creation, so the token enters untapped
//!   and not attacking.
//! * "Zurgo's Vanguard's power is equal to the number of creatures you
//!   control." — characteristic-defining power; no CDA P/T primitive is
//!   in scope, so base power is recorded as 0 and the dynamic value is
//!   GAP'd.

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
    let name = reg.interner_mut().intern("Zurgo's Vanguard");
    let dog = reg.interner_mut().intern("Dog");
    let soldier = reg.interner_mut().intern("Soldier");
    let _warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dog);
    subtypes.0.insert(soldier);

    // GAP: Mobilize 1 keyword is not in the usable keyword surface;
    // its triggered-ability reminder text is wired below.
    // GAP: characteristic-defining power "equal to the number of
    // creatures you control" — base power recorded as 0.

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfAttacks,
            intervening_if: None,
            effect: mobilize_token,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn mobilize_token(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let warrior = reg.interner().lookup("Warrior").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(warrior);
    vec![Effect::CreateTokenSacEot {
        controller: trig.controller,
        token: TokenDefinition {
            name: warrior,
            colors: ColorSet::red(),
            types: TypeLine::CREATURE.into(),
            subtypes,
            power: Some(PtValue::Fixed(1)),
            toughness: Some(PtValue::Fixed(1)),
            keywords: vec![],
            abilities: vec![],
        },
    }]
}
