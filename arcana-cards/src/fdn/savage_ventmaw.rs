//! Savage Ventmaw — `{4}{R}{G}` 4/4 red-green Dragon with Flying.
//!
//! "Flying
//!  Whenever this creature attacks, add {R}{R}{R}{G}{G}{G}. Until end of turn,
//!  you don't lose this mana as steps and phases end."
//!
//! Flying is a base keyword. The attack trigger adds {R}{R}{R}{G}{G}{G} via
//! `Effect::AddMana`.
//!
//! FIDELITY GAP: the "until end of turn, you don't lose this mana as steps and
//! phases end" rider (mana that survives the empty-pool step) is not
//! expressible — the added mana empties normally at the end of the step/phase.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::{ManaCost, ManaUnit};
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, ManaColor, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Savage Ventmaw");
    let dragon = reg.interner_mut().intern("Dragon");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dragon);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{R}{G}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfAttacks,
            intervening_if: None,
            effect: add_rrr_ggg,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn add_rrr_ggg(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    let mut mana = vec![ManaUnit::plain(ManaColor::Red, trig.source); 3];
    mana.extend(vec![ManaUnit::plain(ManaColor::Green, trig.source); 3]);
    vec![Effect::AddMana {
        player: trig.controller,
        mana,
    }]
}
