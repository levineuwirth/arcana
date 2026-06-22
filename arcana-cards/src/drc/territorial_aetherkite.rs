//! Territorial Aetherkite — `{4}{R}{R}` 6/5 Cat Dragon with Flying, Haste.
//! "When this creature enters, you get {E}{E} (two energy counters). Then you may
//!  pay one or more {E}. When you do, this creature deals that much damage to each
//!  other creature."
//!
//! GAP: paying {E} as a cost is not modeled (energy spend is not a cost field),
//! so the reflexive "pay one or more {E} → deal that much damage to each other
//! creature" rider is omitted. Only the {E}{E} energy gain half of the ETB is
//! implemented (plus Flying + Haste).

use arcana_core::effects::{Effect, KeywordAbility};
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
    let name = reg.interner_mut().intern("Territorial Aetherkite");
    let cat = reg.interner_mut().intern("Cat");
    let dragon = reg.interner_mut().intern("Dragon");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(cat);
    subtypes.0.insert(dragon);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(6)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![KeywordAbility::Flying, KeywordAbility::Haste],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: etb_gain_energy,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn etb_gain_energy(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: pay-{E}-then-deal-damage reflexive rider not expressible.
    vec![Effect::GainEnergy { player: trig.controller, amount: 2 }]
}
